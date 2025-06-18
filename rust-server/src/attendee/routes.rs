// External Libraries
use actix_web::{web, Responder, HttpResponse, HttpRequest};
use sqlx::SqlitePool;

// Internal Mappers
use crate::attendee::mapper::{fetch_attendees_by_event, fetch_monthly_attendees, fetch_daily_attendee_counts, fetch_attendance_extremes, fetch_monthly_no_shows};
use crate::event::mapper::fetch_event;

// Internal Models
use crate::attendee::models::{GetAttendeeData, AttendeeTotals, AttendanceExtremes, AttendeeCounts, NoShowTotals};
use crate::event::models::GetEventData;
use crate::overview::models::{YearQuery, GetOverview};

// Internal Services
use crate::auth::services::validate_session;


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


/// Retrieves aggregated no show data including monthly no shows and total no shows
/// for a specific organizer and year.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `query` - A query parameter containing the year to retrieve no show data for.
/// * `pool` - A reference to the SQLite database connection pool.
///
/// # Returns
///
/// A JSON response containing no show data or an error message if the operation fails.
pub async fn get_monthly_no_shows(
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
    
    match fetch_monthly_no_shows(GetOverview {organizer_id, year}, &pool).await {
        Ok(totals) => HttpResponse::Ok().json(NoShowTotals {..totals}),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to fetch monthly no show rates: {}", e)),
    }
}


/// Handles retrieving a specific event's attendees by ID, ensuring the organizer owns the event.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `event_id` - The path parameter representing the event's ID.
/// * `pool` - The SQLite database connection pool.
///
/// # Returns
///
/// An HTTP response with the attendee information if found, or an error message.
pub async fn get_attendees_by_event(
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

    match fetch_attendees_by_event(GetAttendeeData {event_id: event.id}, &pool).await {
        Ok(attendees) => HttpResponse::Ok().json(attendees),
        Err(e) => HttpResponse::InternalServerError().body(format!("Attendees not found: {}", e)),
    }
}


/// Configures the attendee-related routes for the application.
///
/// # Arguments
///
/// * `cfg` - A mutable reference to the service configuration.
///
/// # Returns
///
/// Configures the provided service with overview routes.
pub fn configure_attendee_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/attendees/counts/monthly/", web::get().to(get_monthly_attendees))
        .route("/attendees/counts/daily/", web::get().to(get_daily_attendee_counts))
        .route("/attendees/extremes/", web::get().to(get_attendance_extremes))
        .route("/attendees/no-shows/monthly/", web::get().to(get_monthly_no_shows))
        .route("/attendees/{event_id}/", web::get().to(get_attendees_by_event));
}