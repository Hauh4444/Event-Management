// External Libraries
import { createContext, useContext } from "react";

// Create the authentication context
export const AuthContext = createContext(null);

// Custom hook to access authentication context
export const useAuth = () => useContext(AuthContext);