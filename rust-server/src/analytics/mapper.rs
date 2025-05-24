// External Libraries
use sqlx::SqlitePool;
use chrono::Datelike;

// Internal Modules
use crate::analytics::models::{GetOverviewTotals, OverviewTotals};
use crate::event::models::{Event};


pub async fn fetch_overview_totals(data: GetOverviewTotals, pool: &SqlitePool) -> Result<OverviewTotals, sqlx::Error> {
    let year = data.year.to_string();
    let organizer_id = data.organizer_id;

    let events = sqlx::query_as!(
        Event,
        "SELECT * FROM events WHERE strftime('%Y', event_date) = ? AND organizer_id = ?",
        year, organizer_id
    )
        .fetch_all(pool)
        .await?;

    let mut events_by_month = vec![0i64; 12];
    let mut upcoming_by_month = vec![0i64; 12];
    let mut canceled_by_month = vec![0i64; 12];
    let mut tickets_by_month = vec![0i64; 12];
    let mut attendees_by_month = vec![0i64; 12];

    for event in events {
        let month = event.event_date.month() as usize - 1;

        events_by_month[month] += 1;
        if event.status == "upcoming" {
            upcoming_by_month[month] += 1;
        }
        if event.status == "canceled" {
            canceled_by_month[month] += 1;
        }

        tickets_by_month[month] += event.tickets_sold;
        attendees_by_month[month] += event.attendees;
    }

    Ok(OverviewTotals {
        events: events_by_month,
        upcoming: upcoming_by_month,
        canceled: canceled_by_month,
        tickets: tickets_by_month,
        attendees: attendees_by_month,
    })
}