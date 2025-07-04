// External Libraries
use sqlx::SqlitePool;

// Internal Models
use crate::faq::models::{Faq, GetFaqData};


/// Retrieves faq items by their event ID.
///
/// # Arguments
///
/// * `data` - A struct containing the `event_id`.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing a list of `Faqs` if found, or an `sqlx::Error` if the query fails.
///
/// # Errors
///
/// Returns an error if the query fails or no faq is found.
pub async fn fetch_faqs(
    data: GetFaqData, 
    pool: &SqlitePool
) -> Result<Vec<Faq>, sqlx::Error> {
    let event_id = data.event_id;

    sqlx::query_as!(
        Faq,
        "SELECT id, event_id, question, answer
         FROM faqs
         WHERE event_id = ?",
        event_id
    )
        .fetch_all(pool)
        .await
}


/// Creates multiple faq items in the database.
///
/// # Arguments
///
/// * `data` - A vector of `Faq` structs containing the created faq items.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` indicating success (`Ok(())`) or failure (`Err(sqlx::Error)`).
///
/// # Errors
///
/// Returns an error if any of the creation queries fail during execution.
pub async fn create_faqs(
    data: Vec<Faq>, 
    pool: &SqlitePool
) -> Result<Vec<Faq>, sqlx::Error> {
    let mut faqs = Vec::new();
    
    for faq_item in data {
        let rec = sqlx::query_as!(
            Faq,
            "INSERT INTO faqs (event_id, question, answer) 
             VALUES (?, ?, ?)
             RETURNING id, event_id, question, answer",
            faq_item.event_id, faq_item.question, faq_item.answer
        )
            .fetch_one(pool)
            .await?;
        
        faqs.push(rec);
    };

    Ok(faqs)
}


/// Updates multiple faq items in the database.
///
/// # Arguments
///
/// * `data` - A vector of `Faq` structs containing the updated faq items.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` indicating success (`Ok(())`) or failure (`Err(sqlx::Error)`).
///
/// # Errors
///
/// Returns an error if any of the update queries fail during execution.
pub async fn update_faqs(
    data: Vec<Faq>, 
    pool: &SqlitePool
) -> Result<(), sqlx::Error> {
    for faq_item in data {
        sqlx::query_as!(
            Faq,
            "UPDATE faqs 
             SET question = ?, answer = ?
             WHERE id = ?",
            faq_item.question, faq_item.answer, faq_item.id
        )
            .execute(pool)
            .await?;
    };

    Ok(())
}