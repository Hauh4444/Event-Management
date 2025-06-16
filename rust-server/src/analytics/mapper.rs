// External Libraries
use sqlx::SqlitePool;
use chrono::Datelike;

// Internal Modules
use crate::analytics::models::{OverviewTotals, GetOverview, TicketsOverview, EventsOverview, DateCount, AttendeesOverview};
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
/// A `Result` containing an `OverviewTotals` struct with monthly aggregated statistics,
/// or an `sqlx::Error` if the query fails.
///
/// # Errors
///
/// Returns an error if the query to fetch events fails.
pub async fn fetch_overview_monthly_totals(data: GetOverview, pool: &SqlitePool) -> Result<OverviewTotals, sqlx::Error> {
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

    Ok(OverviewTotals {
        events: events_by_month,
        upcoming: upcoming_by_month,
        canceled: canceled_by_month,
        tickets: tickets_by_month,
        attendees: attendees_by_month,
    })
}


/// Fetches monthly ticket revenue and total profit for a specific organizer and year.
///
/// # Arguments
///
/// * `data` - A struct containing the `year` and `organizer_id`.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing a `TicketsOverview` struct with monthly ticket revenue and total profit,
/// or an `sqlx::Error` if the query fails.
///
/// # Errors
///
/// Returns an error if the query to fetch events fails.
pub async fn fetch_tickets_monthly_overview(data: GetOverview, pool: &SqlitePool) -> Result<TicketsOverview, sqlx::Error> {
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

    let mut tickets_by_month = vec![0f64; 12];
    let mut total_profit = 0f64;

    for event in events {
        let month = event.event_date.month() as usize - 1;

        tickets_by_month[month] += event.tickets_sold as f64 * event.price;
        total_profit += event.tickets_sold as f64 * event.price;
    }

    Ok(TicketsOverview {
        tickets: tickets_by_month,
        profit: total_profit,
    })
}


/// Fetches daily event counts for a specific organizer and year.
///
/// # Arguments
///
/// * `data` - A struct containing the `year` and `organizer_id`.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing an `EventsOverview` struct with daily event counts,
/// or an `sqlx::Error` if the query fails.
///
/// # Errors
///
/// Returns an error if the query to fetch daily event counts fails.
pub async fn fetch_events_daily_overview(data: GetOverview, pool: &SqlitePool) -> Result<EventsOverview, sqlx::Error> {
    let year = data.year.to_string();
    let organizer_id = data.organizer_id;

    let daily_rows = sqlx::query!(
        r#"
        SELECT
            strftime('%Y-%m-%d', event_date) AS day,
            COUNT(*) AS event_count
        FROM events
        WHERE strftime('%Y', event_date) = ? AND organizer_id = ?
        GROUP BY day
        ORDER BY day
        "#,
        year,
        organizer_id
    )
        .fetch_all(pool)
        .await?;

    let daily_totals = daily_rows.into_iter().filter_map(|row| {
        if let Some(day) = row.day {
            Some(DateCount { date: day, count: row.event_count as usize })
        } else {
            None
        }
    }).collect();

    Ok(EventsOverview {
        event_counts: daily_totals,
    })
}


/// Fetches monthly attendees and total attendees for a specific organizer and year.
///
/// # Arguments
///
/// * `data` - A struct containing the `year` and `organizer_id`.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing a `AttendeesOverview` struct with monthly attendees and total attendees,
/// or an `sqlx::Error` if the query fails.
///
/// # Errors
///
/// Returns an error if the query to fetch events fails.
pub async fn fetch_attendees_monthly_overview(data: GetOverview, pool: &SqlitePool) -> Result<AttendeesOverview, sqlx::Error> {
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

    let mut attendees_by_month = vec![0f64; 12];
    let mut total_attendees = 0f64;

    for event in events {
        let month = event.event_date.month() as usize - 1;

        attendees_by_month[month] += event.attendees as f64;
        total_attendees += event.attendees as f64;
    }

    Ok(AttendeesOverview {
        attendees: attendees_by_month,
        total: total_attendees,
    })
}