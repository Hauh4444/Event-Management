// External Libraries
use sqlx::SqlitePool;

// Internal Modules
use crate::agenda::models::{Agenda, GetAgendaData};


/// Retrieves agenda items by their event ID.
///
/// # Arguments
///
/// * `data` - A struct containing the `event_id`.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing a list of `Agendas` if found, or an `sqlx::Error` if the query fails.
///
/// # Errors
///
/// Returns an error if the query fails or no agenda is found.
pub async fn fetch_agenda(data: GetAgendaData, pool: &SqlitePool) -> Result<Vec<Agenda>, sqlx::Error> {
    let event_id = data.event_id;

    sqlx::query_as!(
        Agenda,
        "SELECT id, event_id, start_time, title, speaker
         FROM agendas
         WHERE event_id = ?",
        event_id
    )
        .fetch_all(pool)
        .await
}


/// Creates multiple agenda items in the database.
///
/// # Arguments
///
/// * `data` - A vector of `Agenda` structs containing the new agenda items.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` indicating success (`Ok(())`) or failure (`Err(sqlx::Error)`).
///
/// # Errors
///
/// Returns an error if any of the creation queries fail during execution.
pub async fn create_agenda(data: Vec<Agenda>, pool: &SqlitePool) -> Result<Vec<Agenda>, sqlx::Error> {
    let mut agendas = Vec::new();
    
    for agenda_item in data {
        let res = sqlx::query_as!(
            Agenda,
            "INSERT INTO agendas (event_id, start_time, title, speaker) 
             VALUES (?, ?, ?, ?)
             RETURNING id, event_id, start_time, title, speaker",
            agenda_item.event_id, agenda_item.start_time, agenda_item.title, agenda_item.speaker
        )
            .fetch_one(pool)
            .await?;
        
        agendas.push(res);
    };

    Ok(agendas)
}


/// Updates multiple agenda items in the database.
///
/// # Arguments
///
/// * `data` - A vector of `Agenda` structs containing the updated agenda items.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` indicating success (`Ok(())`) or failure (`Err(sqlx::Error)`).
///
/// # Errors
///
/// Returns an error if any of the update queries fail during execution.
pub async fn update_agenda(data: Vec<Agenda>, pool: &SqlitePool) -> Result<(), sqlx::Error> {
    for agenda_item in data {
        sqlx::query_as!(
            Agenda,
            "UPDATE agendas 
             SET start_time = ?, title = ?, speaker = ? 
             WHERE id = ?",
            agenda_item.start_time, agenda_item.title, agenda_item.speaker, agenda_item.id
        )
            .execute(pool)
            .await?;
    };
    
    Ok(())
}