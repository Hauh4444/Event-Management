// External Libraries
use actix_web::{web, Responder, HttpResponse, HttpRequest};
use sqlx::SqlitePool;

// Internal Modules
use crate::overview::mapper::{
    fetch_monthly_totals,
    fetch_monthly_ticket_sales,
    fetch_daily_event_counts,
    fetch_monthly_attendees,
    fetch_attendance_extremes,
    fetch_daily_attendee_counts
};
use crate::overview::models::{
    MonthlyTotals,
    YearQuery,
    TicketTotals,
    GetOverview,
    EventCounts,
    AttendeeTotals,
    AttendanceExtremes,
    AttendeeCounts
};
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


/// Retrieves aggregated attendee data including monthly attendees and total attendees
/// for a specific organizer and year.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `query` - A query parameter containing the year to retrieve attendee data for.
/// * `pool` - A reference to the SQLite database connection pool.
///
/// # Returns
///
/// A JSON response containing attendee data or an error message if the operation fails.
pub async fn get_monthly_attendees(
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

    match fetch_monthly_attendees(GetOverview {organizer_id, year}, &pool).await {
        Ok(totals) => HttpResponse::Ok().json(AttendeeTotals {..totals}),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to fetch monthly attendee data: {}", e)),
    }
}


/// Retrieves daily attendee counts for a specific organizer and year.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `query` - A query parameter containing the year to retrieve data for.
/// * `pool` - A reference to the SQLite database connection pool.
///
/// # Returns
///
/// A JSON response containing a list of daily attendee counts with dates or an error message if the operation fails.
pub async fn get_daily_attendee_counts(
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

    match fetch_daily_attendee_counts(GetOverview {organizer_id, year}, &pool).await {
        Ok(totals) => HttpResponse::Ok().json(AttendeeCounts {..totals}),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to fetch daily attendee counts: {}", e)),
    }
}


/// Retrieves the top 5 most attended and bottom 5 least attended events
/// for a specific organizer and year.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `query` - A query parameter containing the year to retrieve attendance extremes for.
/// * `pool` - A reference to the SQLite database connection pool.
///
/// # Returns
///
/// A JSON response containing the most and least attended events or an error message if the operation fails.
pub async fn get_attendance_extremes(
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

    match fetch_attendance_extremes(GetOverview {organizer_id, year}, &pool).await {
        Ok(totals) => HttpResponse::Ok().json(AttendanceExtremes {..totals}),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to fetch attendance extremes: {}", e)),
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
        .route("/overview/totals/", web::get().to(get_monthly_totals))
        .route("/overview/monthly-ticket-sales/", web::get().to(get_monthly_ticket_sales))
        .route("/overview/daily-event-counts/", web::get().to(get_daily_event_counts))
        .route("/overview/monthly-attendee-counts/", web::get().to(get_monthly_attendees))
        .route("/overview/daily-attendee-counts/", web::get().to(get_daily_attendee_counts))
        .route("/overview/attendance-extremes/", web::get().to(get_attendance_extremes));
}
