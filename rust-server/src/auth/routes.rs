// External Libraries
use actix_web::{web, Responder, HttpResponse};
use sqlx::SqlitePool;

// Internal Modules
use crate::auth::mapper::{get_user_by_username, create_user, update_user_password, delete_user, create_session, delete_session, get_session_by_token};
use crate::auth::models::{AuthData, GetUserData, UpdatePasswordData, DeleteUserData, SessionData, GetSessionData, DeleteSessionData};
use crate::auth::token::generate_session_token;


/// Checks the authentication status of a user based on the provided session token.
///
/// # Arguments
///
/// * `data` - A JSON object containing the session token.
/// * `pool` - A reference to the SQLite database connection pool.
///
/// # Returns
///
/// A response indicating whether the user is authenticated or not.
pub async fn check_auth_status(
    data: web::Json<GetSessionData>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    match get_session_by_token(data.into_inner(), &pool).await {
        Ok(session) => HttpResponse::Ok().body(format!("Authenticated: {}", session.id)),
        Err(e) => HttpResponse::Unauthorized().body(format!("Unauthenticated: {}", e)),
    }
}


/// Logs in a user by verifying their credentials and creating a session.
///
/// # Arguments
///
/// * `data` - A JSON object containing the user's username and password.
/// * `pool` - A reference to the SQLite database connection pool.
///
/// # Returns
///
/// A response indicating the result of the login attempt.
pub async fn login_user(
    data: web::Json<AuthData>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let auth_data = data.into_inner();

    let user = match get_user_by_username(GetUserData{username: auth_data.username}, &pool).await {
        Ok(user) => user,
        Err(e) => return HttpResponse::Unauthorized().body(format!("User not found: {}", e)),
    };

    let token = generate_session_token();

    match create_session(SessionData{user_id: user.id, token: token.clone()}, &pool).await {
        Ok(()) => HttpResponse::Ok().body(format!("Session created: {}", token)),
        Err(e) => HttpResponse::Unauthorized().body(format!("Failed to create session: {}", e)),
    }
}


/// Registers a new user with the provided username and password.
///
/// # Arguments
///
/// * `data` - A JSON object containing the user's username and password.
/// * `pool` - A reference to the SQLite database connection pool.
///
/// # Returns
///
/// A response indicating the result of the registration attempt.
pub async fn register_user(
    data: web::Json<AuthData>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    match create_user(data.into_inner(), &pool).await {
        Ok(user) => HttpResponse::Ok().body(format!("User {} registered", user.username)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to register user: {}", e)),
    }
}


/// Logs out a user by deleting their session.
///
/// # Arguments
///
/// * `data` - A JSON object containing the session data.
/// * `pool` - A reference to the SQLite database connection pool.
///
/// # Returns
///
/// A response indicating the result of the logout attempt.
pub async fn logout_user(
    data: web::Json<DeleteSessionData>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    match delete_session(data.into_inner(), &pool).await {
        Ok(()) => HttpResponse::Ok().body("Logged out successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to log out user: {}", e)),
    }
}


/// Changes the password of an existing user.
///
/// # Arguments
///
/// * `data` - A JSON object containing the user's ID and new password.
/// * `pool` - A reference to the SQLite database connection pool.
///
/// # Returns
///
/// A response indicating the result of the password change attempt.
pub async fn change_password(
    data: web::Json<UpdatePasswordData>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    match update_user_password(data.into_inner(), &pool).await {
        Ok(()) => HttpResponse::Ok().body("Password updated"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to update password: {}", e)),
    }
}


/// Deletes a user from the system.
///
/// # Arguments
///
/// * `data` - A JSON object containing the user's ID.
/// * `pool` - A reference to the SQLite database connection pool.
///
/// # Returns
///
/// A response indicating the result of the user deletion attempt.
pub async fn remove_user(
    data: web::Json<DeleteUserData>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    match delete_user(data.into_inner(), &pool).await {
        Ok(()) => HttpResponse::Ok().body("User deleted"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to delete user: {}", e)),
    }
}


/// Configures the authentication-related routes for the application.
///
/// # Arguments
///
/// * `cfg` - A mutable reference to the service configuration.
///
/// # Returns
///
/// Configures the provided service with authentication routes.
pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/check_auth_status", web::post().to(check_auth_status))
        .route("/login", web::post().to(login_user))
        .route("/register", web::post().to(register_user))
        .route("/logout", web::post().to(logout_user))
        .route("/update_password", web::put().to(change_password))
        .route("/delete_user", web::delete().to(remove_user));
}
