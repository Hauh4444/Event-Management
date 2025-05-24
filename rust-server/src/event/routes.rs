// External Libraries
use actix_web::{web, Responder, HttpResponse, HttpRequest};
use sqlx::SqlitePool;

// Internal Modules
use crate::event::mapper::{get_event_by_id, create_event, delete_event};
use crate::event::models::{EventData, GetEventData, DeleteEventData};
use crate::auth::services::validate_session;


/// Retrieves an event by its unique identifier.
///
/// # Arguments
///
/// * `data` - A JSON object containing the event ID.
/// * `pool` - A reference to the SQLite database connection pool.
///
/// # Returns
///
/// A response indicating whether the event was found or not.
pub async fn get_event(
    req: HttpRequest,
    data: web::Json<GetEventData>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    // TODO use session.user_id as organizer_id check
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };
    
    match get_event_by_id(&data.event_id, &pool).await {
        Ok(event) => HttpResponse::Ok().body(format!("Event {} found", event.id)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Event not found: {}", e)),
    }
}


/// Registers a new event in the system.
///
/// # Arguments
///
/// * `data` - A JSON object containing the event details.
/// * `pool` - A reference to the SQLite database connection pool.
///
/// # Returns
///
/// A response indicating the result of the event registration attempt.
pub async fn register_event(
    req: HttpRequest,
    data: web::Json<EventData>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    // TODO use session.user_id as organizer_id 
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };
    
    match create_event(&data.into_inner(), &pool).await {
        Ok(event) => HttpResponse::Ok().body(format!("Event {} registered", event.title)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to register event: {}", e)),
    }
}


/// Removes an event from the system.
///
/// # Arguments
///
/// * `data` - A JSON object containing the event ID.
/// * `pool` - A reference to the SQLite database connection pool.
///
/// # Returns
///
/// A response indicating the result of the event removal attempt.
pub async fn remove_event(
    req: HttpRequest,
    data: web::Json<DeleteEventData>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    // TODO use session.user_id as organizer_id check
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };
    
    match delete_event(data.event_id, &pool).await {
        Ok(()) => HttpResponse::Ok().body("Event deleted"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to delete event: {}", e)),
    }
}


/// Configures the event-related routes for the application.
///
/// # Arguments
///
/// * `cfg` - A mutable reference to the service configuration.
///
/// # Returns
///
/// Configures the provided service with event routes.
pub fn configure_event_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/event/{id}/", web::get().to(get_event))
        .route("/event/{id}/", web::post().to(register_event))
        .route("/event/{id}/", web::delete().to(remove_event));
}
