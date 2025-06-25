// External Libraries
use chrono::Datelike;
use sqlx::SqlitePool;

// Internal Models
use crate::event::models::{
    Event,
    EventData,
    GetUserEventsData,
    GetEventData,
    TicketTotals, 
    EventCounts,
};
use crate::overview::models::{
    CountByDate, 
    GetOverview,
};


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
pub async fn fetch_monthly_ticket_sales(
    data: GetOverview, 
    pool: &SqlitePool
) -> Result<TicketTotals, sqlx::Error> {
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
pub async fn fetch_daily_event_counts(
    data: GetOverview, 
    pool: &SqlitePool
) -> Result<EventCounts, sqlx::Error> {
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


/// Retrieves all events created by a specific organizer.
///
/// # Arguments
///
/// * `data` - A struct containing the `organizer_id`.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing a list of `Events` if found, or an `sqlx::Error` if the query fails.
///
/// # Errors
///
/// Returns an error if the query fails or no event is found.
pub async fn fetch_events(
    data: GetUserEventsData, 
    pool: &SqlitePool
) -> Result<Vec<Event>, sqlx::Error> {
    let year = data.year.to_string();
    let organizer_id = data.organizer_id;

    sqlx::query_as!(
        Event,
        "SELECT id, title, description, event_date, start_time, end_time, location, category_id, status, organizer_id,
                price, tickets_sold, attendees, max_attendees, contact_email, contact_phone, registration_deadline,
                is_virtual, image, map_embed, accessibility_info, safety_guidelines, created_at, updated_at
         FROM events
         WHERE strftime('%Y', event_date) = ? AND organizer_id = ?
         ORDER BY event_date ASC",
        year, organizer_id
    )
        .fetch_all(pool)
        .await
}


/// Retrieves a specific event by its ID and organizer ID.
///
/// # Arguments
///
/// * `data` - A struct containing the `event_id` and `organizer_id`.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing the `Event` if found, or an `sqlx::Error` if the query fails.
///
/// # Errors
///
/// Returns an error if the query fails or the event does not match the provided IDs.
pub async fn fetch_event(
    data: GetEventData, 
    pool: &SqlitePool
) -> Result<Event, sqlx::Error> {
    let event_id = data.event_id;
    let organizer_id = data.organizer_id;

    sqlx::query_as!(
        Event,
        "SELECT id, title, description, event_date, start_time, end_time, location, category_id, status, organizer_id, 
                price, tickets_sold, attendees, max_attendees, contact_email, contact_phone, registration_deadline,
                is_virtual, image, map_embed, accessibility_info, safety_guidelines, created_at, updated_at
         FROM events 
         WHERE id = ? AND organizer_id = ?",
        event_id, organizer_id
    )
        .fetch_one(pool)
        .await
}


/// Inserts a new event into the database.
///
/// # Arguments
///
/// * `data` - A struct containing all the event data.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing the newly created `Event`, or an `sqlx::Error` if the insert fails.
///
/// # Errors
///
/// Returns an error if the query fails or any constraint is violated.
pub async fn create_event(
    data: EventData, 
    pool: &SqlitePool
) -> Result<Event, sqlx::Error> {
    let rec = sqlx::query_as!(
        Event,
        "INSERT INTO events (title, description, event_date, start_time, end_time, location, category_id, status, organizer_id, 
                     price, tickets_sold, attendees, max_attendees, contact_email, contact_phone, registration_deadline,
                     is_virtual, image, map_embed, accessibility_info, safety_guidelines, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
         RETURNING id, title, description, event_date, start_time, end_time, location, category_id, status, 
                   organizer_id, price, tickets_sold, attendees, max_attendees, contact_email, contact_phone, 
                   registration_deadline, is_virtual, image, map_embed, accessibility_info, safety_guidelines,
                   created_at, updated_at",
        data.title, data.description, data.event_date, data.start_time, data.end_time, data.location, data.category_id, 
        data.status, data.organizer_id, data.price, data.tickets_sold, data.attendees, data.max_attendees,
        data.contact_email, data.contact_phone, data.registration_deadline, data.is_virtual, data.image, data.map_embed,
        data.accessibility_info, data.safety_guidelines, data.created_at, data.updated_at
    )
        .fetch_one(pool)
        .await?;

    Ok(rec)
}


/// Updates an event in the database.
///
/// # Arguments
///
/// * `data` - A struct containing all the event data.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing the newly updated `Event`, or an `sqlx::Error` if the update fails.
///
/// # Errors
///
/// Returns an error if the query fails or any constraint is violated.
pub async fn update_event(
    data: Event, 
    pool: &SqlitePool
) -> Result<(), sqlx::Error> {
    sqlx::query_as!(
        Event,
        "UPDATE events 
         SET title = ?, description = ?, event_date = ?, start_time = ?, end_time = ?, location = ?, category_id = ?,
             status = ?, organizer_id = ?, price = ?, tickets_sold = ?, attendees = ?, max_attendees = ?, 
             contact_email = ?, contact_phone = ?, registration_deadline = ?, is_virtual = ?, image = ?, map_embed = ?, 
             accessibility_info = ?, safety_guidelines = ?, updated_at = CURRENT_TIMESTAMP
         WHERE id = ?",
        data.title, data.description, data.event_date, data.start_time, data.end_time, data.location, data.category_id, 
        data.status, data.organizer_id, data.price, data.tickets_sold, data.attendees, data.max_attendees,
        data.contact_email, data.contact_phone, data.registration_deadline, data.is_virtual, data.image, data.map_embed,
        data.accessibility_info, data.safety_guidelines, data.id
    )
        .execute(pool)
        .await?;

    Ok(())
}