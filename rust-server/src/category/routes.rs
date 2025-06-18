// External Libraries
use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;

// Internal Mappers
use crate::category::mapper::fetch_categories;


/// Handles retrieving all categories.
///
/// # Arguments
///
/// * `pool` - The SQLite database connection pool.
///
/// # Returns
///
/// An HTTP response with category data if successful, or an error message.
pub async fn get_categories(
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    match fetch_categories(&pool).await {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(e) => HttpResponse::InternalServerError().body(format!("Categories not found: {}", e)),
    }
}


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
        .route("/categories/", web::get().to(get_categories));
}
