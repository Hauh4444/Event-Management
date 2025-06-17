// External Libraries
use sqlx::SqlitePool;
use chrono::Datelike;

// Internal Modules
use crate::overview::models::{
    MonthlyTotals,
    GetOverview,
    TicketTotals,
    EventCounts,
    CountByDate,
    AttendeeTotals,
    AttendanceExtremes,
    AttendeeCounts,
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


/// Fetches monthly ticket revenue and total profit for a specific organizer and year.
///
/// # Arguments
///
/// * `data` - A struct containing the `year` and `organizer_id`.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing a `TicketTotals` struct with monthly ticket revenue and total profit,
/// or an `sqlx::Error` if the query fails.
///
/// # Errors
///
/// Returns an error if the query to fetch events fails.
pub async fn fetch_monthly_ticket_sales(data: GetOverview, pool: &SqlitePool) -> Result<TicketTotals, sqlx::Error> {
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

    Ok(TicketTotals {
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
/// A `Result` containing an `EventCounts` struct with daily event counts,
/// or an `sqlx::Error` if the query fails.
///
/// # Errors
///
/// Returns an error if the query to fetch daily event counts fails.
pub async fn fetch_daily_event_counts(data: GetOverview, pool: &SqlitePool) -> Result<EventCounts, sqlx::Error> {
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
            Some(CountByDate { date: day, count: row.event_count as usize })
        } else {
            None
        }
    }).collect();

    Ok(EventCounts {
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

    let mut attendees_by_month = vec![0f64; 12];
    let mut total_attendees = 0f64;

    for event in events {
        let month = event.event_date.month() as usize - 1;

        attendees_by_month[month] += event.attendees as f64;
        total_attendees += event.attendees as f64;
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