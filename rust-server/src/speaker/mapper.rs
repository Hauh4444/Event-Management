// External Libraries
use sqlx::SqlitePool;

// Internal Modules
use crate::speaker::models::{Speaker, GetSpeakerData};


/// Retrieves speaker items by their event ID.
///
/// # Arguments
///
/// * `data` - A struct containing the `event_id`.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing a list of `Speakers` if found, or an `sqlx::Error` if the query fails.
///
/// # Errors
///
/// Returns an error if the query fails or no speaker is found.
pub async fn fetch_speakers(data: GetSpeakerData, pool: &SqlitePool) -> Result<Vec<Speaker>, sqlx::Error> {
    let event_id = data.event_id;

    sqlx::query_as!(
        Speaker,
        "SELECT id, event_id, name, bio, photo
         FROM speakers
         WHERE event_id = ?",
        event_id
    )
        .fetch_all(pool)
        .await
}


/// Updates multiple speaker items in the database.
///
/// # Arguments
///
/// * `data` - A vector of `Speaker` structs containing the updated speaker items.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` indicating success (`Ok(())`) or failure (`Err(sqlx::Error)`).
///
/// # Errors
///
/// Returns an error if any of the update queries fail during execution.
pub async fn update_speakers(data: Vec<Speaker>, pool: &SqlitePool) -> Result<(), sqlx::Error> {
    for speaker_item in data {
        sqlx::query_as!(
            Speaker,
            "UPDATE speakers 
             SET name = ?, bio = ?, photo = ? 
             WHERE id = ?",
            speaker_item.name, speaker_item.bio, speaker_item.photo, speaker_item.id
        )
            .execute(pool)
            .await?;
    };

    Ok(())
}