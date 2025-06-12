// External Libraries
use sqlx::SqlitePool;

// Internal Modules
use crate::attachment::models::{Attachment, GetAttachmentData};


/// Retrieves attachment items by their event ID.
///
/// # Arguments
///
/// * `data` - A struct containing the `event_id`.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing a list of `Attachments` if found, or an `sqlx::Error` if the query fails.
///
/// # Errors
///
/// Returns an error if the query fails or no attachment is found.
pub async fn fetch_attachments(data: GetAttachmentData, pool: &SqlitePool) -> Result<Vec<Attachment>, sqlx::Error> {
    let event_id = data.event_id;

    sqlx::query_as!(
        Attachment,
        "SELECT id, event_id, name, url
         FROM attachments
         WHERE event_id = ?",
        event_id
    )
        .fetch_all(pool)
        .await
}


/// Creates multiple attachment items in the database.
///
/// # Arguments
///
/// * `data` - A vector of `Attachment` structs containing the created attachment items.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` indicating success (`Ok(())`) or failure (`Err(sqlx::Error)`).
///
/// # Errors
///
/// Returns an error if any of the creation queries fail during execution.
pub async fn create_attachments(data: Vec<Attachment>, pool: &SqlitePool) -> Result<Vec<Attachment>, sqlx::Error> {
    let mut attachments = Vec::new();
    
    for attachment_item in data {
        let rec = sqlx::query_as!(
            Attachment,
            "INSERT INTO attachments (event_id, name, url)
             VALUES (?, ?, ?)
             RETURNING id, event_id, name, url",
            attachment_item.event_id, attachment_item.name, attachment_item.url
        )
            .fetch_one(pool)
            .await?;
        
        attachments.push(rec);
    };

    Ok(attachments)
}


/// Updates multiple attachment items in the database.
///
/// # Arguments
///
/// * `data` - A vector of `Attachment` structs containing the updated attachment items.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` indicating success (`Ok(())`) or failure (`Err(sqlx::Error)`).
///
/// # Errors
///
/// Returns an error if any of the update queries fail during execution.
pub async fn update_attachments(data: Vec<Attachment>, pool: &SqlitePool) -> Result<(), sqlx::Error> {
    for attachment_item in data {
        sqlx::query_as!(
            Attachment,
            "UPDATE attachments 
             SET name = ?, url = ? 
             WHERE id = ?",
            attachment_item.name, attachment_item.url, attachment_item.id
        )
            .execute(pool)
            .await?;
    };

    Ok(())
}