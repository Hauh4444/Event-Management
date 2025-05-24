// External Libraries
use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveDateTime};


/// Represents an event in the system.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Event {
    /// Unique identifier for the event.
    pub id: i64,

    /// Title of the event.
    pub title: String,

    /// Description of the event.
    pub description: String,

    /// The date of the event (required for yearly/monthly queries).
    pub event_date: NaiveDate,

    /// Start time of the event in string format.
    pub start_time: String,

    /// End time of the event in string format.
    pub end_time: String,

    /// Location where the event is held.
    pub location: String,

    /// Identifier for the associated category.
    pub category_id: i64,

    /// Status of the event ("upcoming", "canceled", etc.).
    pub status: String,

    /// Identifier for the event organizer.
    pub organizer_id: i64,

    /// Price to attend the event.
    pub price: f64,

    /// Number of tickets sold for the event.
    pub tickets_sold: i64,

    /// Number of attendees for the event.
    pub attendees: i64,

    /// Maximum number of attendees allowed.
    pub max_attendees: i64,

    /// Email contact for the event.
    pub contact_email: String,

    /// Phone contact for the event.
    pub contact_phone: String,

    /// Deadline for event registration.
    pub registration_deadline: NaiveDate,

    /// Flag indicating whether the event is virtual.
    pub is_virtual: i64,

    /// Timestamp for when the event was created.
    pub created_at: NaiveDateTime,

    /// Timestamp for the last update to the event.
    pub updated_at: NaiveDateTime,
}



/// Data required to create or update an event.
#[derive(Deserialize)]
pub struct EventData {
    /// Title of the event.
    pub title: String,

    /// Description of the event.
    pub description: String,

    /// The date of the event (required for yearly/monthly queries).
    pub event_date: NaiveDate,

    /// Start time of the event in string format.
    pub start_time: String,

    /// End time of the event in string format.
    pub end_time: String,

    /// Location where the event is held.
    pub location: String,

    /// Identifier for the associated category.
    pub category_id: i64,

    /// Status of the event ("upcoming", "canceled", etc.).
    pub status: String,

    /// Identifier for the event organizer.
    pub organizer_id: i64,

    /// Price to attend the event.
    pub price: f64,

    /// Number of tickets sold for the event.
    pub tickets_sold: i64,

    /// Number of attendees for the event.
    pub attendees: i64,

    /// Maximum number of attendees allowed.
    pub max_attendees: i64,

    /// Email contact for the event.
    pub contact_email: String,

    /// Phone contact for the event.
    pub contact_phone: String,

    /// Deadline for event registration.
    pub registration_deadline: String,

    /// Flag indicating whether the event is virtual.
    pub is_virtual: i64,

    /// Timestamp for when the event was created.
    pub created_at: String,

    /// Timestamp for the last update to the event.
    pub updated_at: String,
}


/// Data required to retrieve a specific event.
#[derive(Deserialize)]
pub struct GetEventData {
    /// Unique identifier of the event to retrieve.
    pub event_id: i64,
}


/// Data required to delete an event.
#[derive(Deserialize)]
pub struct DeleteEventData {
    /// Unique identifier of the event to delete.
    pub event_id: i64,
}
