// External Libraries
use sqlx::SqlitePool;

// Internal Modules
use crate::organizer::models::{Organizer, GetOrganizerData};


/// Retrieves a specific organizer by its ID.
///
/// # Arguments
///
/// * `data` - A struct containing the `organizer_id`.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing the `Organizer` if found, or an `sqlx::Error` if the query fails.
///
/// # Errors
///
/// Returns an error if the query fails or the event does not match the provided ID.
pub async fn fetch_organizer(data: GetOrganizerData, pool: &SqlitePool) -> Result<Organizer, sqlx::Error> {
    let organizer_id = data.organizer_id;

    sqlx::query_as!(
        Organizer,
        "SELECT id, name, logo, website
         FROM organizers
         WHERE id = ?",
        organizer_id
    )
        .fetch_one(pool)
        .await
}