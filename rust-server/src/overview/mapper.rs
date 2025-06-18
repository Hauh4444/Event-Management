// External Libraries
use chrono::Datelike;
use sqlx::SqlitePool;

// Internal Models
use crate::overview::models::{
    MonthlyTotals,
    GetOverview,
};
use crate::event::models::{Event};


/// Fetches aggregated event statistics for a specific organizer and year.
///
/// # Arguments
///
/// * `data` - A struct containing the `year` and `organizer_id`.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing an `MonthlyTotals` struct with monthly aggregated statistics,
/// or an `sqlx::Error` if the query fails.
///
/// # Errors
///
/// Returns an error if the query to fetch events fails.
pub async fn fetch_monthly_totals(data: GetOverview, pool: &SqlitePool) -> Result<MonthlyTotals, sqlx::Error> {
    let year = data.year.to_string();
    let organizer_id = data.organizer_id;

    let events = sqlx::query_as!(
        Event,
        "SELECT id, title, description, event_date, start_time, end_time, location, category_id, status, organizer_id, 
                price, tickets_sold, attendees, max_attendees, contact_email, contact_phone, registration_deadline,
                is_virtual, image, map_embed, accessibility_info, safety_guidelines, created_at, updated_at
         FROM events 
         WHERE strftime('%Y', event_date) = ? AND organizer_id = ?",
        year, organizer_id
    )
        .fetch_all(pool)
        .await?;

    let mut events_by_month = vec![0i64; 12];
    let mut upcoming_by_month = vec![0i64; 12];
    let mut canceled_by_month = vec![0i64; 12];
    let mut tickets_by_month = vec![0i64; 12];
    let mut attendees_by_month = vec![0i64; 12];

    for event in events {
        let month = event.event_date.month() as usize - 1;

        events_by_month[month] += 1;
        tickets_by_month[month] += event.tickets_sold;
        attendees_by_month[month] += event.attendees;

        if event.status == "upcoming" {
            upcoming_by_month[month] += 1;
        }
        if event.status == "canceled" {
            canceled_by_month[month] += 1;
        }
    }

    Ok(MonthlyTotals {
        events: events_by_month,
        upcoming: upcoming_by_month,
        canceled: canceled_by_month,
        tickets: tickets_by_month,
        attendees: attendees_by_month,
    })
}