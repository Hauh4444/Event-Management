// External Libraries
import { createContext, useContext } from "react";

// Create the theme context
export const ThemeContext = createContext();

// Custom hook to access theme context
export const useTheme = () => useContext(ThemeContext);
