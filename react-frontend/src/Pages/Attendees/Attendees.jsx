// External Libraries
import { useEffect, useState } from "react";
import { Button } from "@mui/material";
import { areaElementClasses, LineChart, lineElementClasses } from "@mui/x-charts/LineChart";

// External Icons
import { AiOutlineExport } from "react-icons/ai";
import { FaArrowDownLong, FaArrowUpLong, FaInfinity } from "react-icons/fa6";

// Internal Modules
import Sidebar from "@/Components/Sidebar/Sidebar.jsx";
import TopNav from "@/Components/TopNav/TopNav.jsx";
import YearPicker from "@/Components/YearPicker/YearPicker.jsx";
import axiosInstance from "@/API/axiosInstance.js";

// Stylesheets
import "./Attendees.css";
import CalendarHeatmap from "@/Components/CalendarHeatmap/CalendarHeatmap.jsx";


const Attendees = () => {
    // State variables
    const [attendeesOverview, setAttendeesOverview] = useState({
        seriesData: new Array(12).fill(0),
        xAxisData: Array.from({ length: 12 }, (_, i) => new Date(2025, i, 1)),
        total: 0,
        lastYearTotal: 0,
    });
    const [selectedYear, setSelectedYear] = useState(new Date().getFullYear());
    const [query, setQuery] = useState("");

    // Derived constants
    const isLastYearZero = attendeesOverview.lastYearTotal === 0;
    const salesChange = isLastYearZero
        ? ""
        : (((attendeesOverview.total - attendeesOverview.lastYearTotal) / attendeesOverview.lastYearTotal) * 100)
            .toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 });
    const isIncrease = isLastYearZero || (parseFloat(salesChange) > 0);


    /**
     * Fetches attendee overview data from API and updates state.
     * Uses Promise all to fetch current and previous year attendee data concurrently.
     * 
     * @param { number } year - The year selected.
     * 
     * @returns { Promise<void> }
     */
    const fetchData = async (year) => {
        // Fetch attendee overview data for current year and last year in parallel
        const [currentYearResponse, previousYearResponse] = await Promise.all([
            axiosInstance.get("/overview/attendees/", { params: { year: year } }),
            axiosInstance.get("/overview/attendees/", { params: { year: year - 1 } }),
        ]);

        // Extract last year attendees array, fallback to empty array if undefined
        const lastYearAttendees = previousYearResponse.data.attendees || [];
        // Sum all attendees sold last year
        const lastYearTotal = lastYearAttendees.reduce((total, count) => total + count, 0);

        // Extract current year attendees monthly data, fallback to zero array if undefined
        const currentYearAttendees = currentYearResponse.data.attendees || new Array(12).fill(0);

        // Generate month labels for x-axis for current year
        const xAxisDates = Array.from({ length: 12 }, (_, monthIndex) =>
            new Date(year, monthIndex, 1));

        // Update attendees overview state
        setAttendeesOverview({
            seriesData: currentYearAttendees,
            xAxisData: xAxisDates,
            total: currentYearResponse.data.total,
            lastYearTotal: lastYearTotal,
        });
    }


    /**
     * Handles year selection change for attendee information.
     *
     * @param { number } year - The year selected.
     */
    const onYearChange = (year) => {
        setSelectedYear(year);
        setQuery("");
        fetchData(year).catch((err) => console.error(err));
    };


    /**
     * Handles fetching data on component mount.
     */
    useEffect(() => {
        fetchData(new Date().getFullYear()).catch((err) => console.error(err));
    }, []);


    // Component JSX
    return (
        <div className="attendeesPage page">
            <Sidebar />

            <div className="mainPage">
                <TopNav />

                <div className="content">
                    <div className="head">
                        <div>
                            <h1>
                                Attendee Tracking
                            </h1>

                            <p>
                                Track attendee growth and trends year over year—gain clear, visual insights into your events' performance.
                            </p>
                        </div>

                        <span>
                            { /* Year select for attendee information */ }
                            <YearPicker
                                startYear={ 2020 }
                                endYear={ 2030 }
                                value={ selectedYear }
                                onChange={ (year) => onYearChange(year) }
                                size="small"
                                sx={{
                                    height: "50px",
                                    borderRadius: "5px",
                                    border: "2px solid rgba(53, 54, 52, 0.1)",
                                    boxShadow: "none",
                                    marginRight: "15px",
                                    fontWeight: "bold",
                                    "& .MuiInputBase-input": {
                                        height: "33px",
                                        lineHeight: "33px",
                                        borderRadius: "5px",
                                        transition: "all 0.2s ease"
                                    },
                                }}
                            />

                            { /* Export button triggers PDF export */ }
                            { /* TODO export functionality */ }
                            <Button className="btn">
                                <AiOutlineExport className="icon" />
                                Export
                            </Button>
                        </span>
                    </div>

                    <div className="overviewItem">
                        <div className="info">
                            <h2>
                                Total Attendees
                            </h2>

                            { /* Display total formatted */ }
                            <h1>
                                { attendeesOverview.total.toLocaleString(undefined, 0) }
                            </h1>

                            <p>
                                <span className={ isIncrease ? "increase" : "decrease" }>
                                    { /* Show up/down arrow or infinity if last year total is zero */ }
                                    { isLastYearZero ? (
                                        <>
                                            <FaArrowUpLong className="icon" />
                                            <FaInfinity className="icon" />
                                        </>
                                    ) : (
                                        isIncrease
                                            ? <FaArrowUpLong className="icon" />
                                            : <FaArrowDownLong className="icon" />
                                    )}
                                    { salesChange }%
                                </span>
                                &ensp;vs last year
                            </p>

                        </div>

                        { /* Line chart visualizing monthly attendees */ }
                        <LineChart
                            height={ 300 }
                            margin={{ left: 0, right: 35 }}
                            series={[
                                {
                                    area: true,
                                    data: attendeesOverview.seriesData,
                                    showMark: false,
                                    valueFormatter: (value) =>
                                        `${ value.toLocaleString(undefined, 0) }`,
                                },
                            ]}
                            xAxis={[
                                {
                                    scaleType: "point",
                                    data: attendeesOverview.xAxisData,
                                    valueFormatter: (value) => value.toLocaleString("default", { month: "short" }),
                                },
                            ]}
                            yAxis={[
                                {
                                    width: 75,
                                    position: "left",
                                    min: 0,
                                    valueFormatter: (value) => {
                                        if (value >= 1_000_000) return `${ (value / 1_000_000) }m`;
                                        else if (value >= 1_000) return `${ (value / 1_000) }k`;
                                        else return `${ value }`;
                                    },
                                },
                            ]}
                            grid={{ horizontal: true }}
                            sx={{
                                [`& .${ areaElementClasses.root }`]: {
                                    fill: "url(#areaGradient)",
                                },
                                [`& .${ lineElementClasses.root }`]: {
                                    stroke: "var(--mui-palette-primary-main)",
                                    strokeWidth: 3,
                                },
                                "& .MuiChartsAxis-line": {
                                    display: "none",
                                },
                                "& .MuiChartsAxis-tick": {
                                    display: "none",
                                },
                                "& .MuiChartsAxis-bottom .MuiChartsAxis-tickLabel": {
                                    transform: "translateY(15px)",
                                },
                                "& .MuiChartsGrid-line": {
                                    stroke: "rgba(53, 54, 52,  0.1)",
                                    strokeDasharray: "5 5",
                                },
                            }}
                        >
                            { /* Gradient fill for area chart */ }
                            <defs>
                                <linearGradient id="areaGradient" x1="0%" y1="0%" x2="0%" y2="100%">
                                    <stop offset="0%" stopColor="var(--mui-palette-primary-main)" stopOpacity="0.25" />
                                    <stop offset="100%" stopColor="var(--mui-palette-primary-main)" stopOpacity="0" />
                                </linearGradient>
                            </defs>
                        </LineChart>
                    </div>

                    { /* TODO Basic attendee info, action items, etc */ }
                </div>
            </div>
        </div>
    )
}


export default Attendees;