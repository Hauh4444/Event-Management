// External Libraries
use sqlx::SqlitePool;

// Internal Modules
use crate::event::models::{Event, EventData, GetUserEventsData, GetEventData, DeleteEventData};


/// Retrieves all events created by a specific organizer.
///
/// # Arguments
///
/// * `data` - A struct containing the `organizer_id`.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing the `Event` if found, or an `sqlx::Error` if the query fails.
///
/// # Errors
///
/// Returns an error if the query fails or no event is found.
pub async fn get_user_events(data: GetUserEventsData, pool: &SqlitePool) -> Result<Vec<Event>, sqlx::Error> {
    sqlx::query_as!(
        Event,
        "SELECT * FROM events WHERE organizer_id = ? ORDER BY event_date ASC",
        data.organizer_id
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
pub async fn get_event_by_id(data: GetEventData, pool: &SqlitePool) -> Result<Event, sqlx::Error> {
    sqlx::query_as!(
        Event,
        "SELECT * FROM events WHERE id = ? AND organizer_id = ?",
        data.event_id, data.organizer_id
    )
        .fetch_one(pool)
        .await
}


/// Inserts a new event into the database.
///
/// # Arguments
///
/// * `data` - A struct containing all the event details.
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
                     is_virtual, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                     RETURNING *",
        data.title, data.description, data.event_date, data.start_time, data.end_time, data.location, data.category_id, 
        data.status, data.organizer_id, data.price, data.tickets_sold, data.attendees, data.max_attendees,
        data.contact_email, data.contact_phone, data.registration_deadline, data.is_virtual, data.created_at, data.updated_at
    )
        .fetch_one(pool)
        .await?;

    Ok(rec)
}


/// Deletes an event from the database by its ID and organizer ID.
///
/// # Arguments
///
/// * `data` - A struct containing the `event_id` and `organizer_id`.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` indicating success or an `sqlx::Error` if the deletion fails.
///
/// # Errors
///
/// Returns an error if the query fails or the specified event does not exist.
pub async fn delete_event(data: DeleteEventData, pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM events WHERE id = ? AND organizer_id = ?",
        data.event_id, data.organizer_id
    )
        .execute(pool)
        .await?;

    Ok(())
}