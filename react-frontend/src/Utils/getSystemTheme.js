/**
 * Detects the user's preferred system color scheme ("light" or "dark").
 * Falls back to "light" if window is undefined (e.g., during server-side rendering).
 *
 * @function getSystemTheme
 * @returns { "light" | "dark" } The current system theme preference.
 */
const getSystemTheme = () => {
    if (typeof window === "undefined") return "light"; // SSR safety
    return window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
}


export default getSystemTheme;