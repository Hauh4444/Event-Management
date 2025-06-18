// External Libraries
use chrono::Datelike;
use sqlx::SqlitePool;

// Internal Models
use crate::attendee::models::{Attendee, GetAttendeeData, AttendeeTotals, AttendanceExtremes, AttendeeCounts, NoShowTotals};
use crate::event::models::Event;
use crate::overview::models::{CountByDate, GetOverview};


/// Fetches monthly attendees and total attendees for a specific organizer and year.
///
/// # Arguments
///
/// * `data` - A struct containing the `year` and `organizer_id`.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing a `AttendeeTotals` struct with monthly attendees and total attendees,
/// or an `sqlx::Error` if the query fails.
///
/// # Errors
///
/// Returns an error if the query to fetch events fails.
pub async fn fetch_monthly_attendees(data: GetOverview, pool: &SqlitePool) -> Result<AttendeeTotals, sqlx::Error> {
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

    let mut attendees_by_month = vec![0i64; 12];
    let mut total_attendees = 0i64;

    for event in events {
        let month = event.event_date.month() as usize - 1;

        attendees_by_month[month] += event.attendees;
        total_attendees += event.attendees;
    }

    Ok(AttendeeTotals {
        attendees: attendees_by_month,
        total: total_attendees,
    })
}


/// Fetches daily attendee counts for a specific organizer and year.
///
/// # Arguments
///
/// * `data` - A struct containing the `year` and `organizer_id`.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing an `AttendeeCounts` struct with daily attendee counts,
/// or an `sqlx::Error` if the query fails.
///
/// # Errors
///
/// Returns an error if the query to fetch daily attendee counts fails.
pub async fn fetch_daily_attendee_counts(data: GetOverview, pool: &SqlitePool) -> Result<AttendeeCounts, sqlx::Error> {
    let year = data.year.to_string();
    let organizer_id = data.organizer_id;

    let daily_rows = sqlx::query!(
        r#"
        SELECT
            strftime('%Y-%m-%d', event_date) AS day,
            SUM(attendees) AS attendee_count
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
            Some(CountByDate { date: day, count: row.attendee_count as usize })
        } else {
            None
        }
    }).collect();

    Ok(AttendeeCounts {
        attendee_counts: daily_totals,
    })
}


/// Fetches the top 5 most attended and bottom 5 least attended completed events
/// for a specific organizer and year.
///
/// Only events with a `status` of `'complete'` and that occurred before the current date
/// are considered in the query.
///
/// # Arguments
///
/// * `data` - A struct containing the `year` and `organizer_id`.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing an `AttendanceExtremes` struct with the 5 most and 5 least attended events,
/// or an `sqlx::Error` if the query fails.
///
/// # Errors
///
/// Returns an error if either query to fetch the top or bottom attended events fails.
pub async fn fetch_attendance_extremes(data: GetOverview, pool: &SqlitePool) -> Result<AttendanceExtremes, sqlx::Error> {
    let year = data.year.to_string();
    let organizer_id = data.organizer_id;

    let most_attended = sqlx::query_as!(
        Event,
        "SELECT id, title, description, event_date, start_time, end_time, location, category_id, status, organizer_id, 
                price, tickets_sold, attendees, max_attendees, contact_email, contact_phone, registration_deadline,
                is_virtual, image, map_embed, accessibility_info, safety_guidelines, created_at, updated_at
         FROM events 
         WHERE event_date < CURRENT_DATE 
           AND strftime('%Y', event_date) = ? 
           AND status = 'complete' 
           AND organizer_id = ?
         ORDER BY attendees DESC
         LIMIT 5",
        year, organizer_id
    )
        .fetch_all(pool)
        .await?;

    let least_attended = sqlx::query_as!(
        Event,
        "SELECT id, title, description, event_date, start_time, end_time, location, category_id, status, organizer_id, 
                price, tickets_sold, attendees, max_attendees, contact_email, contact_phone, registration_deadline,
                is_virtual, image, map_embed, accessibility_info, safety_guidelines, created_at, updated_at
         FROM events 
         WHERE event_date < CURRENT_DATE 
           AND strftime('%Y', event_date) = ? 
           AND status = 'complete' 
           AND organizer_id = ?
         ORDER BY attendees ASC
         LIMIT 5",
        year, organizer_id
    )
        .fetch_all(pool)
        .await?;

    Ok(AttendanceExtremes {
        most: most_attended,
        least: least_attended,
    })
}


/// Fetches monthly no shows and total no shows for a specific organizer and year.
///
/// # Arguments
///
/// * `data` - A struct containing the `year` and `organizer_id`.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing a `NoShowTotals` struct with monthly no shows and total no shows,
/// or an `sqlx::Error` if the query fails.
///
/// # Errors
///
/// Returns an error if the query to fetch events fails.
pub async fn fetch_monthly_no_shows(data: GetOverview, pool: &SqlitePool) -> Result<NoShowTotals, sqlx::Error> {
    let year = data.year.to_string();
    let organizer_id = data.organizer_id;

    let events = sqlx::query_as!(
        Event,
        "SELECT id, title, description, event_date, start_time, end_time, location, category_id, status, organizer_id, 
                price, tickets_sold, attendees, max_attendees, contact_email, contact_phone, registration_deadline,
                is_virtual, image, map_embed, accessibility_info, safety_guidelines, created_at, updated_at
         FROM events 
         WHERE strftime('%Y', event_date) = ? AND event_date < CURRENT_DATE AND organizer_id = ?",
        year, organizer_id
    )
        .fetch_all(pool)
        .await?;

    let mut event_counts_by_month = vec![0i64; 12];
    let mut no_show_counts_by_month = vec![0i64; 12];
    let mut no_show_rates_by_month = vec![0f64; 12];
    let mut total_no_show_count = 0i64;
    let mut total_no_show_rate = 0f64;

    for event in &events {
        let month = event.event_date.month() as usize - 1;

        no_show_counts_by_month[month] += event.tickets_sold - event.attendees;
        total_no_show_count += event.tickets_sold - event.attendees;
        event_counts_by_month[month] += 1;
    }
    
    for month in 0..12 {
        if no_show_counts_by_month[month] > 0 {
            no_show_rates_by_month[month] = no_show_counts_by_month[month] as f64 / event_counts_by_month[month] as f64;
        }
    }
    
    if total_no_show_count > 0 {
        total_no_show_rate = total_no_show_count as f64 / events.len() as f64;
    }

    Ok(NoShowTotals {
        no_show_counts: no_show_counts_by_month,
        no_show_rates: no_show_rates_by_month,
        total_count: total_no_show_count,
        total_rate: total_no_show_rate,
    })
}


/// Retrieves all attendees for a specific event.
///
/// # Arguments
///
/// * `data` - A struct containing the `organizer_id`.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing a list of `Attendees` if found, or an `sqlx::Error` if the query fails.
///
/// # Errors
///
/// Returns an error if the query fails or no attendee is found.
pub async fn fetch_attendees_by_event(data: GetAttendeeData, pool: &SqlitePool) -> Result<Vec<Attendee>, sqlx::Error> {
    let event_id = data.event_id;

    sqlx::query_as!(
        Attendee,
        "SELECT id, event_id, name, email, ticket_type, registration_date
         FROM attendees
         WHERE event_id = ?",
        event_id
    )
        .fetch_all(pool)
        .await
}