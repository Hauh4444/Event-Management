// External Libraries
use serde::{Serialize, Deserialize};


/// Represents an event in the system.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Event {
    /// Unique identifier for the event.
    pub id: i64,

    /// Title of the event.
    pub title: String,

    /// Optional description of the event.
    pub description: Option<String>,

    /// Start time of the event in string format.
    pub start_time: String,

    /// Optional end time of the event in string format.
    pub end_time: Option<String>,

    /// Optional location where the event is held.
    pub location: Option<String>,

    /// Optional identifier for the associated category.
    pub category_id: Option<i64>,

    /// Flag indicating whether the event is active.
    pub is_active: Option<i64>,

    /// Optional identifier for the event organizer.
    pub organizer_id: Option<i64>,

    /// Optional price to attend the event.
    pub price: Option<f64>,

    /// Optional maximum number of attendees allowed.
    pub max_attendees: Option<i64>,

    /// Optional email contact for the event.
    pub contact_email: Option<String>,

    /// Optional phone contact for the event.
    pub contact_phone: Option<String>,

    /// Optional deadline for event registration.
    pub registration_deadline: Option<String>,

    /// Flag indicating whether the event is virtual.
    pub is_virtual: Option<i64>,

    /// Optional timestamp for when the event was created.
    pub created_at: Option<String>,

    /// Optional timestamp for the last update to the event.
    pub updated_at: Option<String>,
}


/// Data required to create or update an event.
#[derive(Deserialize)]
pub struct EventData {
    /// Title of the event.
    pub title: String,

    /// Optional description of the event.
    pub description: Option<String>,

    /// Start time of the event in string format.
    pub start_time: String,

    /// Optional end time of the event in string format.
    pub end_time: Option<String>,

    /// Optional location where the event is held.
    pub location: Option<String>,

    /// Optional identifier for the associated category.
    pub category_id: Option<i64>,

    /// Flag indicating whether the event is active.
    pub is_active: Option<i64>,

    /// Optional identifier for the event organizer.
    pub organizer_id: Option<i64>,

    /// Optional price to attend the event.
    pub price: Option<f64>,

    /// Optional maximum number of attendees allowed.
    pub max_attendees: Option<i64>,

    /// Optional email contact for the event.
    pub contact_email: Option<String>,

    /// Optional phone contact for the event.
    pub contact_phone: Option<String>,

    /// Optional deadline for event registration.
    pub registration_deadline: Option<String>,

    /// Flag indicating whether the event is virtual.
    pub is_virtual: Option<i64>,

    /// Optional timestamp for when the event was created.
    pub created_at: Option<String>,

    /// Optional timestamp for the last update to the event.
    pub updated_at: Option<String>,
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
