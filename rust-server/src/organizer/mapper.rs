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


/// Creates an organizer in the database.
///
/// # Arguments
///
/// * `data` - A struct containing all the new organizer data.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing the newly created `Organizer`, or an `sqlx::Error` if the update fails.
///
/// # Errors
///
/// Returns an error if the query fails or any constraint is violated.
pub async fn create_organizer(data: Organizer, pool: &SqlitePool) -> Result<Organizer, sqlx::Error> {
    let rec = sqlx::query_as!(
        Organizer,
        "INSERT INTO organizers (id, name, logo, website)
         VALUES (?, ?, ?, ?)
         RETURNING id, name, logo, website",
        data.id, data.name, data.logo, data.website
    )
        .fetch_one(pool)
        .await?;

    Ok(rec)
}


/// Updates an organizer in the database.
///
/// # Arguments
///
/// * `data` - A struct containing all the organizer data.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing the newly updated `Organizer`, or an `sqlx::Error` if the update fails.
///
/// # Errors
///
/// Returns an error if the query fails or any constraint is violated.
pub async fn update_organizer(data: Organizer, pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query_as!(
        Organizer,
        "UPDATE organizers
         SET name = ?, logo = ?, website = ?
         WHERE id = ?",
        data.name, data.logo, data.website, data.id
    )
        .execute(pool)
        .await?;

    Ok(())
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