// External Libraries
use actix_web::{web, Responder, HttpResponse, HttpRequest};
use sqlx::{SqlitePool};

// Internal Mappers
use crate::event::mapper::{
    fetch_events,
    fetch_event,
    create_event,
    update_event,
    fetch_monthly_ticket_sales,
    fetch_daily_event_counts
};
use crate::organizer::mapper::fetch_organizer;
use crate::agenda::mapper::{fetch_agenda, create_agenda, update_agenda};
use crate::speaker::mapper::{fetch_speakers, create_speakers, update_speakers};
use crate::faq::mapper::{fetch_faqs, create_faqs, update_faqs};
use crate::attachment::mapper::{fetch_attachments, create_attachments, update_attachments};
use crate::comment::mapper::fetch_comments;

// Internal Models
use crate::event::models::{
    Event,
    EventData,
    GetUserEventsQuery,
    GetUserEventsData,
    GetEventData,
    EventDetails,
    CreateEventDetails,
    TicketTotals,
    EventCounts
};
use crate::organizer::models::{Organizer, GetOrganizerData};
use crate::overview::models::{GetOverview, YearQuery};
use crate::agenda::models::GetAgendaData;
use crate::speaker::models::GetSpeakerData;
use crate::faq::models::GetFaqData;
use crate::attachment::models::GetAttachmentData;
use crate::comment::models::GetCommentData;

// Internal Services
use crate::auth::services::validate_session;


/// Retrieves aggregated ticket sales data including monthly ticket counts and revenue
/// for a specific organizer and year.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `query` - A query parameter containing the year to retrieve ticket data for.
/// * `pool` - A reference to the SQLite database connection pool.
///
/// # Returns
///
/// A JSON response containing ticket sales data or an error message if the operation fails.
pub async fn get_monthly_ticket_sales(
    req: HttpRequest,
    query: web::Query<YearQuery>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };

    let year = query.year;
    let organizer_id = session.user_id;

    match fetch_monthly_ticket_sales(GetOverview {organizer_id, year}, &pool).await {
        Ok(totals) => HttpResponse::Ok().json(TicketTotals {..totals}),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to fetch ticket sales data: {}", e)),
    }
}


/// Retrieves daily event counts for a specific organizer and year.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `query` - A query parameter containing the year to retrieve data for.
/// * `pool` - A reference to the SQLite database connection pool.
///
/// # Returns
///
/// A JSON response containing a list of daily events counts with dates or an error message if the operation fails.
pub async fn get_daily_event_counts(
    req: HttpRequest,
    query: web::Query<YearQuery>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };

    let year = query.year;
    let organizer_id = session.user_id;

    match fetch_daily_event_counts(GetOverview {organizer_id, year}, &pool).await {
        Ok(totals) => HttpResponse::Ok().json(EventCounts {..totals}),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to fetch daily event counts: {}", e)),
    }
}


/// Handles retrieving all events associated with the authenticated organizer.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `pool` - The SQLite database connection pool.
///
/// # Returns
///
/// An HTTP response with event data if successful, or an error message.
pub async fn get_events(
    req: HttpRequest,
    query: web::Query<GetUserEventsQuery>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };
    
    match fetch_events(GetUserEventsData {organizer_id: session.user_id, year: query.year}, &pool).await {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(e) => HttpResponse::InternalServerError().body(format!("Events not found: {}", e)),
    }
}


/// Handles retrieving a specific event by ID, ensuring the organizer owns it.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `event_id` - The path parameter representing the event's ID.
/// * `pool` - The SQLite database connection pool.
///
/// # Returns
///
/// An HTTP response with the event information if found, or an error message.
pub async fn get_event(
    req: HttpRequest,
    event_id: web::Path<i64>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };

    match fetch_event(GetEventData {event_id: *event_id, organizer_id: session.user_id}, &pool).await {
        Ok(event) => HttpResponse::Ok().json(Event {..event}),
        Err(e) => HttpResponse::InternalServerError().body(format!("Event not found: {}", e)),
    }
}


/// Handles retrieving a specific event's details by ID, ensuring the organizer owns it.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `event_id` - The path parameter representing the event's ID.
/// * `pool` - The SQLite database connection pool.
///
/// # Returns
///
/// An HTTP response with the event detail information if found, or an error message.
pub async fn get_event_details(
    req: HttpRequest,
    event_id: web::Path<i64>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };

    let event = match fetch_event(GetEventData {event_id: *event_id, organizer_id: session.user_id}, &pool).await {
        Ok(event) => event,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Event not found: {}", e)),
    };

    let organizer_info = fetch_organizer(GetOrganizerData { organizer_id: event.organizer_id }, &pool)
        .await.unwrap_or_else(|_| Organizer::default());
    let agenda_items = fetch_agenda(GetAgendaData { event_id: event.id }, &pool)
        .await.unwrap_or_else(|_| vec![]);
    let speaker_items = fetch_speakers(GetSpeakerData { event_id: event.id }, &pool)
        .await.unwrap_or_else(|_| vec![]);
    let faq_items = fetch_faqs(GetFaqData { event_id: event.id }, &pool)
        .await.unwrap_or_else(|_| vec![]);
    let attachment_items = fetch_attachments(GetAttachmentData { event_id: event.id }, &pool)
        .await.unwrap_or_else(|_| vec![]);
    let comment_items = fetch_comments(GetCommentData { event_id: event.id }, &pool)
        .await.unwrap_or_else(|_| vec![]);
    
    // TODO Fetch related events based on similar data: category_id, speakers, etc

    HttpResponse::Ok().json(EventDetails {
        organizer: organizer_info,
        agenda: agenda_items,
        speakers: speaker_items,
        faqs: faq_items,
        attachments: attachment_items,
        comments: comment_items,
        related_events: vec![],
    })
}


/// Handles registering a new event under the authenticated organizer.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `data` - The JSON body containing new event data.
/// * `pool` - The SQLite database connection pool.
///
/// # Returns
///
/// An HTTP response indicating success or failure of event creation.
pub async fn register_event(
    req: HttpRequest,
    data: web::Json<EventData>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };
    
    // TODO Save image file and update image to be location reference

    match create_event(EventData {organizer_id: session.user_id, ..data.into_inner()}, &pool).await {
        Ok(event) => HttpResponse::Ok().body(format!("Event '{}' registered", event.title)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to register event: {}", e)),
    }
}


/// Handles registering a new events details under the authenticated organizer.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `event_id` - The path parameter representing the event's ID.
/// * `data` - The JSON body containing new event detail data.
/// * `pool` - The SQLite database connection pool.
///
/// # Returns
///
/// An HTTP response indicating success or failure of event detail creation.
pub async fn register_event_details(
    req: HttpRequest,
    event_id: web::Path<i64>,
    data: web::Json<CreateEventDetails>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };
    
    match fetch_event(GetEventData {event_id: *event_id, organizer_id: session.user_id}, &pool).await {
        Ok(event) => event,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Event not found: {}", e)),
    };
    
    let CreateEventDetails { 
        agenda, 
        speakers, 
        faqs, 
        attachments, 
    } = data.into_inner();
    
    let agenda_items = match create_agenda(agenda, &pool).await {
        Ok(agenda_items) => agenda_items,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to create agenda: {}", e)),
    };
    let speaker_items = match create_speakers(speakers, &pool).await {
        Ok(speaker_items) => speaker_items,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to create speakers: {}", e)),
    };
    let faq_items = match create_faqs(faqs, &pool).await {
        Ok(faq_items) => faq_items,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to create faqs: {}", e)),
    };
    let attachment_items = match create_attachments(attachments, &pool).await {
        Ok(attachment_items) => attachment_items,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to create attachments: {}", e)),
    };

    HttpResponse::Ok().json(CreateEventDetails {
        agenda: agenda_items, 
        speakers: speaker_items,
        faqs: faq_items,
        attachments: attachment_items,
    })
}


/// Handles updating an event under the authenticated organizer.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `event_id` - The path parameter representing the event's ID.
/// * `data` - The JSON body containing new event data.
/// * `pool` - The SQLite database connection pool.
///
/// # Returns
///
/// An HTTP response indicating success or failure of updating event.
pub async fn put_event(
    req: HttpRequest,
    event_id: web::Path<i64>,
    data: web::Json<Event>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };

    let event = match fetch_event(GetEventData {event_id: *event_id, organizer_id: session.user_id}, &pool).await {
        Ok(event) => event,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Event not found: {}", e)),
    };
    
    // TODO Remove old and save new image file and update image location reference
    if data.image != event.image {
        
    }

    match update_event(Event {id: *event_id, ..data.into_inner()}, &pool).await {
        Ok(()) => HttpResponse::Ok().body(format!("Event '{}' updated", event_id)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to update event: {}", e)),
    }
}


/// Handles updating the detailed information of a specific event.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `event_id` - The path parameter representing the event's ID to update.
/// * `data` - The JSON body containing new event detail data.
/// * `pool` - The SQLite database connection pool.
///
/// # Returns
///
/// An HTTP response indicating success or failure of the update operation.
pub async fn put_event_details(
    req: HttpRequest,
    event_id: web::Path<i64>,
    data: web::Json<EventDetails>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };

    match fetch_event(GetEventData {event_id: *event_id, organizer_id: session.user_id}, &pool).await {
        Ok(event) => event,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Event not found: {}", e)),
    };

    let EventDetails { 
        agenda, 
        speakers, 
        faqs, 
        attachments, 
        .. 
    } = data.into_inner();

    match update_agenda(agenda, &pool).await {
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to update agenda: {}", e)),
        _ => {},
    };
    match update_speakers(speakers, &pool).await {
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to update speakers: {}", e)),
        _ => {},
    };
    match update_faqs(faqs, &pool).await {
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to update faqs: {}", e)),
        _ => {},
    };
    match update_attachments(attachments, &pool).await {
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to update attachments: {}", e)),
        _ => {},
    };
    
    HttpResponse::Ok().body("Event details updated")
}


/// Configures all routes related to event management.
///
/// # Arguments
///
/// * `cfg` - A mutable reference to the Actix service configuration.
///
/// # Returns
///
/// Adds all event-related routes to the Actix web application.
pub fn configure_event_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/events/sales/", web::get().to(get_monthly_ticket_sales))
        .route("/events/counts/daily/", web::get().to(get_daily_event_counts))
        .route("/events/", web::get().to(get_events))
        .route("/events/{id}/", web::get().to(get_event))
        .route("/events/{id}/details/", web::get().to(get_event_details))
        .route("/events/", web::post().to(register_event))
        .route("/events/{id}/details/", web::post().to(register_event_details))
        .route("/events/{id}/", web::put().to(put_event))
        .route("/events/{id}/details/", web::put().to(put_event_details));
}
