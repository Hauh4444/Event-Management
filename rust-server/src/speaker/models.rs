// External Libraries
use serde::{Serialize, Deserialize};


/// Represents a speaker in the system.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Speaker {
    /// Unique identifier for the speaker.
    pub id: i64,

    /// Unique identifier of the event for the speaker.
    pub event_id: i64,

    /// Name of the speaker.
    pub name: String,

    /// Bio of the speaker
    pub bio: Option<String>,

    /// Photo of the speaker
    pub photo: Option<String>,
}


/// Data required to retrieve speaker info.
#[derive(Deserialize)]
pub struct GetSpeakerData {
    /// Unique identifier for the event of the speaker.
    pub event_id: i64,
}