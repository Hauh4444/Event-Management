// External Libraries
use actix_web::{App, HttpServer, web};
use sqlx::sqlite::SqlitePoolOptions;
use dotenv::dotenv;
use std::env;

// Internal Modules
mod auth;
mod event;


/// Initializes the application, sets up the database connection pool,
/// and configures the HTTP server with authentication and event routes.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from the .env file
    dotenv().ok();

    // Retrieve the database URL from the environment
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the .env file");

    // Create a connection pool for SQLite
    let pool = SqlitePoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            // Share the database connection pool with the application
            .app_data(web::Data::new(pool.clone()))
            // Configure authentication routes
            .configure(auth::routes::configure_auth_routes)
            // Configure event routes
            .configure(event::routes::configure_event_routes)
    })
        .bind("127.0.0.1:8080")? // Bind to localhost on port 8080
        .run()
        .await // Run the server asynchronously
}
