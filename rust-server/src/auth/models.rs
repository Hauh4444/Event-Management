// External Libraries
use serde::{Deserialize, Serialize};


/// Represents a user in the system.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    /// Unique identifier for the user.
    pub id: i64,

    /// Username chosen by the user.
    pub username: String,

    /// Hashed password associated with the user.
    pub password: String,
}


/// Represents user information to be returned to the client.
#[derive(Serialize)]
pub struct UserData {
    /// Username chosen by the user.
    pub username: String,

    /// Name of the user organizer
    pub name: String,

    /// Logo of the user organizer
    pub logo: Option<String>,
    
    /// Website URL of the user organizer
    pub website: Option<String>,
}


/// Data required for user authentication.
#[derive(Deserialize)]
pub struct AuthData {
    /// Username for authentication.
    pub username: String,

    /// Password for authentication.
    pub password: String,
}


/// Data required to retrieve a user by username.
#[derive(Serialize, Deserialize)]
pub struct GetUserData {
    /// Username of the user to retrieve.
    pub username: String,
}


/// Data required to retrieve a user by ID.
#[derive(Deserialize)]
pub struct GetUserIDData {
    /// ID of the user to retrieve.
    pub id: i64,
}


/// Data required to update a user's password.
#[derive(Deserialize)]
pub struct UpdatePasswordRequestData {
    /// New password to set for the user.
    pub new_password: String,
}


/// Data required to update a user's password.
#[derive(Deserialize)]
pub struct UpdatePasswordData {
    /// Unique identifier of the user whose password is to be updated.
    pub user_id: i64,

    /// New password to set for the user.
    pub new_password: String,
}


/// Data required to delete a user.
#[derive(Deserialize)]
pub struct DeleteUserData {
    /// Unique identifier of the user to delete.
    pub user_id: i64,
}


/// Represents a user session in the system.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Session {
    /// Unique identifier for the session.
    pub id: i64,

    /// Unique identifier of the user associated with the session.
    pub user_id: i64,

    /// Session token used for authentication.
    pub token: String,
}


/// Data required to create a session.
#[derive(Deserialize)]
pub struct SessionData {
    /// Unique identifier of the user for whom the session is created.
    pub user_id: i64,

    /// Session token to associate with the user.
    pub token: String,
}


/// Data required to retrieve a session by token.
#[derive(Deserialize)]
pub struct GetSessionData {
    /// Session token to retrieve.
    pub token: String,
}


/// Data required to delete a session.
#[derive(Deserialize)]
pub struct DeleteSessionData {
    /// Session token of the session to delete.
    pub token: String,
}
