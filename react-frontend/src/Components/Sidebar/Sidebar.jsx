// External Libraries
import { useLocation, useNavigate } from "react-router-dom";
import { Button } from "@mui/material";

// External Icons
import { RiDashboardHorizontalLine } from "react-icons/ri";
import { BsCalendar4Event } from "react-icons/bs";
import { PiUsersFour } from "react-icons/pi";
import { MdOutlineAnalytics, MdLogout } from "react-icons/md";
import { TbSettings, TbHeadset } from "react-icons/tb";

// Internal Contexts
import { useAuth } from "@/ContextAPI/Auth/AuthContext.js";

// Stylesheets
import "./Sidebar.css";


/**
 * Sidebar component.
 *
 * Displays the main navigation sidebar for the application, including
 * links to dashboard, events, attendees, analytics, settings, support,
 * and logout functionality.
 *
 * @returns {JSX.Element} The rendered Sidebar component.
 */
const Sidebar = () => {
    // React hooks
    const navigate = useNavigate();
    const location = useLocation();

    // Auth context
    const auth = useAuth();

    // Navigation and footer items with icons and paths
    const navItems = [
        { title: "Dashboard", path: "/dashboard", icon: <RiDashboardHorizontalLine /> },
        { title: "Events", path: "/events", icon: <BsCalendar4Event /> },
        { title: "Attendees", path: "/attendees", icon: <PiUsersFour /> },
        { title: "Analytics", path: "/analytics", icon: <MdOutlineAnalytics /> },
        { title: "Settings", path: "/settings", icon: <TbSettings /> },
    ];
    const footerItems = [
        { title: "Support", path: "/support", icon: <TbHeadset /> },
        { title: "Log out", path: "/logout", icon: <MdLogout /> },
    ]


    /**
     * Handles navigation actions for sidebar buttons.
     *
     * Determines the appropriate action based on the provided path:
     * - Navigates to the specified path if it's not "/logout".
     * - Logs out the user and redirects to the login page if the path is "/logout".
     *
     * @param {string} path - The target path to navigate to or the logout action.
     */
    const handleButtonPress = (path) => {
        if (path === "/logout") auth.logout().then(() => navigate("/login"));
        else navigate(path);
    }


    // Component JSX
    return (
        <>
            <div className="sidebar-spacer"></div>

            <div className="sidebar">
                <div className="header">
                    <h1>
                        Momentix
                    </h1>
                </div>

                <div className="nav">
                    { navItems.map((item, index) => (
                        <Button
                            className={ `btn ${ location.pathname === item.path && "focused" } ` }
                            key={ index }
                            onClick={ () => handleButtonPress(item.path) }
                        >
                            { item.icon } &ensp; { item.title }
                        </Button>
                    )) }
                </div>

                <div className="footer">
                    { footerItems.map((item, index) => (
                        <Button
                            className={ `btn ${ location.pathname === item.path && "focused" } ` }
                            key={ index }
                            onClick={ () => handleButtonPress(item.path) }
                        >
                            { item.icon } &ensp; { item.title }
                        </Button>
                    )) }
                </div>
            </div>
        </>
    )
}

export default Sidebar;