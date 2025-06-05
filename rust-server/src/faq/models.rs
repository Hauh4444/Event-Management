// External Libraries
use serde::{Serialize, Deserialize};


/// Represents a faq in the system.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Faq {
    /// Unique identifier for the faq.
    pub id: i64,

    /// Unique identifier of the event for the faq.
    pub event_id: i64,

    /// Question of the faq.
    pub question: String,

    /// Answer of the faq
    pub answer: Option<String>,
}


/// Data required to retrieve faq info.
#[derive(Deserialize)]
pub struct GetFaqData {
    /// Unique identifier for the event of the faq.
    pub event_id: i64,
}