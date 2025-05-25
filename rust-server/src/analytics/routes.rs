// External Libraries
use actix_web::{web, Responder, HttpResponse, HttpRequest};
use sqlx::SqlitePool;

// Internal Modules
use crate::analytics::mapper::{fetch_overview_totals, fetch_tickets_overview};
use crate::analytics::models::{OverviewTotals, OverviewQuery, GetOverviewTotals, TicketsOverview, GetTicketsOverview};
use crate::auth::services::validate_session;


/// Retrieves aggregated monthly totals for events, upcoming events, canceled events,
/// ticket sales, and attendees for a specific year.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `query` - A query parameter containing the year to retrieve data for.
/// * `pool` - A reference to the SQLite database connection pool.
///
/// # Returns
///
/// A JSON response containing the aggregated totals for each category or an error message if the operation fails.
pub async fn get_overview_totals(
    req: HttpRequest,
    query: web::Query<OverviewQuery>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };

    let year = query.year;
    let organizer_id = session.user_id;

    match fetch_overview_totals(GetOverviewTotals {organizer_id, year}, &pool).await {
        Ok(totals) => HttpResponse::Ok().json(OverviewTotals {..totals}),
        Err(e) => HttpResponse::InternalServerError().body(format!("Totals not found: {}", e)),
    }
}


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
pub async fn get_tickets_overview(
    req: HttpRequest,
    query: web::Query<OverviewQuery>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };
    
    let year = query.year;
    let organizer_id = session.user_id;
    
    match fetch_tickets_overview(GetTicketsOverview {organizer_id, year}, &pool).await {
        Ok(totals) => HttpResponse::Ok().json(TicketsOverview {..totals}),
        Err(e) => HttpResponse::InternalServerError().body(format!("Ticket totals not found: {}", e)),       
    }
}


/// Configures the analytics-related routes for the application.
///
/// # Arguments
///
/// * `cfg` - A mutable reference to the service configuration.
///
/// # Returns
///
/// Configures the provided service with analytics routes.
pub fn configure_analytics_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/overview/totals/", web::get().to(get_overview_totals))
        .route("/overview/tickets/", web::get().to(get_tickets_overview));
}
