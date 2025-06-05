// External Libraries
use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime};


/// Represents an agenda in the system.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Agenda {
    /// Unique identifier for the agenda.
    pub id: i64,

    /// Unique identifier of the event for the agenda.
    pub event_id: i64,

    /// Time of the agenda.
    pub start_time: NaiveDateTime,

    /// Title of the agenda
    pub title: String,

    /// Speaker of the agenda
    pub speaker: String,
}


/// Data required to retrieve a agenda info.
#[derive(Deserialize)]
pub struct GetAgendaData {
    /// Unique identifier for the event of the agenda.
    pub event_id: i64,
}