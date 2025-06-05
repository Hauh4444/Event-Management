// External Libraries
use sqlx::SqlitePool;

// Internal Modules
use crate::comment::models::{Comment, GetCommentData};


/// Retrieves comment items by their event ID.
///
/// # Arguments
///
/// * `data` - A struct containing the `event_id`.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing a list of `Comments` if found, or an `sqlx::Error` if the query fails.
///
/// # Errors
///
/// Returns an error if the query fails or no comment is found.
pub async fn fetch_comments(data: GetCommentData, pool: &SqlitePool) -> Result<Vec<Comment>, sqlx::Error> {
    let event_id = data.event_id;

    sqlx::query_as!(
        Comment,
        "SELECT *
         FROM comments
         WHERE event_id = ?",
        event_id
    )
        .fetch_all(pool)
        .await
}