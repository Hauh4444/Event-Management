// External Libraries
use serde::{Deserialize, Serialize};


/// Represents a category in the system.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    /// Unique identifier for the category.
    pub id: i64,

    /// Name of the category.
    pub name: String,

    /// Description of the category.
    pub description: String,
}