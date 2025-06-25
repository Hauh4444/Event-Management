// External Libraries
import { useEffect, useState } from "react";
import { Button } from "@mui/material";
import { areaElementClasses, LineChart, lineElementClasses } from "@mui/x-charts/LineChart";

// External Icons
import { AiOutlineExport } from "react-icons/ai";
import { FaArrowDownLong, FaArrowUpLong, FaInfinity } from "react-icons/fa6";
import { PiCaretLeftBold, PiCaretRightBold } from "react-icons/pi";
import { MdOutlineEventNote } from "react-icons/md";
import { VscListFilter } from "react-icons/vsc";
import { RiFileDownloadLine } from "react-icons/ri";

// Internal Components
import Sidebar from "@/Components/Sidebar/Sidebar.jsx";
import TopNav from "@/Components/TopNav/TopNav.jsx";
import YearPicker from "@/Components/YearPicker/YearPicker.jsx";
import CalendarHeatmap from "@/Components/CalendarHeatmap/CalendarHeatmap.jsx";
import SearchBar from "@/Components/SearchBar/SearchBar.jsx";
import CustomPagination from "@/Components/Pagination/Pagination.jsx";

// Internal Utilities
import axiosInstance from "@/API/axiosInstance.js";
import handleExportPDF from "@/Utils/exportPDF.js";
import handleDownloadCSV from "@/Utils/downloadCSV.js";

// Stylesheets
import "./Attendees.css";


/**
 * Attendees Page Component
 *
 * Provides a comprehensive overview of event attendees, including total attendance
 * metrics, no-show rates, year-over-year comparisons, visualizations (line charts and heatmaps),
 * and detailed, searchable, paginated attendee lists for selected events.
 * Supports exporting the overview as a PDF and downloading attendee data as CSV.
 *
 * @component
 * @returns { JSX.Element } The rendered Attendees page component.
 */
const Attendees = () => {
    // State variables
    const [selectedYear, setSelectedYear] = useState(new Date().getFullYear());
    const [attendeesMonthlyOverview, setAttendeesMonthlyOverview] = useState({
        seriesData: new Array(12).fill(0),
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
        totalCount: 0,
        totalRate: 0,
        lastYearTotal: 0,
        lastYearRate: 0,
    });
    const [generalTicketMonthlyOverview, setGeneralTicketMonthlyOverview] = useState({
        seriesData: new Array(12).fill(0),
    })
    const [studentTicketMonthlyOverview, setStudentTicketMonthlyOverview] = useState({
        seriesData: new Array(12).fill(0),
    })
    const [staffTicketMonthlyOverview, setStaffTicketMonthlyOverview] = useState({
        seriesData: new Array(12).fill(0),
    })
    const [vipTicketMonthlyOverview, setVipTicketMonthlyOverview] = useState({
        seriesData: new Array(12).fill(0),
    })
    const [events, setEvents] = useState([]);
    const [selectedEvent, setSelectedEvent] = useState(0);
    const [attendees, setAttendees] = useState([]);
    const [filteredAttendees, setFilteredAttendees] = useState([]);
    const [page, setPage] = useState(1);
    const [query, setQuery] = useState("");

    // Derived constants
    const xAxisDates = Array.from({ length: 12 }, (_, monthIndex) =>
        new Date(new Date().getFullYear(), monthIndex, 1)
    );

    const ticketTypes = [
        { className: "generalTicket", title: "General Tickets", data: generalTicketMonthlyOverview.seriesData },
        { className: "studentTicket", title: "Student Tickets", data: studentTicketMonthlyOverview.seriesData },
        { className: "staffTicket", title: "Staff Tickets", data: staffTicketMonthlyOverview.seriesData },
        { className: "vipTicket", title: "VIP Tickets", data: vipTicketMonthlyOverview.seriesData },
    ];

    const perPage = 6;
    const paginatedAttendees = filteredAttendees.slice((page - 1) * perPage, page * perPage);
    const pageCount = Math.ceil(filteredAttendees.length / perPage);

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
     * Fetches attendee data for selected event.
     * Resets query filters and pagination.
     *
     * @param { SelectedEvent } event - The event selected.
     * @param { number } year - The year selected.
     *
     * @typedef { Object } SelectedEvent
     * @property { number } id - Unique identifier of the event.
     * @property { string } title - Title of the event.
     * @property { string } event_date - ISO string representing the date of the event.
     * @property { string } location - Location of the event.
     * @property { string } status - Status of the event.
     * @property { number } tickets_sold - Number of tickets sold for this event.
     * @property { number } max_attendees - Max number of attendees allowed.
     *
     * @typedef { Object } Attendee
     * @property { number } id - Unique identifier of the attendee.
     * @property { string } name - Full name of the attendee.
     * @property { string } email - Email address of the attendee.
     * @property { string } ticket_type - Type of ticket purchased.
     * @property { string } registration_date - Date the attendee registered for a ticket.
     *
     * @typedef { Attendee[] } AttendeesResData
     *
     * @returns { Promise<void> }
     */
    const fetchSelectedEventData = async (event, year) => {
        // Fetch attendee data for selected event
        const attendeesRes = await axiosInstance.get(`/attendees/${ event.id }/`, {
            params: { year: year }
        });
        // Set attendees and filtered attendees states with response data
        setAttendees(attendeesRes.data);
        setFilteredAttendees(attendeesRes.data);
        // Reset query filters and pagination
        setQuery("");
        setPage(1);
    }


    /**
     * Fetches attendee overview data from API and updates state.
     * Uses Promise all to fetch current and previous year attendee data concurrently.
     *
     * @param { number } year - The year selected.
     *
     * @typedef { Object } MonthlyAttendeesRes
     * @property { number[] } attendees - Monthly attendee counts, one for each month (index 0 = Jan).
     * @property { number } total - Total number of attendees for the year.
     *
     * @typedef { Object.<string, number> } DailyAttendeesOverview
     * @description Maps date strings (YYYY-MM-DD) to number of attendees on that day.
     *
     * @typedef { Object } AttendanceExtremes
     * @property { EventItem } most_attended - Event with the highest attendance.
     * @property { EventItem } least_attended - Event with the lowest attendance.
     *
     * @typedef { Object } MonthlyNoShowRes
     * @property { number[] } no_show_rates - Monthly no-show rates (as percentages or ratios).
     * @property { number } total_count - Total number of no-shows for the year.
     * @property { number } total_rate - Overall no-show rate for the year.
     *
     * @typedef { Object } TicketTypeMonthlyCounts
     * @property { number[] } general_counts - Monthly counts of general ticket attendees (index 0 = Jan).
     * @property { number[] } student_counts - Monthly counts of student ticket attendees.
     * @property { number[] } staff_counts - Monthly counts of staff ticket attendees.
     * @property { number[] } vip_counts - Monthly counts of VIP ticket attendees.
     *
     * @returns { Promise<void> }
     */
    const fetchData = async (year) => {
        // Fetch monthly attendee counts for current year and previous year concurrently
        const [currentYearAttendanceRes, previousYearAttendanceRes] = await Promise.all([
            axiosInstance.get("/attendees/counts/monthly/", { params: { year: year } }),
            axiosInstance.get("/attendees/counts/monthly/", { params: { year: year - 1 } }),
        ]);
        // Update monthly attendees overview state with current year data and totals
        setAttendeesMonthlyOverview({
            seriesData: currentYearAttendanceRes.data.attendees || new Array(12).fill(0),
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
        // Update monthly no shows overview state with current year data and totals
        setNoShowMonthlyOverview({
            seriesData: currentYearNoShowRes.data.no_show_rates || new Array(12).fill(0),
            totalCount: currentYearNoShowRes.data.total_count,
            totalRate: currentYearNoShowRes.data.total_rate,
            lastYearTotal: previousYearNoShowRes.data.total_count,
            lastYearRate: previousYearNoShowRes.data.total_rate,
        });

        // Fetch attendee overview data by ticket type
        const attendeesByTicketRes = await axiosInstance.get("/attendees/", {
            params: { year: year }
        });
        // Set attendee overview by ticket type states
        setGeneralTicketMonthlyOverview({ seriesData: attendeesByTicketRes.data.general_counts });
        setStudentTicketMonthlyOverview({ seriesData: attendeesByTicketRes.data.student_counts });
        setStaffTicketMonthlyOverview({ seriesData: attendeesByTicketRes.data.staff_counts });
        setVipTicketMonthlyOverview({ seriesData: attendeesByTicketRes.data.vip_counts });

        // Fetch event data
        const eventsRes = await axiosInstance.get("/events/", {
            params: { year: year }
        });
        // Set events state with response data
        setEvents(eventsRes.data);

        // Fetch selected event data
        await fetchSelectedEventData(eventsRes.data[selectedEvent], year);
    }


    /**
     * Handles year selection change for attendee information.
     *
     * @param { number } year - The year selected.
     */
    const onYearChange = (year) => {
        // Update selected year state
        setSelectedYear(year);
        // Reset query filters and pagination
        setQuery("");
        setPage(1);

        // Fetch data for newly selected year
        fetchData(year).catch((err) => console.error(err));
    };


    /**
     * Handles fetching data on component mount.
     */
    useEffect(() => {
        fetchData(new Date().getFullYear()).catch((err) => console.error(err));
    }, []);


    /**
     * Filters the list of attendees based on the current search query.
     *
     * This function normalizes the query by trimming whitespace and converting to lowercase,
     * then filters the original attendees array to include only those attendees where the title,
     * event date, location, or status fields contain the query substring.
     * If the query is empty after trimming, it resets the filtered attendees to the full list.
     *
     * The filtered results are saved to state, and the pagination page is reset to 1.
     */
    const filterAttendees = () => {
        // Normalize the query by trimming whitespace and converting to lowercase
        const q = query.trim().toLowerCase();
        // If the query is empty, reset the filtered attendees to the full attendees list and reset page
        if (!q) {
            setFilteredAttendees(attendees);
            setPage(1);
            return;
        }

        // Filter attendees where any relevant field includes the query substring (case-insensitive)
        const filtered = attendees.filter(attendee =>
            (attendee.name.toLowerCase().includes(q)) ||
            (attendee.email.toLowerCase().includes(q)) ||
            (attendee.ticket_type.toLowerCase().includes(q))
        );

        // Update filtered attendees state with the filtered array
        setFilteredAttendees(filtered);
        // Reset pagination to the first page after filtering
        setPage(1);
    };


    // Component JSX
    return (
        <div className="attendeesPage page">
            <Sidebar />

            <div className="mainPage">
                <TopNav />

                <div className="content">
                    { /* Header Section */ }
                    <div className="head">
                        <div>
                            <h1>Attendee Tracking</h1>
                            <p>
                                Track attendee growth and trends year over year—gain clear, visual insights into your events'
                                performance.
                            </p>
                        </div>

                        <span>
                            { /* Select year to filter attendee data */ }
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

                            { /* Export the full content as a PDF */ }
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

                    { /* Total Attendees Overview */ }
                    { /* TODO Projection information */ }
                    <div className="overviewItem">
                        <div className="info">
                            <h2>Total Attendees</h2>
                            <h1>{ attendeesMonthlyOverview.total.toLocaleString(undefined, 0) }</h1>

                            { /* Attendance comparison with last year */ }
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

                        { /* Monthly Attendees Line Chart */ }
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
                                    data: xAxisDates,
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

                    { /* Daily Attendee Calendar View */ }
                    <CalendarHeatmap
                        values={ dailyAttendeesOverview }
                        valueType="attendees"
                        scaleValues={ [60, 120, 210] }
                        startDate={ new Date(selectedYear, 0, 1) }
                        endDate={ new Date(selectedYear, 11, 31) }
                    />

                    { /* Tables for Most and Least Attended Events */ }
                    <div className="attendanceExtremes">
                        { /* Most Attended Events */ }
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

                        { /* Least Attended Events */ }
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

                    { /* No Show Rate Overview */ }
                    <div className="overviewItem">
                        <div className="info">
                            <h2>No Show Rates</h2>
                            <h1>{ noShowMonthlyOverview.totalRate.toLocaleString(undefined, 0) }%</h1>

                            { /* No Show Rate Comparison */ }
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

                        { /* Monthly No Show Rate Line Chart */ }
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
                                    data: xAxisDates,
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

                    <div className="ticketTypeOverview">
                        <h2>Attendees by Ticket Type</h2>
                        <div className="info">
                            { ticketTypes.map((item, index) => (
                                <div key={ index } className={ item.className }>
                                    <h3>{ item.title }</h3>
                                    <LineChart
                                        height={ 300 }
                                        series={[
                                            {
                                                area: true,
                                                data: item.data,
                                                showMark: false,
                                                valueFormatter: (value) => `${value.toLocaleString(undefined, 0)}`,
                                            },
                                        ]}
                                        xAxis={[
                                            {
                                                scaleType: "point",
                                                data: xAxisDates,
                                                valueFormatter: (value) => value.toLocaleString("default", { month: "short" }),
                                            },
                                        ]}
                                        yAxis={[
                                            {
                                                width: 30,
                                                position: "left",
                                                min: 0,
                                                valueFormatter: (value) => {
                                                    if (value >= 1_000_000) return `${(value / 1_000_000)}m`;
                                                    else if (value >= 1_000) return `${(value / 1_000)}k`;
                                                    else return `${value}`;
                                                },
                                            },
                                        ]}
                                        grid={{ horizontal: true }}
                                        sx={{
                                            [`& .${areaElementClasses.root}`]: {
                                                fill: "url(#areaGradient)",
                                            },
                                            [`& .${lineElementClasses.root}`]: {
                                                stroke: "var(--mui-palette-primary-main)",
                                                strokeWidth: 3,
                                            },
                                            "& .MuiChartsAxis-line": { display: "none" },
                                            "& .MuiChartsAxis-tick": { display: "none" },
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
                                                <stop offset="0%" stopColor="var(--mui-palette-primary-main)" stopOpacity="0.25" />
                                                <stop offset="100%" stopColor="var(--mui-palette-primary-main)" stopOpacity="0" />
                                            </linearGradient>
                                        </defs>
                                    </LineChart>
                                </div>
                            )) }
                        </div>
                    </div>

                    { /* Selected Event Attendees */ }
                    { events[selectedEvent] && (
                        <div className="attendees">
                            { /* Event Title and Nav Buttons */ }
                            <div className="eventSelect">
                                <Button
                                    className="btn"
                                    onClick={ async () => {
                                        setSelectedEvent(selectedEvent - 1);
                                        await fetchSelectedEventData(events[selectedEvent - 1], selectedYear);
                                    } }
                                    disabled={ selectedEvent <= 0 }
                                >
                                    <PiCaretLeftBold />
                                </Button>

                                <h2>
                                    { events[selectedEvent].title }
                                </h2>

                                <Button
                                    className="btn"
                                    onClick={ async () => {
                                        setSelectedEvent(selectedEvent + 1);
                                        await fetchSelectedEventData(events[selectedEvent + 1], selectedYear);
                                    } }
                                    disabled={ selectedEvent >= events.length }
                                >
                                    <PiCaretRightBold />
                                </Button>
                            </div>

                            { /* Attendee Table */ }
                            <table>
                                <colgroup>
                                    <col style={{ width: "20%" }} />
                                    <col style={{ width: "20%" }} />
                                    <col style={{ width: "15%" }} />
                                    <col style={{ width: "15%" }} />
                                    <col style={{ width: "15%" }} />
                                    <col style={{ width: "15%" }} />
                                </colgroup>

                                <thead>
                                <tr>
                                    <th className="title">
                                        <MdOutlineEventNote className="icon"/>Attendees
                                    </th>
                                    <th>
                                        { /* Total number of attendees */ }
                                        <div className="numAttendees">
                                            { filteredAttendees.length } Attendees
                                        </div>
                                    </th>
                                    <th colSpan={ 2 }>
                                        { /* Search bar for attendee filtering */ }
                                        <SearchBar
                                            onChange={ (val) => setQuery(val) }
                                            value={ query }
                                            onClick={ () => filterAttendees() }
                                        />
                                    </th>
                                    <th>
                                        { /* Button for more specific attendee filtering */ }
                                        { /* TODO Filter functionality */ }
                                        <Button className="headBtn">
                                            <VscListFilter className="icon"/>
                                            Filters
                                        </Button>
                                    </th>
                                    <th>
                                        { /* Button to download attendee data as CSV */ }
                                        <Button
                                            className="headBtn"
                                            onClick={ async () => {
                                                await handleDownloadCSV(attendees, "attendees")
                                            } }
                                        >
                                            <RiFileDownloadLine className="icon"/>
                                            Download
                                        </Button>
                                    </th>
                                </tr>
                                </thead>

                                <thead>
                                <tr>
                                    <th>Name</th>
                                    <th colSpan={ 2 }>Email</th>
                                    <th>Ticket Type</th>
                                    <th>Registration Date</th>
                                    <th>Actions</th>
                                </tr>
                                </thead>

                                <tbody>
                                { paginatedAttendees.map((item, index) => (
                                    <tr key={ index }>
                                        <td>{ item.name }</td>
                                        <td colSpan={ 2 }>{ item.email }</td>
                                        <td>{ item.ticket_type }</td>
                                        <td>{ item.registration_date }</td>
                                        { /* TODO Navigate to view attendee page */ }
                                        <td className="btns">
                                            <Button className="btn">
                                                View
                                            </Button>
                                        </td>
                                    </tr>
                                )) }

                                { /* Render empty rows to keep table height consistent */ }
                                { Array.from({ length: perPage - paginatedAttendees.length }).map((_, index) => (
                                    <tr key={`placeholder-${ index }`}>
                                        <td colSpan={ 6 } className="empty-row">&nbsp;</td>
                                    </tr>
                                )) }
                                </tbody>

                                { /* Render pagination */ }
                                <tfoot>
                                <tr>
                                    <td colSpan={ 6 }>
                                        <CustomPagination
                                            pageCount={ pageCount > 0 ? pageCount : 1 }
                                            page={ page > 0 ? page : 1 }
                                            onChange={ (e, value) => setPage(value) }
                                        />
                                    </td>
                                </tr>
                                </tfoot>
                            </table>
                        </div>
                    ) }
                </div>
            </div>
        </div>
    );
}


export default Attendees;