// External Libraries
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";


// Vite Configuration
export default defineConfig({
    // Array of plugins to use with Vite
    plugins: [react()], // Use the Vite React plugin for JSX/React support

    // Resolving paths to make imports cleaner
    resolve: {
        alias: {
            // Alias for the src directory
            "@": "/src", // Any import starting with "@" will be resolved to the "src" directory
        },
    },
    test: {
        globals: true,
        environment: "jsdom",
    },
});
