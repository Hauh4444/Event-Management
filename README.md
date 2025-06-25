# Event Management System

A full-stack platform for managing events, built with React (frontend) and Rust (backend). Easily create and manage events, view attendee lists, and track event details through an intuitive dashboard.

> **Status:** In Progress  
> Features and design are actively evolving.

## Features

- User authentication and secure login
- Event creation, editing, and management
- Attendee tracking and management
- Interactive dashboards for events and attendees
- Responsive UI with Material UI components
- RESTful API built with Actix Web and SQLite

## Tech Stack

- **Frontend:** React, Material UI, CSS
- **Backend:** Rust (Actix Web)
- **Database:** SQLite

## Screenshots

Click to view:
- [Auth Page](./screenshots/auth-page.png)
- [Dashboard Page](./screenshots/dashboard-page.png)
- [Events Page](./screenshots/events-page.png)
- [Attendees Page](./screenshots/attendees-page.png)

## Prerequisites

- **Node.js** v16 or higher
- **Rust** 1.70+
- **SQLite** 3.x

## Setup

### Frontend

1. Navigate to the `react-frontend` directory:
    ```bash
    cd react-frontend
    ```
2. Install dependencies:
    ```bash
    npm install
    ```

### Backend

1. Navigate to the `rust-server` directory:
    ```bash
    cd rust-server
    ```
2. Update dependencies:
    ```bash
    cargo update
    ```
3. Build the server:
    ```bash
    cargo build
    ```
    
## Environment Variables

Before running the backend, create a `.env` file in `rust-server/` with:
```
FRONTEND_URL=http://localhost:5173
DATABASE_URL=sqlite://./database.db
```
- `FRONTEND_URL` — The URL where your frontend application is running (used for CORS and integration).
- `DATABASE_URL` — The connection string for your SQLite database.

**Note:**
- Adjust `FRONTEND_URL` if your frontend runs on a different port or domain.
- You can use other database URLs if you switch from SQLite to another database in the future.

## Running the Applications

### Start the Frontend

```bash
cd react-frontend
npm run dev
```

Frontend will be available at [http://localhost:5173](http://localhost:5173) (or as configured).

### Start the Backend

```bash
cd rust-server
cargo run
```
Backend server will listen on port `8080` by default (configurable).

## Database Setup

Initialize or migrate the SQLite database schema:
```bash
cargo run --bin migrate
```

## Available Scripts & Commands

### Frontend

- `npm run dev` — Start the development server
- `npm run build` — Build for production

### Backend

- `cargo run` — Run the server
- `cargo build` — Build the project
- `cargo run --bin migrate` — Run database migrations

## Troubleshooting

- Ensure all prerequisites are installed and up to date.
- For frontend issues, clear npm cache:
    ```bash
    npm cache clean --force
    ```
- For backend issues, check server logs and environment variables (e.g., `FRONTEND_URL`, `DATABASE_URL`).
- Verify SQLite is installed and accessible.

## Project TODOs

### Core Features

**Frontend**

- Implement Register Event functionality 
- Finish setup for full Event Updating
- Support image and map_embed_url via file picker
- Setup editing for event details
- Enable navigation to View Attendee page
- Display Attendance and Ticket Sales information
- Display Event Category and Map Embed
- Proper sharing of events
- Add Projection Information section

**Backend**
 
- Save new image file and store location reference
- Remove old and save new image file (2 instances)
- Create Organizer (consider dedicated route)
- Fetch related events based on: category_id, speakers, etc.

### UI/UX Improvements

**Frontend**

- Update form layout to support image inputs
- Replace all hardcoded rgba(53, 54, 52, alpha) values with theme colors
- Adjust dark mode color palette
- Improve Banner Styling for consistency and appeal

### Enhancements & Filters

**Frontend**

- Implement event Filter Functionality

## Project Structure

```
/
├── react-frontend/ # React app source
├── rust-server/    # Rust backend source
├── screenshots/    # UI screenshots
├── .gitignore      # Git ignored files and folders
└── README.md       # Project documentation
```
