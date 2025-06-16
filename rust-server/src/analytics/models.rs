// External Libraries
use serde::{Serialize, Deserialize};


/// Represents aggregated totals of various event-related metrics for a given year.
#[derive(Serialize)]
pub struct OverviewTotals {
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
pub struct OverviewQuery {
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
pub struct TicketsOverview {
    /// Monthly totals of ticket sales.
    pub tickets: Vec<f64>,

    /// Net profit.
    pub profit: f64,
}


/// Represents aggregated daily event counts for a given year.
#[derive(Serialize)]
pub struct EventsOverview {
    /// Daily totals of event counts.
    pub event_counts: Vec<DateCount>,
}


/// Represents the number of events on a specific date.
#[derive(Serialize)]
pub struct DateCount {
    /// Date in "YYYY-MM-DD" format.
    pub date: String,

    /// Number of events on the given date.
    pub count: usize,
}


/// Represents aggregated totals for attendee metrics for a given year.
#[derive(Serialize)]
pub struct AttendeesOverview {
    /// Monthly totals of attendees.
    pub attendees: Vec<f64>,

    /// Net profit.
    pub total: f64,
}