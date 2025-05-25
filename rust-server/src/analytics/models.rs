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


/// Query parameters for getting overview totals.
#[derive(Deserialize)]
pub struct GetOverviewTotals {
    /// The year to retrieve totals for (e.g., 2025).
    pub year: i64,
    
    /// Identifier for the event organizer.
    pub organizer_id: i64,
}


#[derive(Serialize)]
pub struct TicketsOverview {
    /// Monthly totals of ticket sales.
    pub tickets: Vec<f64>,

    /// Net profit.
    pub profit: f64,
}


#[derive(Deserialize)]
pub struct GetTicketsOverview {
    /// The year to retrieve totals for (e.g., 2025).
    pub year: i64,

    /// Identifier for the event organizer.
    pub organizer_id: i64,
}