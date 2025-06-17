// External Libraries
use serde::{Serialize, Deserialize};

// Internal Models
use crate::event::models::Event;


/// Represents aggregated totals of various event-related metrics for a given year.
#[derive(Serialize)]
pub struct MonthlyTotals {
    /// Monthly totals of all events.
    pub events: Vec<i64>,

    /// Monthly totals of upcoming events.
    pub upcoming: Vec<i64>,

    /// Monthly totals of canceled events.
    pub canceled: Vec<i64>,

    /// Monthly totals of ticket sales.
    pub tickets: Vec<i64>,

    /// Monthly totals of attendees.
    pub attendees: Vec<i64>,
}


/// Query parameters for requesting overview totals.
#[derive(Deserialize)]
pub struct YearQuery {
    /// The year to retrieve totals for (e.g., 2025).
    pub year: i64,
}


/// Data parameters for getting overview totals.
#[derive(Deserialize)]
pub struct GetOverview {
    /// The year to retrieve totals for (e.g., 2025).
    pub year: i64,
    
    /// Identifier for the event organizer.
    pub organizer_id: i64,
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


/// Represents the number of events on a specific date.
#[derive(Serialize)]
pub struct CountByDate {
    /// Date in "YYYY-MM-DD" format.
    pub date: String,

    /// Number of events on the given date.
    pub count: usize,
}


/// Represents aggregated totals for attendee metrics for a given year.
#[derive(Serialize)]
pub struct AttendeeTotals {
    /// Monthly totals of attendees.
    pub attendees: Vec<f64>,

    /// Net profit.
    pub total: f64,
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