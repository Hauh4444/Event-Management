// External Libraries
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use sqlx::{SqlitePool};

// Internal Modules


/// Configures all routes related to category management.
///
/// # Arguments
///
/// * `cfg` - A mutable reference to the Actix service configuration.
///
/// # Returns
///
/// Adds all event-related routes to the Actix web application.
pub fn configure_category_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // TODO configure backend category management
        .route("/categories/", web::get().to(HttpResponse::Ok));
}
