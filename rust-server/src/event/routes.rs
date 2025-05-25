// External Libraries
use actix_web::{web, Responder, HttpResponse, HttpRequest};
use sqlx::SqlitePool;

// Internal Modules
use crate::event::mapper::{get_user_events, get_event_by_id, create_event, delete_event};
use crate::event::models::{Event, EventData, GetUserEventsData, GetEventData, DeleteEventData};
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
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };
    
    match get_user_events(GetUserEventsData {organizer_id: session.user_id}, &pool).await {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(e) => HttpResponse::InternalServerError().body(format!("Event not found: {}", e)),
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
/// An HTTP response with the event details if found, or an error message.
pub async fn get_event(
    req: HttpRequest,
    event_id: web::Path<i64>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };

    match get_event_by_id(GetEventData {event_id: *event_id, organizer_id: session.user_id}, &pool).await {
        Ok(event) => HttpResponse::Ok().json(Event {..event}),
        Err(e) => HttpResponse::InternalServerError().body(format!("Event not found: {}", e)),
    }
}


/// Handles registering a new event under the authenticated organizer.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `data` - The JSON body containing new event details.
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

    match create_event(EventData {organizer_id: session.user_id, ..data.into_inner()}, &pool).await {
        Ok(event) => HttpResponse::Ok().body(format!("Event '{}' registered", event.title)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to register event: {}", e)),
    }
}


/// Handles deleting a specific event by ID if it belongs to the authenticated organizer.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `event_id` - The path parameter representing the event's ID.
/// * `pool` - The SQLite database connection pool.
///
/// # Returns
///
/// An HTTP response indicating success or failure of deletion.
pub async fn remove_event(
    req: HttpRequest,
    event_id: web::Path<i64>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };

    match delete_event(DeleteEventData {event_id: *event_id, organizer_id: session.user_id}, &pool).await {
        Ok(()) => HttpResponse::Ok().body("Event deleted"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to delete event: {}", e)),
    }
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
        .route("/events/", web::post().to(register_event))
        .route("/events/{id}/", web::delete().to(remove_event));
}
