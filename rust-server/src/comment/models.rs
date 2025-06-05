// External Libraries
use serde::{Serialize, Deserialize};


/// Represents a comment in the system.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Comment {
    /// Unique identifier for the comment.
    pub id: i64,

    /// Unique identifier of the event for the comment.
    pub event_id: i64,

    /// Message of the comment.
    pub message: String,
}


/// Data required to retrieve a comment info.
#[derive(Deserialize)]
pub struct GetCommentData {
    /// Unique identifier for the event of the comment.
    pub event_id: i64,
}