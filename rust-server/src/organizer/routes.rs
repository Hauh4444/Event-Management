// External Libraries
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use sqlx::SqlitePool;

// Internal Mappers
use crate::organizer::mapper::{fetch_organizer, create_organizer, update_organizer};

// Internal Models
use crate::organizer::models::{Organizer, OrganizerData, GetOrganizerData};

// Internal Services
use crate::auth::services::validate_session;


/// Handles retrieving a specific organizer by session token.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `pool` - The SQLite database connection pool.
///
/// # Returns
///
/// An HTTP response with the event information if found, or an error message.
pub async fn get_organizer(
    req: HttpRequest,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };

    match fetch_organizer(GetOrganizerData { organizer_id: session.user_id }, &pool).await {
        Ok(organizer) => HttpResponse::Ok().json(organizer),
        Err(e) => HttpResponse::InternalServerError().body(format!("Organizer not found: {}", e)),
    }
}


/// Handles registering a new organizer.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `data` - The JSON body containing new organizer data.
/// * `pool` - The SQLite database connection pool.
///
/// # Returns
///
/// An HTTP response indicating success or failure of organizer creation.
pub async fn register_organizer(
    req: HttpRequest,
    data: web::Json<OrganizerData>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };
    
    let OrganizerData {
        name,
        logo,
        website,
    } = data.into_inner();
    
    // TODO Save new image file and update image location reference

    match create_organizer(Organizer {id: session.user_id, name, logo, website}, &pool).await {
        Ok(organizer) => HttpResponse::Ok().json(organizer),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to create organizer: {}", e)),
    }
}


/// Handles updating an organizer.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `data` - The JSON body containing new organizer data.
/// * `pool` - The SQLite database connection pool.
///
/// # Returns
///
/// An HTTP response indicating success or failure of updating organizer.
pub async fn put_organizer(
    req: HttpRequest,
    data: web::Json<Organizer>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };

    let organizer = match fetch_organizer(GetOrganizerData { organizer_id: session.user_id }, &pool).await {
        Ok(organizer) => organizer,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Organizer not found: {}", e)),
    };
    
    // TODO Remove old and save new image file and update image location reference
    if data.logo != organizer.logo {
        
    }

    match update_organizer(Organizer {id: session.user_id, ..data.into_inner()}, &pool).await {
        Ok(()) => HttpResponse::Ok().body(format!("Organizer '{}' updated", session.user_id)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to update organizer: {}", e)),
    }
}


/// Configures all routes related to organizer management.
///
/// # Arguments
///
/// * `cfg` - A mutable reference to the Actix service configuration.
///
/// # Returns
///
/// Adds all event-related routes to the Actix web application.
pub fn configure_organizer_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/organizer/", web::get().to(get_organizer))
        .route("/organizer/", web::post().to(register_organizer))
        .route("/organizer/", web::put().to(put_organizer));
}
