// External Libraries
use actix_web::{web, Responder, HttpResponse, HttpRequest};
use sqlx::{SqlitePool};

// Internal Mappers
use crate::event::mapper::{fetch_events, fetch_event, create_event, update_event};
use crate::organizer::mapper::fetch_organizer;
use crate::agenda::mapper::{fetch_agenda, create_agenda, update_agenda};
use crate::speaker::mapper::{fetch_speakers, create_speakers, update_speakers};
use crate::faq::mapper::{fetch_faqs, create_faqs, update_faqs};
use crate::attachment::mapper::{fetch_attachments, create_attachments, update_attachments};
use crate::comment::mapper::fetch_comments;

// Internal Models
use crate::event::models::{Event, EventData, GetUserEventsQuery, GetUserEventsData, GetEventData, EventDetails, CreateEventDetails};
use crate::organizer::models::{Organizer, GetOrganizerData};
use crate::agenda::models::GetAgendaData;
use crate::speaker::models::GetSpeakerData;
use crate::faq::models::GetFaqData;
use crate::attachment::models::GetAttachmentData;
use crate::comment::models::GetCommentData;

// Internal Services
use crate::auth::services::validate_session;


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
        .route("/events/", web::get().to(get_events))
        .route("/events/{id}/", web::get().to(get_event))
        .route("/events/{id}/details/", web::get().to(get_event_details))
        .route("/events/", web::post().to(register_event))
        .route("/events/{id}/details/", web::post().to(register_event_details))
        .route("/events/{id}/", web::put().to(put_event))
        .route("/events/{id}/details/", web::put().to(put_event_details));
}
