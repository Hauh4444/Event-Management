// External Libraries
use actix_web::{web, Responder, HttpRequest, HttpResponse, cookie};
use cookie::Cookie;
use sqlx::SqlitePool;
use time::Duration;

// Internal Modules
use crate::auth::mapper::{get_user_by_username, get_user_by_id, create_user, update_user_password, delete_user, create_session, delete_session, get_session_by_token};
use crate::auth::models::{AuthData, GetUserData, GetUserIDData, UpdatePasswordData, DeleteUserData, SessionData, GetSessionData, DeleteSessionData};
use crate::auth::services::{generate_session_token, hash_password, verify_password};


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
    req: HttpRequest,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    if let Some(cookie) = req.cookie("session_token") {
        let token = cookie.value().to_string();

        let session = match get_session_by_token(GetSessionData {token}, &pool).await {
            Ok(session) => session,
            Err(e) => return HttpResponse::Unauthorized().body(format!("Session not authenticated: {}", e)),
        };

        match get_user_by_id(GetUserIDData {id: session.user_id}, &pool).await {
            Ok(user) => HttpResponse::Ok().json(GetUserData {username: user.username}),
            Err(e) => HttpResponse::Unauthorized().body(format!("User not found: {}", e)),
        }
    } else {
        HttpResponse::Unauthorized().body("No session token found in cookies")
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

    let user = match get_user_by_username(GetUserData {username: auth_data.username}, &pool).await {
        Ok(user) => user,
        Err(e) => return HttpResponse::Unauthorized().body(format!("Username not found: {}", e)),
    };
    
    match verify_password(&user.password, &auth_data.password) {
        Ok(()) => (),
        Err(e) => return HttpResponse::Unauthorized().body(format!("Invalid password: {}", e)),
    }

    let token = generate_session_token();

    let cookie = Cookie::build("session_token", token.clone())
        .path("/")
        .http_only(true)
        .same_site(cookie::SameSite::None)
        .secure(true)
        .finish();

    let mut response = HttpResponse::Ok();
    response.cookie(cookie);

    match create_session(SessionData {user_id: user.id, token: token.clone()}, &pool).await {
        Ok(()) => response.body(format!("Session created: {}", token)),
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
    let password = match hash_password(&data.password) {
        Ok(password) => password,
        Err(e) => return HttpResponse::Unauthorized().body(format!("Error hashing password: {}", e)),
    };
    
    match create_user(AuthData {username: data.username.clone(), password }, &pool).await {
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
    req: HttpRequest,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    if let Some(cookie) = req.cookie("session_token") {
        let token = cookie.value().to_string();
        
        let expired_cookie = Cookie::build("session_token", "")
            .path("/")
            .http_only(true)
            .same_site(cookie::SameSite::None)
            .secure(true)
            .expires(time::OffsetDateTime::now_utc() - Duration::days(1))
            .finish();

        match delete_session(DeleteSessionData { token }, &pool).await {
            Ok(()) => HttpResponse::Ok().cookie(expired_cookie).body("Logged out successfully"),
            Err(e) => HttpResponse::InternalServerError().body(format!("Failed to log out user: {}", e)),
        }
    } else {
        HttpResponse::Unauthorized().body("No session token found in cookies")
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
    req: HttpRequest,
    data: web::Json<UpdatePasswordData>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    if let Some(cookie) = req.cookie("session_token") {
        // TODO check session token matches user ids
        let token = cookie.value().to_string();

        let new_password = match hash_password(&data.new_password) {
            Ok(new_password) => new_password,
            Err(e) => return HttpResponse::Unauthorized().body(format!("Error hashing password: {}", e)),
        };
        
        match update_user_password(UpdatePasswordData {user_id: data.user_id.clone(), new_password}, &pool).await {
            Ok(()) => HttpResponse::Ok().body("Password updated"),
            Err(e) => HttpResponse::InternalServerError().body(format!("Failed to update password: {}", e)),
        }
    } else {
        HttpResponse::Unauthorized().body("No session token found in cookies")
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
        .route("/check_auth_status/", web::get().to(check_auth_status))
        .route("/login/", web::post().to(login_user))
        .route("/register/", web::post().to(register_user))
        .route("/logout/", web::post().to(logout_user))
        .route("/update_password/", web::put().to(change_password))
        .route("/delete_user/", web::delete().to(remove_user));
}
