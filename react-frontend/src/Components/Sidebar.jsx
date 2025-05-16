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
import { useAuth } from "@/ContextAPI/AuthContext.js";

// Stylesheets
import "./Sidebar.css";


const Sidebar = () => {
    const navigate = useNavigate();
    const location = useLocation();

    const auth = useAuth();

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

    const handleButtonPress = (path) => {
        if (path === "/logout") auth.logout().then(() => navigate("/login"));
        else navigate(path);
    }

    return (
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
                        className="btn"
                        key={ index }
                        onClick={ () => handleButtonPress(item.path) }
                    >
                        { item.icon } &ensp; { item.title }
                    </Button>
                )) }
            </div>
        </div>
    )
}

export default Sidebar;