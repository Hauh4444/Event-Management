// External Libraries
use serde::{Serialize, Deserialize};


/// Represents an attachment in the system.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Attachment {
    /// Unique identifier for the attachment.
    pub id: i64,

    /// Unique identifier of the event for the attachment.
    pub event_id: i64,

    /// Name of the attachment.
    pub name: String,

    /// Url of the attachment
    pub url: String,
}


/// Data required to retrieve attachment info.
#[derive(Deserialize)]
pub struct GetAttachmentData {
    /// Unique identifier for the event of the attachment.
    pub event_id: i64,
}