use actix_web::{web, Responder};


pub async fn register_user() -> impl Responder {
    "User registration successful"
}


pub async fn login_user() -> impl Responder {
    "User login successful"
}


pub async fn logout_user() -> impl Responder {
    "User logged out"
}


pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(register_user))
        .route("/login", web::post().to(login_user))
        .route("/logout", web::post().to(logout_user));
}
