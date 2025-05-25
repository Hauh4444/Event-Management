// External Libraries
use actix_cors::Cors;
use actix_web::{App, HttpServer, web, http::header, middleware::Logger};
use sqlx::sqlite::SqlitePoolOptions;
use dotenv::dotenv;
use std::env;
use env_logger::Env;

// Internal Modules
mod analytics;
mod auth;
mod event;


/// Initializes the application, sets up the database connection pool,
/// and configures the HTTP server with routes.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from the .env file
    dotenv().ok();

    // Initialize logging
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Retrieve the database URL from the environment
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the .env file");

    // Create a connection pool for SQLite
    let pool = SqlitePoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Start the Actix-web HTTP server
    HttpServer::new(move || {
        // Configure CORS middleware
        let cors = Cors::default()
            .allowed_origin(&env::var("FRONTEND_URL").expect("FRONTEND_URL must be set"))
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(Logger::new(r#"%a "%r" %s"#))
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .configure(analytics::routes::configure_analytics_routes)
            .configure(auth::routes::configure_auth_routes)
            .configure(event::routes::configure_event_routes)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
