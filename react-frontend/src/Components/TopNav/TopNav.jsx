// External Libraries
import { useNavigate } from "react-router-dom";
import { Button } from "@mui/material";

// External Icons
import { PiCaretLeftBold, PiCaretRightBold } from "react-icons/pi";

// Internal Components
import SearchBar from "@/Components/SearchBar/SearchBar.jsx";

// Internal Contexts
import { useAuth } from "@/ContextAPI/Auth/AuthContext.js";

// Stylesheets
import "./TopNav.css";


/**
 * TopNav Component
 *
 * A navigation bar component that provides quick access to previous and next actions,
 * along with a search functionality. It is typically used at the top of a page to facilitate
 * navigation and content filtering.
 *
 * @component
 * @returns { JSX.Element } The rendered TopNav component.
 */
const TopNav = () => {
    // React hooks
    const navigate = useNavigate();

    // Auth context
    const auth = useAuth();


    // Component JSX
    return (
        <div className="topNav">
            <Button className="btn">
                <PiCaretLeftBold />
            </Button>

            <Button className="btn">
                <PiCaretRightBold />
            </Button>

            <SearchBar />

            <Button className="logoBtn" onClick={ () => navigate("/settings?s=organizer") }>
                <img
                    className="logo"
                    src={ `${ import.meta.env.VITE_BACKEND_STATIC_URL }/${ auth.user.logo }` }
                    alt={ auth.user.name }
                />

                <div className="userInfo">
                    <h3>{ auth.user.name }</h3>
                    <p>{ auth.user.website }</p>
                </div>
            </Button>
        </div>
    )
}


export default TopNav;