// External Libraries
import { useEffect, useState } from "react";
import { Button } from "@mui/material";
import { areaElementClasses, LineChart, lineElementClasses } from "@mui/x-charts/LineChart";

// External Icons
import { AiOutlineExport } from "react-icons/ai";
import { FaArrowDownLong, FaArrowUpLong, FaInfinity } from "react-icons/fa6";
import { MdOutlineEventNote } from "react-icons/md";

// Internal Components
import Sidebar from "@/Components/Sidebar/Sidebar.jsx";
import TopNav from "@/Components/TopNav/TopNav.jsx";
import YearPicker from "@/Components/YearPicker/YearPicker.jsx";
import CalendarHeatmap from "@/Components/CalendarHeatmap/CalendarHeatmap.jsx";

// Internal Utilities
import axiosInstance from "@/API/axiosInstance.js";
import handleExportPDF from "@/Utils/exportPDF.js";

// Stylesheets
import "./Attendees.css";


const Attendees = () => {
    // State variables
    const [selectedYear, setSelectedYear] = useState(new Date().getFullYear());
    const [attendeesMonthlyOverview, setAttendeesMonthlyOverview] = useState({
        seriesData: new Array(12).fill(0),
        xAxisData: Array.from({ length: 12 }, (_, i) => new Date(2025, i, 1)),
        total: 0,
        lastYearTotal: 0,
    });
    const [dailyAttendeesOverview, setDailyAttendeesOverview] = useState([]);
    const [attendanceExtremes, setAttendanceExtremes] = useState({
        most: new Array(5).fill(0),
        least: new Array(5).fill(0),
    })
    const [noShowMonthlyOverview, setNoShowMonthlyOverview] = useState({
        seriesData: new Array(12).fill(0),
        xAxisData: Array.from({ length: 12 }, (_, i) => new Date(2025, i, 1)),
        totalCount: 0,
        totalRate: 0,
        lastYearTotal: 0,
        lastYearRate: 0,
    });

    // Derived constants
    const isLastYearAttendanceZero = attendeesMonthlyOverview.lastYearTotal === 0;
    const attendanceChange = isLastYearAttendanceZero
        ? ""
        : (((attendeesMonthlyOverview.total - attendeesMonthlyOverview.lastYearTotal) /
            attendeesMonthlyOverview.lastYearTotal) * 100)
            .toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 });
    const isAttendanceIncrease = isLastYearAttendanceZero || (parseFloat(attendanceChange) > 0);

    const isLastYearNoShowZero = noShowMonthlyOverview.lastYearTotal === 0;
    const noShowChange = isLastYearNoShowZero
        ? ""
        : (((noShowMonthlyOverview.totalCount - noShowMonthlyOverview.lastYearTotal) /
            noShowMonthlyOverview.lastYearTotal) * 100)
            .toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 });
    const isNoShowIncrease = isLastYearNoShowZero || (parseFloat(noShowChange) > 0);


    /**
     * Handles year selection change for attendee information.
     *
     * @param { number } year - The year selected.
     */
    const onYearChange = (year) => {
        setSelectedYear(year);
        fetchData(year).catch((err) => console.error(err));
    };


    /**
     * Fetches attendee overview data from API and updates state.
     * Uses Promise all to fetch current and previous year attendee data concurrently.
     *
     * @param { number } year - The year selected.
     *
     * @returns { Promise<void> }
     */
    const fetchData = async (year) => {
        // Generate an array of Date objects representing the first day of each month for x-axis labels
        const xAxisDates = Array.from({ length: 12 }, (_, monthIndex) =>
            new Date(year, monthIndex, 1)
        );

        // Fetch monthly attendee counts for current year and previous year concurrently
        const [currentYearAttendanceRes, previousYearAttendanceRes] = await Promise.all([
            axiosInstance.get("/attendees/counts/monthly/", { params: { year: year } }),
            axiosInstance.get("/attendees/counts/monthly/", { params: { year: year - 1 } }),
        ]);
        // Update monthly attendees overview state with current year data and totals
        setAttendeesMonthlyOverview({
            seriesData: currentYearAttendanceRes.data.attendees || new Array(12).fill(0),
            xAxisData: xAxisDates,
            total: currentYearAttendanceRes.data.total,
            lastYearTotal: previousYearAttendanceRes.data.total,
        });

        // Fetch daily attendee counts for the current year
        const dailyAttendeesRes = await axiosInstance.get("/attendees/counts/daily/", {
            params: { year }
        });
        // Update daily attendees overview state with fetched data
        setDailyAttendeesOverview(dailyAttendeesRes.data["attendee_counts"]);

        // Fetch attendance extremes (top and bottom attended events) for the current year
        const attendanceExtremesRes = await axiosInstance.get("/attendees/extremes/", {
            params: { year }
        });
        // Update attendance extremes state with fetched data
        setAttendanceExtremes(attendanceExtremesRes.data);

        // Fetch monthly no show counts for current year and previous year concurrently
        const [currentYearNoShowRes, previousYearNoShowRes] = await Promise.all([
            axiosInstance.get("/attendees/no-shows/monthly/", { params: { year: year } }),
            axiosInstance.get("/attendees/no-shows/monthly/", { params: { year: year - 1 } }),
        ]);
        console.log(currentYearNoShowRes.data);
        // Update monthly no shows overview state with current year data and totals
        setNoShowMonthlyOverview({
            seriesData: currentYearNoShowRes.data.no_show_rates || new Array(12).fill(0),
            xAxisData: xAxisDates,
            totalCount: currentYearNoShowRes.data.total_count,
            totalRate: currentYearNoShowRes.data.total_rate,
            lastYearTotal: previousYearNoShowRes.data.total_count,
            lastYearRate: previousYearNoShowRes.data.total_rate,
        });
    }


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
                                Track attendee growth and trends year over year—gain clear, visual insights into your events'
                                performance.
                            </p>
                        </div>

                        <span>
                            { /* Year select for attendee information */ }
                            <YearPicker
                                startYear={ new Date().getFullYear() - 5 }
                                endYear={ new Date().getFullYear() }
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
                            <Button
                                className="btn"
                                onClick={ async () => {
                                    await handleExportPDF(document.querySelector(".content"))
                                } }
                            >
                                <AiOutlineExport className="icon" />
                                Export
                            </Button>
                        </span>
                    </div>

                    { /* TODO Projection information */ }
                    <div className="overviewItem">
                        <div className="info">
                            <h2>
                                Total Attendees
                            </h2>

                            { /* Display total formatted */ }
                            <h1>
                                { attendeesMonthlyOverview.total.toLocaleString(undefined, 0) }
                            </h1>

                            <p>
                                <span className={ isAttendanceIncrease ? "increase" : "decrease" }>
                                    { /* Show up/down arrow or infinity if last year total is zero */ }
                                    { isLastYearAttendanceZero ? (
                                        <>
                                            <FaArrowUpLong className="icon" />
                                            <FaInfinity className="icon" />
                                        </>
                                    ) : (
                                        isAttendanceIncrease
                                            ? <FaArrowUpLong className="icon" />
                                            : <FaArrowDownLong className="icon" />
                                    )}
                                    { attendanceChange }%
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
                                    data: attendeesMonthlyOverview.seriesData,
                                    showMark: false,
                                    valueFormatter: (value) => `${ value.toLocaleString(undefined, 0) }`,
                                },
                            ]}
                            xAxis={[
                                {
                                    scaleType: "point",
                                    data: attendeesMonthlyOverview.xAxisData,
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

                    <CalendarHeatmap
                        values={ dailyAttendeesOverview }
                        valueType="attendees"
                        scaleValues={ [60, 120, 210] }
                        startDate={ new Date(selectedYear, 0, 1) }
                        endDate={ new Date(selectedYear, 11, 31) }
                    />

                    <div className="attendanceExtremes">
                        <table>
                            <colgroup>
                                <col style={{ width: "40%" }} />
                                <col style={{ width: "30%" }} />
                                <col style={{ width: "15%" }} />
                                <col style={{ width: "15%" }} />
                            </colgroup>

                            <thead>
                            <tr>
                                <th className="title most" colSpan={ 4 }>
                                    <MdOutlineEventNote className="icon"/>Most Attended Events
                                </th>
                            </tr>
                            </thead>

                            <thead>
                            <tr>
                                <th>Title</th>
                                <th>Event Date</th>
                                <th>Attendees</th>
                                <th>Expected</th>
                            </tr>
                            </thead>

                            <tbody>
                            { attendanceExtremes.most.map((item, index) => (
                                <tr key={index}>
                                    <td>{ item.title }</td>
                                    <td>{ item.event_date }</td>
                                    <td>{ item.attendees }</td>
                                    <td>{ item.tickets_sold }</td>
                                </tr>
                            )) }
                            </tbody>
                        </table>

                        <table>
                            <colgroup>
                                <col style={{ width: "40%" }} />
                                <col style={{ width: "30%" }} />
                                <col style={{ width: "15%" }} />
                                <col style={{ width: "15%" }} />
                            </colgroup>

                            <thead>
                            <tr>
                                <th className="title least" colSpan={ 4 }>
                                    <MdOutlineEventNote className="icon"/>Least Attended Events
                                </th>
                            </tr>
                            </thead>

                            <thead>
                            <tr>
                                <th>Title</th>
                                <th>Event Date</th>
                                <th>Attendees</th>
                                <th>Expected</th>
                            </tr>
                            </thead>

                            <tbody>
                            { attendanceExtremes.least.map((item, index) => (
                                <tr key={index}>
                                    <td>{ item.title }</td>
                                    <td>{ item.event_date }</td>
                                    <td>{ item.attendees }</td>
                                    <td>{ item.tickets_sold }</td>
                                </tr>
                            )) }
                            </tbody>
                        </table>
                    </div>

                    <div className="overviewItem">
                        <div className="info">
                            <h2>
                                No Show Rates
                            </h2>

                            { /* Display total formatted */ }
                            <h1>
                                { noShowMonthlyOverview.totalRate.toLocaleString(undefined, 0) }%
                            </h1>

                            <p>
                                <span className={ !isNoShowIncrease ? "increase" : "decrease" }>
                                    { /* Show up/down arrow or infinity if last year total is zero */ }
                                    { isLastYearNoShowZero ? (
                                        <>
                                            <FaArrowUpLong className="icon" />
                                            <FaInfinity className="icon" />
                                        </>
                                    ) : (
                                        isNoShowIncrease
                                            ? <FaArrowUpLong className="icon" />
                                            : <FaArrowDownLong className="icon" />
                                    )}
                                    { noShowChange }%
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
                                    data: noShowMonthlyOverview.seriesData,
                                    showMark: false,
                                    valueFormatter: (value) =>
                                        `${ value.toLocaleString(undefined, {
                                        minimumFractionDigits: 2,
                                        maximumFractionDigits: 2,
                                    }) }%`,
                                },
                            ]}
                            xAxis={[
                                {
                                    scaleType: "point",
                                    data: noShowMonthlyOverview.xAxisData,
                                    valueFormatter: (value) => value.toLocaleString("default", { month: "short" }),
                                },
                            ]}
                            yAxis={[
                                {
                                    width: 75,
                                    position: "left",
                                    min: 0,
                                    valueFormatter: (value) => `${ value }%`,
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

                    { /* TODO Attendee basic information */ }
                </div>
            </div>
        </div>
    )
}


export default Attendees;