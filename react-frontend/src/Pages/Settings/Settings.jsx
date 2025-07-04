// External Libraries
import { useEffect, useState } from "react";
import { useLocation } from "react-router-dom";
import { Button, FormControl, InputLabel, MenuItem, Select, TextField } from "@mui/material";

// Internal Components
import Sidebar from "@/Components/Sidebar/Sidebar.jsx";
import TopNav from "@/Components/TopNav/TopNav.jsx";
import SettingsNav from "@/Components/SettingsNav/SettingsNav.jsx";

// Internal Utilities
import axiosInstance from "@/API/axiosInstance.js";

// Internal Contexts
import { useTheme } from "@/ContextAPI/Theme/ThemeContext.js";

// Stylesheets
import "./Settings.css";


/**
 * Settings Component
 *
 * This component renders a settings page that allows administrators to view and update
 * dynamic configuration options for a selected settings type (determined via URL parameters).
 *
 * @component
 * @returns { JSX.Element } The rendered Settings page component.
 */
// TODO Update form layout with image inputs
const Settings = () => {
    // React hooks
    const location = useLocation();
    // URL search parameters
    const filters = Object.fromEntries(new URLSearchParams(location.search));

    // Theme context
    const theme = useTheme();

    // State variables
    const [settings, setSettings] = useState({
        theme: theme.mode,
        color_blind_mode: "off",
    });

    // Derived constants
    const imageKeys = ["logo"]


    /**
     * Fetches settings data from API and updates state.
     *
     * @param { string } settingsType
     *
     * @typedef { Object } SettingsResponse
     * @property { any } [key] - Settings data keys vary depending on settingsType.
     *
     * @returns { Promise<void> }
     */
    const fetchData = async (settingsType) => {
        // Fetch settings
        let response = await axiosInstance.get(`/${ settingsType }/`);
        // Set settings state with response data
        setSettings(response.data);
    }


    /**
     * Handles fetching data on component mount.
     */
    useEffect(() => {
        // Fetch settings based on filters
        if (filters.s) fetchData(filters.s).catch((err) => console.error(err));
        // Manually reset settings to default general settings if no filters
        else setSettings({ theme: theme.mode });
    }, [filters.s])


    /**
     * Handles submitting updated settings.
     *
     * @param e
     */
    const handleSubmit = async (e) => {
        e.preventDefault();

        // Set general settings manually
        if (!filters.s) {
            theme.setTheme(settings.theme);
            return;
        }

        // PUT updated settings to backend
        await axiosInstance.put(`/${ filters.s }/`, settings).catch((err) => console.error(err));
        // Fetch updated data
        fetchData(filters.s).catch((err) => console.error(err));
    }


    // Component JSX
    return (
        <div className="settingsPage page">
            <Sidebar />

            <div className="mainPage">
                <TopNav />

                <div className="content">
                    { /* Settings Navigation */ }
                    <SettingsNav />

                    { /* Page Header */ }
                    <h1>{ filters.s ? filters.s.charAt(0).toUpperCase() + filters.s.slice(1) : "General" } Settings</h1>

                    { /* Settings Form */ }
                    <form className="settingsForm" onSubmit={ handleSubmit }>
                        { filters.s ? Object.entries(settings).map(([key, value]) => {
                            /* Map Settings By Filters */
                            if (key === "id") return;
                            return (
                                !imageKeys.includes(key) ? (
                                    /* Text Input Fields */
                                    <TextField
                                        className="input"
                                        key={ key }
                                        label={ key.charAt(0).toUpperCase() + key.slice(1) }
                                        name={ key }
                                        id={ key }
                                        type="text"
                                        variant="outlined"
                                        value={ value }
                                        onChange={ (e) => setSettings(prev =>
                                            ({ ...prev, [key]: e.target.value })) }
                                        fullWidth
                                    />
                                ) : (
                                    /* Image Input Fields */
                                    <div className="imageUpload" key={ key }>
                                        <div className="image">
                                            <img
                                                src={ `${ import.meta.env.VITE_BACKEND_STATIC_URL }/${ value }` }
                                                alt="image"
                                                onError={ (e) => {
                                                    e.currentTarget.onerror = null;
                                                    e.currentTarget.src = value;
                                                } }
                                            />
                                        </div>
                                        <input
                                            type="file"
                                            accept="image/*"
                                            id="image"
                                            style={ { display: "none" } }
                                            onChange={ (e) => {
                                                const file = e.target.files[0];
                                                const url = URL.createObjectURL(file);
                                                setSettings(prev => ({ ...prev, [key]: url }));
                                            } }
                                        />
                                        <Button
                                            className="btn"
                                            component="label"
                                            htmlFor="image"
                                        >
                                            Choose File
                                        </Button>
                                    </div>
                                )
                            )
                        }) : (
                            /* Manual General Settings */
                            <>
                                <FormControl fullWidth>
                                    <InputLabel id="site_theme">Theme</InputLabel>
                                    <Select
                                        labelId="site_theme"
                                        className="input"
                                        label="Theme"
                                        name="theme"
                                        id="theme"
                                        value={ settings.theme || theme.mode }
                                        onChange={ (e) => setSettings(prev =>
                                            ({ ...prev, theme: e.target.value })) }
                                        size="medium"
                                        variant="outlined"
                                    >
                                        <MenuItem value="light">Light</MenuItem>
                                        <MenuItem value="dark">Dark</MenuItem>
                                    </Select>
                                </FormControl>
                            </>
                        ) }

                        { filters.s === "user" && (
                            <TextField
                                className="input"
                                label="Password"
                                name="password"
                                id="password"
                                type="text"
                                variant="outlined"
                                value="********"
                                disabled
                                fullWidth
                            />
                        ) }

                        { /* Submit Button */ }
                        <Button
                            type="submit"
                            className="submitBtn"
                            size="small"
                        >
                            Save Changes
                        </Button>
                    </form>
                </div>
            </div>
        </div>
    )
}


export default Settings;