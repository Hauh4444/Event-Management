// External Libraries
use sqlx::SqlitePool;

// Internal Modules
use crate::auth::models::{User, AuthData, GetUserData, UpdatePasswordData, DeleteUserData, Session, SessionData, GetSessionData, DeleteSessionData};


/// Fetches a user from the database by their username.
///
/// # Arguments
///
/// * `data` - A struct containing the username of the user to fetch.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing either the `User` struct if found, or an error if the query fails.
pub async fn get_user_by_username(data: GetUserData, pool: &SqlitePool) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT id, username, password FROM users WHERE username = ?",
        data.username
    )
        .fetch_one(pool)
        .await
}


/// Creates a new user in the database.
///
/// # Arguments
///
/// * `data` - A struct containing the username and password of the user to create.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing the `User` struct representing the newly created user, or an error if the query fails.
pub async fn create_user(data: AuthData, pool: &SqlitePool) -> Result<User, sqlx::Error> {
    let rec = sqlx::query_as!(
        User,
        "INSERT INTO users (username, password) VALUES (?, ?) RETURNING *",
        data.username,
        data.password 
    )
        .fetch_one(pool)
        .await?;

    Ok(rec)
}


/// Updates the password for an existing user.
///
/// # Arguments
///
/// * `data` - A struct containing the user ID and the new password to update.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` indicating success or failure of the password update.
pub async fn update_user_password(data: UpdatePasswordData, pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE users SET password = ? WHERE id = ?",
        data.new_password,
        data.user_id
    )
        .execute(pool)
        .await?;

    Ok(())
}


/// Deletes a user from the database.
///
/// # Arguments
///
/// * `data` - A struct containing the user ID of the user to delete.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` indicating success or failure of the user deletion.
pub async fn delete_user(data: DeleteUserData, pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM users WHERE id = ?",
        data.user_id
    )
        .execute(pool)
        .await?;

    Ok(())
}


/// Fetches a session from the database based on the provided session token.
///
/// # Arguments
///
/// * `data` - A struct containing the token of the session to fetch.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing the `Session` struct if found, or an error if the query fails.
pub async fn get_session_by_token(data: GetSessionData, pool: &SqlitePool) -> Result<Session, sqlx::Error> {
    sqlx::query_as!(
        Session,
        "SELECT * FROM sessions WHERE token = ?",
        data.token
    )
        .fetch_one(pool)
        .await
}


/// Creates a new session for a user.
///
/// # Arguments
///
/// * `data` - A struct containing the user ID and session token.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` containing the `Session` struct for the newly created session, or an error if the query fails.
pub async fn create_session(data: SessionData, pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query_as!(
        Session,
        "INSERT INTO sessions (user_id, token) VALUES (?, ?) RETURNING *;",
        data.user_id, data.token
    )
        .fetch_one(pool)
        .await?;

    Ok(())
}


/// Deletes a session from the database based on the provided session token.
///
/// # Arguments
///
/// * `data` - A struct containing the token of the session to delete.
/// * `pool` - A reference to the SQLite connection pool.
///
/// # Returns
///
/// A `Result` indicating success or failure of the session deletion.
pub async fn delete_session(data: DeleteSessionData, pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM sessions WHERE token = ?",
        data.token
    )
        .execute(pool)
        .await?;

    Ok(())
}

