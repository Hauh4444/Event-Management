// External Libraries
import { createSearchParams, useLocation, useNavigate } from "react-router-dom";
import { Button } from "@mui/material";

// Stylesheets
import "./SettingsNav.css";


const SettingsNav = () => {
    // React hooks
    const navigate = useNavigate();
    const location = useLocation();
    // URL search parameters
    const filters = Object.fromEntries(new URLSearchParams(location.search));

    // Derived constants
    const navButtons = [
        { nav: undefined, label: "General" },
        { nav: "organizer", label: "Account" },
        { nav: "user", label: "User" },
    ];


    /**
     * Handles navigation within settings
     *
     * @param { string } nav
     */
    const handleNav = (nav) => {
        // Set settings filters
        if (nav) filters.s = nav;
        else delete filters.s;

        // Navigate with new url parameters
        navigate({
            pathname: location.pathname,
            search: createSearchParams(filters).toString(),
        });
    }


    // Component JSX
    return (
        <div className="settingsNav">
            { navButtons.map((item, index) => (
                <Button
                    className={ filters.s === item.nav ? "btn selected" : "btn" }
                    key={ index }
                    onClick={ () => handleNav(item.nav) }
                >
                    { item.label }
                </Button>
            )) }
        </div>
    )
}


export default SettingsNav;