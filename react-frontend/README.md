# React Frontend

This is the frontend application for the Event Management system. It is built with React, Material UI, and CSS, providing a user-friendly interface for managing events, attendees, and dashboards.

## Features

- User authentication pages
- Event and attendee dashboards
- Responsive UI with Material UI components
- Integration with Rust backend API

## Prerequisites

- Node.js v16 or higher
- npm (comes with Node.js)

## Setup

1. Navigate to this directory.
2. Run the following command to install dependencies:
    ```bash
    npm install
    ```

## Running the App

To start the development server with hot reload:
```bash
npm run dev
```

The app will be available at http://localhost:5173 (or as configured).

## Available Scripts

- `npm run dev` — Start the dev server
- `npm run build` — Create a production build

## Troubleshooting

- Make sure Node.js and npm are installed and updated. 
- Check your network connection if API requests fail. 
- Clear npm cache if install issues occur:
    ```bash
    npm cache clean --force
    ```

## Project TODOs

### Core Features

***Frontend***

- Implement Register Event functionality
- Finish setup for full Event Updating
- Support image and map_embed_url via file picker
- Setup editing for event details
- Enable navigation to View Attendee page
- Display Attendance and Ticket Sales information
- Display Event Category and Map Embed
- Proper sharing of events
- Add Projection Information section

### UI/UX Improvements

- Update form layout to support image inputs
- Replace all hardcoded rgba(53, 54, 52, alpha) values with theme colors
- Adjust dark mode color palette
- Improve Banner Styling for consistency and appeal

### Enhancements & Filters

- Implement event Filter Functionality

## Project Structure

```
.
├── src/                    # Application source code
│   ├── API/                # API utilities (e.g., axios setup)
│   ├── assets/             # Images and static assets
│   ├── Components/         # Reusable UI components
│   ├── ContextAPI/         # React context providers (Auth, Theme)
│   ├── Pages/              # Top-level pages (Dashboard, Events, etc.)
│   ├── Routes/             # App routing components
│   ├── Utils/              # Utility/helper functions
│   ├── App.jsx             # Main App component
│   ├── main.jsx            # Application entry point
│   ├── App.css             # Global styles
│   └── index.css           # Base CSS
├── package.json            # Project metadata and dependencies
├── package-lock.json       # Exact dependency versions
├── vite.config.js          # Vite build tool configuration
├── jsconfig.json           # JS project config (path aliases, etc.)
├── .eslint.config.js       # ESLint configuration for code linting
└── README.md               # Project documentation
```
