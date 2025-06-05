// External Libraries
use actix_web::{HttpRequest, HttpResponse};
use argon2::{password_hash, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::{RngCore};
use rand::rngs::OsRng;
use sqlx::SqlitePool;

// Internal Modules
use crate::auth::mapper::fetch_session_by_token;
use crate::auth::models::{GetSessionData, Session};


/// Validates the user session from the HTTP request cookies by checking the session token.
///
/// # Arguments
///
/// * `req` - A reference to the incoming HTTP request, from which the session cookie is extracted.
/// * `pool` - A reference to the SQLite connection pool used to query the session database.
///
/// # Returns
///
/// A `Result<Session, HttpResponse>` which is:
/// - `Ok(Session)` if the session token is found in cookies and successfully validated against the database.
/// - `Err(HttpResponse)` containing a `401 Unauthorized` response if the session token is missing or invalid.
pub async fn validate_session(
    req: &HttpRequest,
    pool: &SqlitePool,
) -> Result<Session, HttpResponse> {
    let cookie = req
        .cookie("session_token")
        .ok_or_else(|| HttpResponse::Unauthorized().body("No session token found in cookies"))?;

    let token = cookie.value().to_string();

    fetch_session_by_token(GetSessionData { token }, pool)
        .await
        .map_err(|e| HttpResponse::Unauthorized().body(format!("Session not authenticated: {}", e)))
}


/// Generates a secure, random session token encoded in URL-safe Base64 (without padding).
///
/// # Returns
///
/// A `String` containing the generated session token.
pub fn generate_session_token() -> String {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(&bytes)
}


/// Hashes a plaintext password using the Argon2 algorithm and a securely generated salt.
///
/// # Arguments
///
/// * `password` - A string slice that holds the plaintext password to be hashed.
///
/// # Returns
///
/// A `Result<String, password_hash::Error>` which contains the hashed password string on success,
/// or a password hashing error on failure.
pub fn hash_password(password: &str) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    let hash = hash.to_string();
    Ok(hash)
}


/// Verifies that a plaintext password matches a previously hashed password string.
///
/// # Arguments
///
/// * `hash` - A string slice that holds the hashed password.
/// * `password` - A string slice of the plaintext password to verify.
///
/// # Returns
///
/// A `Result<(), password_hash::Error>` indicating whether the password is valid.
/// Returns `Ok(())` if the password matches, or an error if it does not.
pub fn verify_password(hash: &str, password: &str) -> password_hash::Result<()> {
    let hash = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();
    argon2.verify_password(password.as_bytes(), &hash)
}