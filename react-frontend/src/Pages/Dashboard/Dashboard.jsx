// External Libraries
import { useEffect, useState } from "react";
import { LineChart, areaElementClasses, lineElementClasses, markElementClasses } from "@mui/x-charts/LineChart";

// External Icons
import { MdOutlineEventNote } from "react-icons/md";

// Internal Components
import Sidebar from "@/Components/Sidebar/Sidebar.jsx";
import TopNav from "@/Components/TopNav/TopNav.jsx";
import YearPicker from "@/Components/YearPicker/YearPicker.jsx";

// Internal Utilities
import axiosInstance from "@/API/axiosInstance.js";

// Stylesheets
import "./Dashboard.css";


/**
 * Dashboard Page Component
 *
 * Provides an overview of event-related statistics, including events, upcoming events,
 * cancellations, ticket sales, and attendee trends. It fetches data based on the selected
 * year and displays it using line charts.
 *
 * @component
 * @returns { JSX.Element } The rendered Dashboard page component.
 */
const Dashboard = () => {
    // Initialize graph data with default zero values and current year
    const [graphItems, setGraphItems] = useState([
        {
            title: "Events",
            iconColor: "var(--mui-palette-primary-main)",
            year: new Date().getFullYear(),
            seriesData: new Array(12).fill(0),
            xAxisData: Array.from({ length: 12 }, (_, i) => new Date(2025, i, 1)),
            label: "events",
        },
        {
            title: "Upcoming",
            iconColor: "var(--mui-palette-primary-main)",
            year: new Date().getFullYear(),
            seriesData: new Array(12).fill(0),
            xAxisData: Array.from({ length: 12 }, (_, i) => new Date(2025, i, 1)),
            label: "upcoming",
        },
        {
            title: "Canceled",
            iconColor: "var(--mui-palette-red-primary)",
            year: new Date().getFullYear(),
            seriesData: new Array(12).fill(0),
            xAxisData: Array.from({ length: 12 }, (_, i) => new Date(2025, i, 1)),
            label: "canceled",
        },
    ]);
    // Initialize overview data similarly with zero values and current year
    const [overviewItems, setOverviewItems] = useState([
        {
            title: "Events Tickets Sales",
            year: new Date().getFullYear(),
            seriesData: new Array(12).fill(0),
            xAxisData: Array.from({ length: 12 }, (_, i) => new Date(2025, i, 1)),
            label: "tickets",
        },
        {
            title: "Attendees Trends",
            year: new Date().getFullYear(),
            seriesData: new Array(12).fill(0),
            xAxisData: Array.from({ length: 12 }, (_, i) => new Date(2025, i, 1)),
            label: "attendees",
        }
    ]);


    /**
     * Fetches data for the specified year and updates graphItems and overviewItems state.
     *
     * @param { number } newYear - The year for which to fetch data.
     *
     * @returns { Promise<void> }
     */
    const fetchData = async (newYear) => {
        // Fetch summary data for the given year from API
        const res = await axiosInstance.get("/overview/totals/", {
            params: { year: newYear }
        });

        // Update graph items with the fetched data, defaulting to zeros if missing
        setGraphItems((prev) =>
            prev.map((item) =>
                item.year === newYear
                    ? { ...item, seriesData: res.data[item.label] || new Array(12).fill(0) }
                    : item
            )
        );

        // Update overview items similarly with fetched data
        setOverviewItems((prev) =>
            prev.map((item) =>
                item.year === newYear
                    ? { ...item, seriesData: res.data[item.label] || new Array(12).fill(0) }
                    : item
            )
        );
    };


    /**
     * Handles fetching data for current year on component mount.
     */
    useEffect(() => {
        fetchData(new Date().getFullYear()).catch((err) => console.error(err));
    }, []);


    /**
     * Handles year selection change for either graph or overview items.
     *
     * @param { number } index - Index of the item to update.
     * @param { number } newYear - The new year selected.
     * @param { boolean } isGraph - Whether the item is part of graphItems or overviewItems.
     */
    const onYearChange = (index, newYear, isGraph) => {
        // Update the year in the respective state array
        if (isGraph) {
            setGraphItems((prev) =>
                prev.map((item, idx) =>
                    idx === index
                        ? { ...item, year: newYear }
                        : item
                )
            );
        } else {
            setOverviewItems((prev) =>
                prev.map((item, idx) =>
                    idx === index
                        ? { ...item, year: newYear }
                        : item
                )
            );
        }

        // Fetch fresh data for the selected year to update charts
        fetchData(newYear).catch((err) => console.error(err));
    };


    // Component JSX
    return (
        <div className="dashboardPage page">
            <Sidebar />

            <div className="mainPage">
                <TopNav />

                <div className="content">
                    <h1>
                        Overview
                    </h1>

                    { /* Render line charts for event-related graph items */ }
                    <div className="graphs">
                        { graphItems.map((item, index) => (
                            <div className="item" key={ index }>
                                <h3>
                                    <MdOutlineEventNote className="icon" style={{ fill: item.iconColor }} />
                                    { item.title }
                                    <YearPicker
                                        startYear={ 2020 }
                                        endYear={ 2030 }
                                        value={ item.year }
                                        onChange={ (year) => onYearChange(index, year, true) }
                                    />
                                </h3>

                                { /* Display total sum of the series data */ }
                                <h2>
                                    { item.seriesData.reduce((sum, val) => sum + val, 0) }
                                </h2>

                                <div className="chart">
                                    <LineChart
                                        height={ 175 }
                                        margin={{ left: 0, right: 0 }}
                                        series={[
                                            {
                                                data: item.seriesData,
                                                curve: "linear",
                                                valueFormatter: (value) => `${ value } ${ item.label }`,
                                            }
                                        ]}
                                        xAxis={[
                                            {
                                                scaleType: "band",
                                                data: item.xAxisData,
                                                valueFormatter: (value) => value.toLocaleString("default", { month: "short" }),
                                            }
                                        ]}
                                        yAxis={[
                                            {
                                                position: "none",
                                                min: 0,
                                            }
                                        ]}
                                        axisHighlight={{
                                            x: "band",
                                        }}
                                        sx={{
                                            [`& .${ lineElementClasses.root }`]: {
                                                stroke: "var(--mui-palette-primary-main)",
                                                strokeWidth: 3,
                                            },
                                            [`& .${ markElementClasses.root }`]: {
                                                stroke: "var(--mui-palette-background-paper)",
                                                r: 5,
                                                fill: "var(--mui-palette-primary-main)",
                                            },
                                            "& .MuiChartsAxis-line": {
                                                display: "none",
                                            },
                                            "& .MuiChartsAxis-tick": {
                                                display: "none",
                                            },
                                            "& .MuiChartsGrid-line": {
                                                stroke: "rgba(53, 54, 52,  0.15)",
                                                strokeDasharray: "3 3",
                                            },
                                            "&:hover": {
                                                "& .MuiChartsAxisHighlight-root": {
                                                    fill: "url(#bandGradient)",
                                                },
                                            },
                                        }}
                                    >
                                        <defs>
                                            <linearGradient id="bandGradient" x1="0%" y1="0%" x2="0%" y2="100%">
                                                <stop offset="0%" stopColor="var(--mui-palette-primary-main)" stopOpacity="1" />
                                                <stop offset="100%" stopColor="var(--mui-palette-primary-main)" stopOpacity="0.25" />
                                            </linearGradient>
                                        </defs>
                                    </LineChart>
                                </div>
                            </div>
                        )) }
                    </div>

                    { /* Render line charts for overview items with area charts */ }
                    { overviewItems.map((item, index) => (
                        <div className="overviewItem" key={ index }>
                            <h2>
                                { item.title }

                                <YearPicker
                                    startYear={ 2020 }
                                    endYear={ 2030 }
                                    value={ item.year }
                                    onChange={ (year) => onYearChange(index, year, false) }
                                />
                            </h2>

                            <LineChart
                                height={ 300 }
                                margin={{ left: 0, right: 35 }}
                                series={[
                                    {
                                        data: item.seriesData,
                                        area: true,
                                        showMark: false,
                                        valueFormatter: (value) => `${ value } ${ item.label }`,
                                    }
                                ]}
                                xAxis={[
                                    {
                                        scaleType: "point",
                                        data: item.xAxisData,
                                        valueFormatter: (value) => value.toLocaleString("default", { month: "short" }),
                                    }
                                ]}
                                yAxis={[
                                    {
                                        position: "left",
                                        min: 0,
                                    }
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
                                <defs>
                                    <linearGradient id="areaGradient" x1="0%" y1="0%" x2="0%" y2="100%">
                                        <stop offset="0%" stopColor="var(--mui-palette-primary-main)" stopOpacity="0.25" />
                                        <stop offset="100%" stopColor="var(--mui-palette-primary-main)" stopOpacity="0" />
                                    </linearGradient>
                                </defs>
                            </LineChart>
                        </div>
                    )) }
                </div>
            </div>
        </div>
    )
}


export default Dashboard;
