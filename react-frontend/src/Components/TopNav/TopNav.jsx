// External Libraries
import { Button } from "@mui/material";

// External Icons
import { PiCaretLeftBold, PiCaretRightBold } from "react-icons/pi";

// Internal Modules
import SearchBar from "@/Components/SearchBar/SearchBar.jsx";

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
        </div>
    )
}


export default TopNav;