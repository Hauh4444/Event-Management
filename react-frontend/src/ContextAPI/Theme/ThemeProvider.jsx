// External Libraries
import { useMemo, useState } from "react";
import { createTheme, ThemeProvider } from "@mui/material/styles";

// Internal Modules
import { ThemeContext } from "./ThemeContext";
import PropTypes from "prop-types";


/**
 * CustomThemeProvider is a context provider component that manages the application's theme mode ("light" or "dark").
 * It supplies the current theme mode and a toggle function to the entire React component tree via ThemeContext.
 * Internally, it uses Material-UI's ThemeProvider to apply the selected theme to all MUI components.
 *
 * @param { Object } props - The component props.
 * @param { React.ReactNode } props.children - The child components that will have access to the theme context.
 *
 * @component
 * @returns { JSX.Element } The provider component wrapping its children with both ThemeContext and MUI ThemeProvider.
 */
export const CustomThemeProvider = ({ children }) => {
    // State variable holds the theme mode defaulted to light theme
    const [mode, setMode] = useState("light");


    /**
     * Sets the theme to the passed props
     */
    const setTheme = (theme) => {
        setMode(theme);
    };


    /**
     * Memoized Material-UI theme object based on the current `mode`.
     * This object is recalculated only when `mode` changes, ensuring optimal performance.
     *
     * @constant
     * @type { import('@mui/material/styles').Theme }
     */
    const theme = useMemo(
        () => {
            if (mode === "light") {
                { /* Light mode theme */ }
                return createTheme({
                    cssVariables: true,
                    palette: {
                        mode: "light",
                        primary: {
                            main: "#3b5faf",
                            light: "#fefffe",
                            dark: "#353634",
                            contrastText: "#353634",
                        },
                        secondary: {
                            main: "#4b77bf",
                            light: "#ebf0fa",
                            dark: "#7d8189",
                            contrastText: "#7d8189",
                        },
                        background: {
                            default: "#fefffe",
                            paper: "#f8fbfd",
                        },
                        text: {
                            primary: "#353634",
                            secondary: "#7d8189",
                            disabled: "#b0b0b0",
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
                        heatmap: {
                            scale0: "rgba(53, 54, 52,  0.05)",
                            scale1: "#d0dbf9",
                            scale2: "#8b9ce2",
                            scale3: "#556fc8",
                            scale4: "#3b5faf",
                        }
                    },
                });
            } else if (mode === "dark") {
                { /* Dark mode theme */ }
                { /* TODO Adjust dark colors */ }
                return createTheme({
                    cssVariables: true,
                    palette: {
                        mode: "dark",
                        primary: {
                            main: "#3b5faf",
                            light: "#fefffe",
                            dark: "#353634",
                            contrastText: "#353634",
                        },
                        secondary: {
                            main: "#4b77bf",
                            light: "#ebf0fa",
                            dark: "#7d8189",
                            contrastText: "#7d8189",
                        },
                        background: {
                            default: "#fefffe",
                            paper: "#f8fbfd",
                        },
                        text: {
                            primary: "#353634",
                            secondary: "#7d8189",
                            disabled: "#b0b0b0",
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
                        heatmap: {
                            scale0: "rgba(53, 54, 52,  0.05)",
                            scale1: "#d0dbf9",
                            scale2: "#8b9ce2",
                            scale3: "#556fc8",
                            scale4: "#3b5faf",
                        }
                    },
                });
            }
        },
        [mode]
    );


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