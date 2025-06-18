// External Libraries
use actix_web::{web, Responder, HttpResponse, HttpRequest};
use sqlx::SqlitePool;

// Internal Mappers
use crate::overview::mapper::{
    fetch_monthly_totals,
};

// Internal Models
use crate::overview::models::{
    MonthlyTotals,
    YearQuery,
    GetOverview,
};

// Internal Services
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
pub async fn get_monthly_totals(
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

    match fetch_monthly_totals(GetOverview {organizer_id, year}, &pool).await {
        Ok(totals) => HttpResponse::Ok().json(MonthlyTotals {..totals}),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to fetch monthly totals: {}", e)),
    }
}


/// Configures the overview-related routes for the application.
///
/// # Arguments
///
/// * `cfg` - A mutable reference to the service configuration.
///
/// # Returns
///
/// Configures the provided service with overview routes.
pub fn configure_overview_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/overview/totals/", web::get().to(get_monthly_totals));
}
