// External Libraries
import { MenuItem, Select } from "@mui/material";


/**
 * YearPicker component.
 *
 * A custom year picker component that allows users to select a year from a dropdown list.
 * The component generates a list of years starting from `startYear` up to `endYear` and displays them as selectable options.
 * It utilizes Material-UI's `Select` and `MenuItem` components for rendering the dropdown list.
 *
 * @param { Object } props - The component's props.
 * @param { number } [props.startYear=1900] - The starting year in the list.
 * @param { number } [props.endYear=new Date().getFullYear()] - The ending year in the list.
 * @param { number } [props.value] - The currently selected year.
 * @param { function } props.onChange - Callback function to handle year selection.
 *
 * @component
 * @returns { JSX.Element } The rendered YearPicker component.
 */

const YearPicker = (
    { startYear = 1900, endYear = new Date().getFullYear(), value,
        onChange = () => {}, size = "small", sx = {} }
) => {
    // Derived constants
    const years = [];
    for (let y = startYear; y <= endYear; y++) years.push(y);


    // Component JSX
    return (
        <Select
            className="yearSelect"
            value={ value || "" }
            onChange={ (e) => onChange(e.target.value) }
            variant="outlined"
            size={ size }
            sx={{
                height: "40px",
                borderRadius: "10px",
                boxShadow: "0 0 2px rgba(53, 54, 52, 0.2)",
                "& fieldset": { border: "none" },
                "&:hover fieldset": { border: "none" },
                "&.Mui-focused fieldset": { border: "none" },
                "& .MuiInputBase-input": { borderRadius: "10px", transition: "all 0.2s ease", color: "var(--mui-palette-text-primary)" },
                "&:hover .MuiInputBase-input": { backgroundColor: "var(--mui-palette-secondary-light)", color: "var(--mui-palette-primary-main)", },
                "& .MuiSvgIcon-root": { fill: "var(--mui-palette-text-primary)" },
                "&:hover .MuiSvgIcon-root": { fill: "var(--mui-palette-primary-main)" },
                ...sx,
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
