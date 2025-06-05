// External Libraries
use sqlx::SqlitePool;

// Internal Modules
use crate::event::models::{Event, EventData, GetUserEventsData, GetEventData};


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
pub async fn fetch_events(data: GetUserEventsData, pool: &SqlitePool) -> Result<Vec<Event>, sqlx::Error> {
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
pub async fn fetch_event(data: GetEventData, pool: &SqlitePool) -> Result<Event, sqlx::Error> {
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
pub async fn create_event(data: EventData, pool: &SqlitePool) -> Result<Event, sqlx::Error> {
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
pub async fn update_event(data: Event, pool: &SqlitePool) -> Result<(), sqlx::Error> {
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