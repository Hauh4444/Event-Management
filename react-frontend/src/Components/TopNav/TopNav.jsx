// External Libraries
import { useEffect, useState } from "react";
import { Button, InputAdornment, TextField } from "@mui/material";

// External Icons
import { IoIosSearch } from "react-icons/io";
import { PiCaretLeftBold, PiCaretRightBold } from "react-icons/pi";

// Stylesheets
import "./TopNav.css";



const TopNav = () => {


    return (
        <div className="topNav">
            <Button className="btn">
                <PiCaretLeftBold />
            </Button>

            <Button className="btn">
                <PiCaretRightBold />
            </Button>

            <TextField
                className="search"
                type="text"
                placeholder="Search"
                slotProps={{
                    input: {
                        endAdornment: (
                            <InputAdornment position="end">
                                <Button className="btn">
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
        </div>
    )
}


export default TopNav;