// External Libraries
use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveDateTime};

// Internal Models
use crate::organizer::models::{Organizer};
use crate::agenda::models::{Agenda};
use crate::speaker::models::{Speaker};
use crate::faq::models::{Faq};
use crate::attachment::models::{Attachment};
use crate::comment::models::{Comment};
use crate::overview::models::CountByDate;


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

    /// Optional encoded image or image link url
    pub image: Option<String>,

    /// Optional embedded map link url
    pub map_embed: Option<String>,

    /// Optional accessibility information
    pub accessibility_info: Option<String>,

    /// Optional safety guidelines
    pub safety_guidelines: Option<String>,

    /// Timestamp for when the event was created.
    pub created_at: NaiveDateTime,

    /// Timestamp for the last update to the event.
    pub updated_at: NaiveDateTime,
}


/// Data required to create an event.
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

    /// Optional encoded image or image link url
    pub image: Option<String>,

    /// Optional embedded map link url
    pub map_embed: Option<String>,

    /// Optional accessibility information
    pub accessibility_info: Option<String>,

    /// Optional safety guidelines
    pub safety_guidelines: Option<String>,

    /// Timestamp for when the event was created.
    pub created_at: String,

    /// Timestamp for the last update to the event.
    pub updated_at: String,
}


/// Query parameters for getting overview totals.
#[derive(Deserialize)]
pub struct GetUserEventsQuery {
    /// The year to retrieve totals for (e.g., 2025).
    pub year: i64,
}


/// Data required to retrieve a user's events.
#[derive(Deserialize)]
pub struct GetUserEventsData {
    /// Identifier for the event organizer.
    pub organizer_id: i64,
    
    /// The year to retrieve totals for (e.g., 2025).
    pub year: i64,
}


/// Data required to retrieve a specific event.
#[derive(Deserialize)]
pub struct GetEventData {
    /// Unique identifier of the event to retrieve.
    pub event_id: i64,

    /// Identifier for the event organizer.
    pub organizer_id: i64,
}


/// Represents related detail information of the event.
#[derive(Deserialize, Serialize)]
pub struct EventDetails {
    /// Organizer info of the event.
    pub organizer: Organizer,

    /// List of agenda items of the event.
    pub agenda: Vec<Agenda>,

    /// List of speakers of the event.
    pub speakers: Vec<Speaker>,

    /// List of faqs of the event.
    pub faqs: Vec<Faq>,

    /// List of attachments of the event.
    pub attachments: Vec<Attachment>,
    
    /// List of comments on the event.
    pub comments: Vec<Comment>,

    /// List of related events.
    pub related_events: Vec<Event>,
}


/// Represents registerable related detail information of the event.
#[derive(Deserialize, Serialize)]
pub struct CreateEventDetails {
    /// List of agenda items of the event.
    pub agenda: Vec<Agenda>,

    /// List of speakers of the event.
    pub speakers: Vec<Speaker>,

    /// List of faqs of the event.
    pub faqs: Vec<Faq>,

    /// List of attachments of the event.
    pub attachments: Vec<Attachment>,
}


/// Represents aggregated totals for ticket metrics for a given year.
#[derive(Serialize)]
pub struct TicketTotals {
    /// Monthly totals of ticket sales.
    pub tickets: Vec<f64>,

    /// Net profit.
    pub profit: f64,
}


/// Represents aggregated daily event counts for a given year.
#[derive(Serialize)]
pub struct EventCounts {
    /// Daily totals of event counts.
    pub event_counts: Vec<CountByDate>,
}