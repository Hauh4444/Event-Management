// External Libraries
use serde::{Serialize, Deserialize};


/// Represents an organizer in the system.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Organizer {
    /// Unique identifier for the organizer.
    pub id: i64,

    /// Name of the organizer.
    pub name: String,

    /// Logo of the organizer.
    pub logo: Option<String>,

    /// Website of the organizer
    pub website: Option<String>,
}


/// Represents a default organizer.
impl Default for Organizer {
    fn default() -> Self {
        Organizer {
            id: 0,
            name: String::new(),
            logo: None,
            website: None,
        }
    }
}


/// Data required to retrieve a organizer info.
#[derive(Deserialize)]
pub struct GetOrganizerData {
    /// Unique identifier for the organizer.
    pub organizer_id: i64,
}