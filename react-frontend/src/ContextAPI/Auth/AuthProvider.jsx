// External Libraries
import { useState } from "react";
import PropTypes from "prop-types";

// Internal Modules
import { AuthContext } from "./AuthContext.js";
import axiosInstance from "@/API/axiosInstance.js";


/**
 * Provides authentication state and methods to its children via context.
 *
 * Manages user info, authentication errors, and exposes async functions
 * for login, registration, logout, and auth status checking.
 *
 * @param { Object } props
 * @param { React.ReactNode } props.children - React children nodes.
 *
 * @component
 * @returns { JSX.Element } AuthProvider component wrapping children with AuthContext.
 */
const AuthProvider = ({ children }) => {
    // Holds the authenticated user data or null if not logged in
    const [user, setUser] = useState(null);
    // Holds the latest authentication error message, if any
    const [error, setError] = useState("");


    /**
     * Checks the current authentication status by requesting user data.
     * Updates the user state with returned data or null on failure.
     */
    const checkAuthStatus = async () => {
        try {
            const res = await axiosInstance.get("/check_auth_status/");
            setUser(res.data);
        } catch (err) {
            console.error(err);
            setUser(null);
        }
    };


    /**
     * Attempts to log in with provided credentials.
     * On success, updates user state by checking auth status.
     * On failure, sets an error message.
     *
     * @param { Object } credentials - User login credentials { username, password }.
     *
     * @returns { Promise<boolean> } True if login succeeded, else false.
     */
    const login = async (credentials) => {
        try {
            await axiosInstance.post("/login/", credentials);
            await checkAuthStatus();
            return true;
        } catch (err) {
            console.error(err);
            setError("Authentication failed. Please check your credentials.");
            return false;
        }
    };


    /**
     * Attempts to register a new user with provided credentials.
     * If successful, automatically logs in the new user.
     * On failure, sets an error message.
     *
     * @param { Object } credentials - User registration credentials.
     *
     * @returns { Promise<boolean> } True if registration (and login) succeeded, else false.
     */
    const register = async (credentials) => {
        try {
            await axiosInstance.post("/register/", credentials);
            await login(credentials);
            return true;
        } catch (err) {
            console.error(err);
            setError("Failed to create account.");
            return false;
        }
    };


    /**
     * Logs out the current user by calling the logout endpoint.
     * Clears the user state on success.
     * Sets an error message on failure.
     *
     * @returns { Promise<void> }
     */
    const logout = async () => {
        try {
            await axiosInstance.post("/logout/");
            setUser(null);
        } catch (err) {
            console.error(err);
            setError("Logout failed. Please try again.");
        }
    };


    // Context JSX
    return (
        <AuthContext.Provider value={ { user, error, checkAuthStatus, login, register, logout } }>
            { children }
        </AuthContext.Provider>
    );

};


AuthProvider.propTypes = {
    children: PropTypes.node.isRequired,
};


export default AuthProvider;
