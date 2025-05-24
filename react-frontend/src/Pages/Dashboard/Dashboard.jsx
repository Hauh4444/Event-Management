// External Libraries
import { useEffect, useState } from "react";
import { LineChart, areaElementClasses, lineElementClasses, markElementClasses } from "@mui/x-charts/LineChart";

// External Icons
import { MdOutlineEventNote } from "react-icons/md";

// Internal Modules
import Sidebar from "@/Components/Sidebar/Sidebar.jsx";
import TopNav from "@/Components/TopNav/TopNav.jsx";
import YearPicker from "@/Components/YearPicker/YearPicker.jsx";
import axiosInstance from "@/API/axiosInstance.js";

// Stylesheets
import "./Dashboard.css";


const Dashboard = () => {
    const [graphItems, setGraphItems] = useState([
        {
            title: "Events",
            iconColor: "#3b5faf",
            year: new Date().getFullYear(),
            seriesData: new Array(12).fill(0),
            xAxisData: Array.from({ length: 12 }, (_, i) => new Date(2025, i, 1)),
            label: "events",
        },
        {
            title: "Upcoming",
            iconColor: "#3b5faf",
            year: new Date().getFullYear(),
            seriesData: new Array(12).fill(0),
            xAxisData: Array.from({ length: 12 }, (_, i) => new Date(2025, i, 1)),
            label: "upcoming",
        },
        {
            title: "Cancelled",
            iconColor: "#ff2400",
            year: new Date().getFullYear(),
            seriesData: new Array(12).fill(0),
            xAxisData: Array.from({ length: 12 }, (_, i) => new Date(2025, i, 1)),
            label: "canceled",
        },
    ]);
    const [overviewItems, setOverviewItems] = useState([
        {
            title: "Events Tickets Sales",
            className: "ticketSales",
            year: new Date().getFullYear(),
            seriesData: new Array(12).fill(0),
            xAxisData: Array.from({ length: 12 }, (_, i) => new Date(2025, i, 1)),
            label: "tickets",
        },
        {
            title: "Attendees Trends",
            className: "attendeesTrends",
            year: new Date().getFullYear(),
            seriesData: new Array(12).fill(0),
            xAxisData: Array.from({ length: 12 }, (_, i) => new Date(2025, i, 1)),
            label: "attendees",
        }
    ]);


    const fetchData = async (newYear) => {
        try {
            const res = await axiosInstance.get("/overview/totals/", {
                params: { year: newYear }
            });

            const data = res.data;

            setGraphItems((prev) =>
                prev.map((item) =>
                    item.year === newYear
                        ? { ...item, seriesData: data[item.label] || new Array(12).fill(0) }
                        : item
                )
            );

            setOverviewItems((prev) =>
                prev.map((item) =>
                    item.year === newYear
                        ? { ...item, seriesData: data[item.label] || new Array(12).fill(0) }
                        : item
                )
            );
        } catch (err) {
            console.error(err);
        }
    };


    const onYearChange = (index, newYear, isGraph) => {
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

        fetchData(newYear).catch((err) => console.error(err));
    };


    useEffect(() => {
        fetchData(new Date().getFullYear()).catch((err) => console.error(err));
    }, []);


    return (
        <div className="dashboardPage page">
            <Sidebar />

            <div className="mainPage">
                <TopNav />

                <div className="content">
                    <h1>
                        Overview
                    </h1>

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

                                <h2>
                                    {item.seriesData.reduce((sum, val) => sum + val, 0)}
                                </h2>

                                <div className="chart">
                                    <LineChart
                                        height={ 175 }
                                        margin={{ left: 0, right: 0 }}
                                        series={[
                                            {
                                                data: item.seriesData,
                                                curve: "linear",
                                                valueFormatter: (value) => `${value} ${item.label}`,
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
                                            [`& .${lineElementClasses.root}`]: {
                                                stroke: "#3b5faf",
                                                strokeWidth: 2.5,
                                            },
                                            [`& .${markElementClasses.root}`]: {
                                                stroke: "#f8fbfd",
                                                r: 5,
                                                fill: "#3b5faf",
                                            },
                                            "& .MuiChartsAxis-line": {
                                                display: "none",
                                            },
                                            "& .MuiChartsAxis-tick": {
                                                display: "none",
                                            },
                                            "& .MuiChartsGrid-line": {
                                                stroke: "rgba(53, 54, 52, 0.25)",
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
                                                <stop offset="0%" stopColor="#3b5faf" stopOpacity="1" />
                                                <stop offset="100%" stopColor="#3b5faf" stopOpacity="0.25" />
                                            </linearGradient>
                                        </defs>
                                    </LineChart>
                                </div>
                            </div>
                        )) }
                    </div>

                    { overviewItems.map((item, index) => (
                        <div className={ item.className } key={ index }>
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
                                        valueFormatter: (value) => `${value} ${item.label}`,
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
                                    [`& .${areaElementClasses.root}`]: {
                                        fill: "url(#areaGradient)",
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
                                        stroke: "rgba(53, 54, 52, 0.1)",
                                        strokeDasharray: "5 5",
                                    },
                                }}
                            >
                                <defs>
                                    <linearGradient id="areaGradient" x1="0%" y1="0%" x2="0%" y2="100%">
                                        <stop offset="0%" stopColor="#3b5faf" stopOpacity="0.4" />
                                        <stop offset="100%" stopColor="#3b5faf" stopOpacity="0" />
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