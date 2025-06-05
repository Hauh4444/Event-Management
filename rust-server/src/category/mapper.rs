// External Libraries
use sqlx::SqlitePool;

// Internal Modules
use crate::category::models::Category;


/// Retrieves all categories created by a specific organizer.
///
/// # Arguments
///
/// * `data` - A struct containing the `organizer_id`.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing a list of `Categories` if found, or an `sqlx::Error` if the query fails.
///
/// # Errors
///
/// Returns an error if the query fails or no category is found.
pub async fn fetch_categories(pool: &SqlitePool) -> Result<Vec<Category>, sqlx::Error> {
    sqlx::query_as!(
        Category,
        "SELECT id, name, description FROM categories"
    )
        .fetch_all(pool)
        .await
}