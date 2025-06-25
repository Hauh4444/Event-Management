# Rust Backend Server

This is the backend API server for the Event Management system, built with Rust using Actix Web and SQLite for data storage.

## Features

- REST API for managing events, attendees, and authentication
- SQLite database integration
- Built with Actix Web for performance and concurrency

## Prerequisites

- Rust (version 1.70 or higher recommended)
- Cargo (comes with Rust)
- SQLite 3.x installed

## Setup

1. Navigate to this directory.
2. Run the following command to update dependencies:
    ```bash
    cargo update
    ```
3. Build the project
    ```bash
    cargo build
    ```

## Environment Variables

Before running the server, create a `.env` file in the project root with the following content:
    ```
    FRONTEND_URL=http://localhost:5173
    DATABASE_URL=sqlite://./database.db
    ```
- `FRONTEND_URL` — The URL where your frontend application is running (used for CORS and integration).
- `DATABASE_URL` — The connection string for your SQLite database.

**Note:**
- Adjust `FRONTEND_URL` if your frontend runs on a different port or domain.
- You can use other database URLs if you switch from SQLite to another database in the future.

## Running the Server

Start the backend server:
    ```bash
    cargo run
    ```
The server will listen on port `8080` by default (configurable).

## Database Setup

Initialize or migrate the SQLite database schema:
    ```bash
    cargo run --bin migrate
    ```

## Available Commands

- `cargo run` — Run the server
- `cargo build` — Build the project
- `cargo run --bin migrate` — Run database migrations

## Troubleshooting

- Ensure Rust and Cargo are installed and up to date. 
- Verify SQLite is installed and accessible. 
- Check environment variables (e.g., `FRONTEND_URL`, `DATABASE_URL`).
- Check server logs for any errors.

## Project TODOs

### Core Features
    
- Save new image file and store location reference
- Remove old and save new image file (2 instances)
- Create Organizer (consider dedicated route)
- Fetch related events based on: category_id, speakers, etc.

## Project Structure

```
.
├── src/                  # Rust source code
│   ├── main.rs           # Application entry point
│   ├── agenda/           # Agenda module
│   ├── attachment/       # Attachment module
│   ├── attendee/         # Attendee module (routes, models, mappers)
│   ├── auth/             # Authentication module (logic, routes, services)
│   ├── category/         # Event category module
│   ├── comment/          # Comment module
│   ├── event/            # Event module (core event logic)
│   ├── faq/              # FAQ module
│   ├── organizer/        # Organizer module
│   ├── overview/         # Overview/dashboard module
│   └── speaker/          # Speaker module
├── static/               # Static files (e.g., images for the app)
├── .env                  # Environment variables
├── .gitignore            # Git ignored files and folders
├── Cargo.toml            # Rust project manifest (dependencies, metadata)
├── Cargo.lock            # Exact dependency versions (auto-generated)
├── database.db           # SQLite database file (local development)
└── README.md             # Project documentation
```