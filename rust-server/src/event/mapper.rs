// External Libraries
use sqlx::SqlitePool;

// Internal Modules
use crate::event::models::{Event, EventData};


/// Retrieves an event from the database by its ID.
///
/// # Arguments
///
/// * `id` - A reference to the ID of the event to retrieve.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing the `Event` if found, or an error if the query fails.
pub async fn get_event_by_id(id: &i64, pool: &SqlitePool) -> Result<Event, sqlx::Error> {
    sqlx::query_as!(
        Event,
        "SELECT * FROM events WHERE id = ?",
        id
    )
        .fetch_one(pool)
        .await
}


/// Creates a new event in the database.
///
/// # Arguments
///
/// * `data` - A reference to the `EventData` struct containing event details.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing the newly created `Event`, or an error if the query fails.
pub async fn create_event(data: &EventData, pool: &SqlitePool) -> Result<Event, sqlx::Error> {
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


/// Deletes an event from the database by its ID.
///
/// # Arguments
///
/// * `event_id` - The ID of the event to delete.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` indicating success or failure of the deletion.
pub async fn delete_event(event_id: i64, pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM events WHERE id = ?",
        event_id
    )
        .execute(pool)
        .await?;

    Ok(())
}