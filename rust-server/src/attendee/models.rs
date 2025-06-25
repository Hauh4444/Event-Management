// External Libraries
use serde::{Serialize, Deserialize};
use chrono::{NaiveDate};
use crate::event::models::Event;
use crate::overview::models::CountByDate;

/// Represents an attendee in the system.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Attendee {
    /// Unique identifier for the attendee
    pub id: i64,

    /// Unique identifier of the event that the attendee attended
    pub event_id: i64,

    /// Name of the attendee
    pub name: String,

    /// Email of the attendee
    pub email: String,

    /// Ticket type that was purchased
    pub ticket_type: String,
    
    /// Attendee registration date
    pub registration_date: NaiveDate,
}


/// Data required to retrieve an event's attendees.
#[derive(Deserialize)]
pub struct GetAttendeeData {
    /// Unique identifier of the event to retrieve attendees for.
    pub event_id: i64,
}


/// Represents aggregated totals for attendee metrics for a given year.
#[derive(Serialize)]
pub struct AttendeeTotals {
    /// Monthly totals of attendees.
    pub attendees: Vec<i64>,

    /// Total attendees.
    pub total: i64,
}


/// Represents the most and least attended completed events for a given year.
#[derive(Serialize)]
pub struct AttendanceExtremes {
    /// List of the most attended events.
    pub most: Vec<Event>,

    /// List of the least attended events.
    pub least: Vec<Event>,
}


/// Represents aggregated daily attendee counts for a given year.
#[derive(Serialize)]
pub struct AttendeeCounts {
    /// Daily totals of attendee counts.
    pub attendee_counts: Vec<CountByDate>,
}


/// Represents aggregated totals for no show metrics for a given year.
#[derive(Serialize)]
pub struct NoShowTotals {
    /// Monthly totals of no shows.
    pub no_show_counts: Vec<i64>,

    /// Monthly rates of no shows.
    pub no_show_rates: Vec<f64>,

    /// Total count of no shows.
    pub total_count: i64,
    
    /// Total rate of no shows.
    pub total_rate: f64,
}


/// Represents aggregated totals for attendee metrics by ticket type for a given year.
#[derive(Serialize)]
pub struct TicketTypeTotals {
    /// Monthly totals of general ticket attendees.
    pub general_counts: Vec<i64>,

    /// Monthly totals of student ticket attendees.
    pub student_counts: Vec<i64>,

    /// Monthly totals of staff ticket attendees.
    pub staff_counts: Vec<i64>,

    /// Monthly totals of vip ticket attendees.
    pub vip_counts: Vec<i64>,
}