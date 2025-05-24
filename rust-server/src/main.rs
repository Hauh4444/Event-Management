// External Libraries
use actix_cors::Cors;
use actix_web::{App, HttpServer, web, http::header};
use sqlx::sqlite::SqlitePoolOptions;
use dotenv::dotenv;
use std::env;

// Internal Modules
mod analytics;
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

    // Start the Actix-web HTTP server
    HttpServer::new(move || {
        // Configure CORS middleware to allow requests from the frontend URL
        // Supports credentials (cookies, authorization headers)
        // Allows common HTTP methods and headers necessary for JSON APIs
        let cors = Cors::default()
            .allowed_origin(&env::var("FRONTEND_URL").expect("FRONTEND_URL must be set"))
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
            .supports_credentials()
            .max_age(3600);

        App::new()
            // Apply the CORS middleware
            .wrap(cors)
            // Share the database connection pool with the application
            .app_data(web::Data::new(pool.clone()))
            // Configure analytics routes
            .configure(analytics::routes::configure_analytics_routes)
            // Configure authentication routes
            .configure(auth::routes::configure_auth_routes)
            // Configure event routes
            .configure(event::routes::configure_event_routes)
    })
        // Bind to localhost on port 8080
        .bind("127.0.0.1:8080")?
        // Run the server asynchronously
        .run()
        .await 
}
