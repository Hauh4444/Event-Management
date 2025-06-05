// External Libraries
import { Button, InputAdornment, TextField } from "@mui/material";

// External Icons
import { IoIosSearch } from "react-icons/io";

// Stylesheets
import "./SearchBar.css";


/**
 * SearchBar component.
 *
 * Renders a styled text input field with a search icon button at the end.
 * Primarily used for search input functionality.
 *
 * @param { Object } props - The component's props.
 * @param { function } props.onChange - Callback function to handle year selection.
 * *
 * @component
 * @returns { JSX.Element } The rendered search bar component.
 */
const SearchBar = ({ onChange = () => {}, value, onClick = () => {} }) => {
    // Component JSX
    return (
        <TextField
            className="search"
            onChange={ (e) => onChange(e.target.value) }
            onKeyDown={(e) => { if (e.key === "Enter") onClick(); }}
            value={ value }
            type="text"
            placeholder="Search"
            slotProps={{
                input: {
                    endAdornment: (
                        <InputAdornment position="end">
                            <Button className="btn" onClick={ () => onClick() }>
                                <IoIosSearch className="icon" />
                            </Button>
                        </InputAdornment>
                    ),
                },
            }}
            sx={{
                "& .MuiInputBase-root": {
                    height: "40px",
                    padding: "0",
                    fontSize: "16px",
                    color: "#353634"
                },
                "& .MuiOutlinedInput-root": {
                    "& fieldset": {
                        borderColor: "rgba(53, 54, 52, 0.1)",
                        borderRadius: "10px",
                    },
                    "&:hover fieldset": {
                        borderColor: "rgba(53, 54, 52, 0.5)",
                    },
                    "&.Mui-focused fieldset": {
                        borderColor: "rgba(53, 54, 52, 0.75)",
                    },
                },
            }}
        />
    )
}


export default SearchBar;