// External Libraries
import { MenuItem, Select } from "@mui/material";


const YearPicker = ({ startYear = 1900, endYear = new Date().getFullYear(), value, onChange }) => {
    const years = [];
    for (let y = startYear; y <= endYear; y++) years.push(y);


    return (
        <Select
            value={ value || "" }
            onChange={(e) => onChange(e.target.value)}
            variant="outlined"
            size="small"
            style={{
                float: "right",
            }}
            sx={{
                height: "40px",
                borderRadius: "10px",
                boxShadow: "0 0 2px rgba(53, 54, 52, 0.2)",
                "& fieldset": { border: "none" },
                "&:hover fieldset": { border: "none" },
                "&.Mui-focused fieldset": { border: "none" },
                "& .MuiInputBase-input": { borderRadius: "10px", transition: "all 0.2s ease" },
                "&:hover .MuiInputBase-input": { backgroundColor: "#ebf0fa" }
            }}
        >
            { years.map((year) => (
                <MenuItem key={ year } value={ year }>
                    { year }
                </MenuItem>
            )) }
        </Select>
    );
};


export default YearPicker;
