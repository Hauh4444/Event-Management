// External Libraries
use sqlx::SqlitePool;

// Internal Modules
use crate::organizer::models::{Organizer, GetOrganizerData, DeleteOrganizerData};


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
/// Returns an error if the query fails.
pub async fn fetch_organizer(data: GetOrganizerData, pool: &SqlitePool) -> Result<Organizer, sqlx::Error> {
    sqlx::query_as!(
        Organizer,
        "SELECT id, name, logo, website
         FROM organizers
         WHERE id = ?",
        data.organizer_id
    )
        .fetch_one(pool)
        .await
}


/// Removes a specific organizer by its ID.
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
/// Returns an error if the query fails.
pub async fn delete_organizer(data: DeleteOrganizerData, pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM organizers WHERE id = ?",
        data.organizer_id
    )
        .execute(pool)
        .await?;

    Ok(())
}