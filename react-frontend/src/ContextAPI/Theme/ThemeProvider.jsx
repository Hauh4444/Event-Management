// External Libraries
import { useEffect, useMemo, useState } from "react";
import { createTheme, ThemeProvider } from "@mui/material/styles";

// Internal Components
import { ThemeContext } from "./ThemeContext";
import PropTypes from "prop-types";

// Internal Utilities
import getSystemTheme from "@/Utils/getSystemTheme.js";


/**
 * CustomThemeProvider is a context provider component that manages the application's theme mode ("light" or "dark").
 * It sets the initial mode based on the user's system preference, listens for system theme changes,
 * and provides a toggle method via ThemeContext. It also wraps children with MUI's ThemeProvider to apply the theme.
 *
 * @param { Object } props - The component props.
 * @param { React.ReactNode } props.children - The child components that will have access to the theme context.
 *
 * @component
 * @returns { JSX.Element } The provider component wrapping its children with both ThemeContext and MUI ThemeProvider.
 */
export const CustomThemeProvider = ({ children }) => {
    /**
     * React state to hold the current theme mode, initialized from system preference.
     *
     * @type {[ "light" | "dark", Function ]}
     */
    const [mode, setMode] = useState(getSystemTheme);


    /**
     * Listens for system theme changes and updates the theme mode reactively.
     * Uses the 'change' event on the prefers-color-scheme media query.
     */
    useEffect(() => {
        const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
        const handleChange = (e) => {
            setMode(e.matches ? "dark" : "light");
        };

        mediaQuery.addEventListener("change", handleChange);
        return () => mediaQuery.removeEventListener("change", handleChange);
    }, []);


    /**
     * Updates the current theme mode manually. Exposed via context.
     *
     * @function setTheme
     * @param { "light" | "dark" } theme - The new theme mode to apply.
     */
    const setTheme = (theme) => {
        setMode(theme);
    };


    /**
     * Memoized MUI theme object that updates whenever `mode` changes.
     * Uses custom palette values and component overrides for consistency.
     *
     * @type { import("@mui/material").Theme }
     */
    const theme = useMemo(() => {
        const isLight = mode === "light";

        return createTheme({
            cssVariables: true,
            components: {
                MuiOutlinedInput: {
                    styleOverrides: {
                        root: {
                            '& input:-webkit-autofill': {
                                WebkitBoxShadow: `0 0 0 1000px var(--mui-palette-background-paper) inset`,
                                WebkitTextFillColor: 'var(--mui-palette-text-primary)',
                                transition: 'background-color 5000s ease-in-out 0s',
                            },
                        },
                    },
                },
            },
            palette: {
                mode: isLight ? "light" : "dark",
                primary: isLight
                    ? {
                        /* Light theme primary colors */
                        main: "#3b5faf",
                        light: "#fefffe",
                        dark: "#353634",
                        contrastText: "#353634",
                    }
                    : {
                        /* Dark theme primary colors */
                        main: "#3b5faf",
                        light: "#353634",
                        dark: "#fefffe",
                        contrastText: "#ebf0fa",
                    },
                secondary: isLight
                    ? {
                        /* Light theme secondary colors */
                        main: "#4b77bf",
                        light: "#ebf0fa",
                        dark: "#7d8189",
                        contrastText: "#7d8189",
                    }
                    : {
                        /* Dark theme secondary colors */
                        main: "#4b77bf",
                        light: "#353634",
                        dark: "#ebf0fa",
                        contrastText: "#b0b0b0",
                    },
                background: isLight
                    ? {
                        /* Light theme background colors */
                        default: "#fefffe",
                        paper: "#f8fbfd",
                    }
                    : {
                        /* Dark theme background colors */
                        default: "#181818",
                        paper: "#212021",
                    },
                text: isLight
                    ? {
                        /* Light theme text colors */
                        primary: "#353634",
                        secondary: "#7d8189",
                        disabled: "#b0b0b0",
                    }
                    : {
                        /* Dark theme text colors */
                        primary: "#ebf0fa",
                        secondary: "#b0b0b0",
                        disabled: "#7d8189",
                    },
                heatmap: isLight
                    ? {
                        /* Light theme heatmap colors */
                        scale0: "rgba(53, 54, 52,  0.05)",
                        scale1: "#d0dbf9",
                        scale2: "#8b9ce2",
                        scale3: "#556fc8",
                        scale4: "#3b5faf",
                    }
                    : {
                        /* Dark theme heatmap colors */
                        scale0: "rgba(53, 54, 52,  0.75)",
                        scale1: "#3b5faf",
                        scale2: "#556fc8",
                        scale3: "#8b9ce2",
                        scale4: "#d0dbf9",
                    },
                yellow: {
                    primary: "#fcae1e",
                },
                red: {
                    primary: "#d21404",
                    secondary: "#e3242b",
                    light: "#fa8072",
                    dark: "#7c0a02",
                },
                green: {
                    main: "#007848",
                    light: "#b9ebd4",
                },
            },
        });
    }, [mode]);


    // Component JSX
    return (
        <ThemeContext.Provider value={ { mode, setTheme } }>
            <ThemeProvider theme={ theme }>{ children }</ThemeProvider>
        </ThemeContext.Provider>
    );
};


CustomThemeProvider.propTypes = {
    children: PropTypes.node.isRequired,
};


export default CustomThemeProvider;