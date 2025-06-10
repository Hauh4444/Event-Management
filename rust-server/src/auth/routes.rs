// External Libraries
use actix_web::{web, Responder, HttpRequest, HttpResponse, cookie};
use cookie::Cookie;
use sqlx::SqlitePool;
use time::Duration;

// Internal Mappers
use crate::auth::mapper::{fetch_user_by_username, fetch_user_by_id, create_user, update_user_password, delete_user, create_session, delete_session};
use crate::organizer::mapper::{delete_organizer, fetch_organizer};

// Internal Models
use crate::auth::models::{UserData, AuthData, GetUserData, GetUserIDData, UpdatePasswordRequestData, UpdatePasswordData, DeleteUserData, SessionData, DeleteSessionData};
use crate::organizer::models::{DeleteOrganizerData, GetOrganizerData, Organizer};

// Internal Services
use crate::auth::services::{generate_session_token, hash_password, validate_session, verify_password};


/// Checks the authentication status of a user based on the provided session token.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `pool` - A reference to the SQLite database connection pool.
///
/// # Returns
///
/// A response indicating whether the user is authenticated or not.
pub async fn check_auth_status(
    req: HttpRequest,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };

    let organizer_info = fetch_organizer(GetOrganizerData { organizer_id: session.user_id }, &pool)
        .await.unwrap_or_else(|_| Organizer::default());
    
    match fetch_user_by_id(GetUserIDData {id: session.user_id}, &pool).await {
        Ok(user) => HttpResponse::Ok().json(UserData {
            username: user.username,
            name: organizer_info.name,
            logo: organizer_info.logo,
            website: organizer_info.website,
        }),
        Err(e) => HttpResponse::Unauthorized().body(format!("User not found: {}", e)),
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

    let user = match fetch_user_by_username(GetUserData {username: auth_data.username}, &pool).await {
        Ok(user) => user,
        Err(e) => return HttpResponse::Unauthorized().body(format!("Username not found: {}", e)),
    };
    
    match verify_password(&user.password, &auth_data.password) {
        Err(e) => return HttpResponse::Unauthorized().body(format!("Invalid password: {}", e)),
        _ => {},
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
    
    match create_user(AuthData {username: data.username.clone(), password}, &pool).await {
        Ok(user) => HttpResponse::Ok().body(format!("User {} registered", user.username)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to register user: {}", e)),
    }
    
    // TODO Create organizer (possibly separate route)
}


/// Logs out a user by deleting their session.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `pool` - A reference to the SQLite database connection pool.
///
/// # Returns
///
/// A response indicating the result of the logout attempt.
pub async fn logout_user(
    req: HttpRequest,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };
    
    let expired_cookie = Cookie::build("session_token", "")
        .path("/")
        .http_only(true)
        .same_site(cookie::SameSite::None)
        .secure(true)
        .expires(time::OffsetDateTime::now_utc() - Duration::days(1))
        .finish();

    match delete_session(DeleteSessionData {token: session.token}, &pool).await {
        Ok(()) => HttpResponse::Ok().cookie(expired_cookie).body("Logged out successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to log out user: {}", e)),
    }
}


/// Changes the password of an existing user.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `data` - A JSON object containing the user's ID and new password.
/// * `pool` - A reference to the SQLite database connection pool.
///
/// # Returns
///
/// A response indicating the result of the password change attempt.
pub async fn change_password(
    req: HttpRequest,
    data: web::Json<UpdatePasswordRequestData>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };
    
    let new_password = match hash_password(&data.new_password) {
        Ok(new_password) => new_password,
        Err(e) => return HttpResponse::Unauthorized().body(format!("Error hashing password: {}", e)),
    };
    
    match update_user_password(UpdatePasswordData {user_id: session.user_id, new_password}, &pool).await {
        Ok(()) => HttpResponse::Ok().body("Password updated"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to update password: {}", e)),
    }
}


/// Deletes a user from the system.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request containing session data.
/// * `pool` - A reference to the SQLite database connection pool.
///
/// # Returns
///
/// A response indicating the result of the user deletion attempt.
pub async fn remove_user(
    req: HttpRequest,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let session = match validate_session(&req, &pool).await {
        Ok(session) => session,
        Err(response) => return response,
    };
    
    match delete_user(DeleteUserData {user_id: session.user_id}, &pool).await {
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to delete user: {}", e)),
        _ => {},
    };
    
    match delete_organizer(DeleteOrganizerData {organizer_id: session.user_id}, &pool).await {
        Ok(()) => HttpResponse::Ok().body("User deleted"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to delete organizer: {}", e)),
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
