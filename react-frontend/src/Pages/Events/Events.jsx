// External Libraries
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { Button } from "@mui/material";
import { areaElementClasses, LineChart, lineElementClasses } from "@mui/x-charts/LineChart";

// External Icons
import { AiOutlineExport } from "react-icons/ai";
import { FaArrowUpLong, FaArrowDownLong, FaInfinity } from "react-icons/fa6";
import { VscListFilter } from "react-icons/vsc";
import { MdOutlineEventNote } from "react-icons/md";
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
import "./Events.css";


/**
 * Events Page Component
 *
 * Displays an overview of event ticket sales, including a line chart visualization,
 * a searchable and paginated list of events, and functionality to export the overview
 * as a PDF document.
 *
 * @component
 * @returns { JSX.Element } The rendered Events page component.
 */
// TODO Register Event Functionality
const Events = () => {
    // React hooks
    const navigate = useNavigate();

    // State variables
    const [selectedYear, setSelectedYear] = useState(new Date().getFullYear());
    const [ticketsMonthlyOverview, setTicketsMonthlyOverview] = useState({
        seriesData: new Array(12).fill(0),
        profit: 0,
        lastYearTotal: 0,
    });
    const [dailyEventsOverview, setDailyEventsOverview] = useState([]);
    const [events, setEvents] = useState([]);
    const [filteredEvents, setFilteredEvents] = useState([]);
    const [page, setPage] = useState(1);
    const [query, setQuery] = useState("");
    
    // Derived constants
    const xAxisDates = Array.from({ length: 12 }, (_, monthIndex) =>
        new Date(new Date().getFullYear(), monthIndex, 1)
    );

    const perPage = 6;
    const paginatedEvents = filteredEvents.slice((page - 1) * perPage, page * perPage);
    const pageCount = Math.ceil(filteredEvents.length / perPage);

    const isLastYearZero = ticketsMonthlyOverview.lastYearTotal === 0;
    const salesChange = isLastYearZero
        ? ""
        : (((ticketsMonthlyOverview.profit - ticketsMonthlyOverview.lastYearTotal) /
            ticketsMonthlyOverview.lastYearTotal) * 100)
            .toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 });
    const isIncrease = isLastYearZero || (parseFloat(salesChange) > 0);


    /**
     * Fetches ticket overview and events data from API and updates state.
     * Uses Promise all to fetch current and previous year ticket data concurrently.
     *
     * @param { number } year - The year selected.
     *
     * @typedef { Object } TicketsOverviewRes
     * @property { number[] } tickets - Monthly ticket counts, one for each month (index 0 = Jan).
     * @property { number } profit - Total profit for the current year.
     *
     * @typedef { Object } EventItem
     * @property { string } title - Event title.
     * @property { string } event_date - Date of the event (ISO string).
     * @property { string } location - Location of the event.
     * @property { string } status - Status of the event (e.g., "upcoming", "completed").
     * @property { number } tickets_sold - Number of tickets sold for the event.
     * @property { number } max_attendees - Maximum number of attendees allowed.
     *
     * @typedef { Object.<string, number> } DailyEventsOverview
     * @description Maps date strings (YYYY-MM-DD) to number of events on that day.
     *
     * @typedef { Array<EventItem> } EventsResData
     *
     * @returns { Promise<void> }
     */
    const fetchData = async (year) => {
        // Fetch ticket overview data for current year and last year in parallel
        const [currentYearRes, previousYearRes] = await Promise.all([
            axiosInstance.get("/events/sales/", { params: { year: year } }),
            axiosInstance.get("/events/sales/", { params: { year: year - 1 } }),
        ]);
        // Update tickets overview state
        setTicketsMonthlyOverview({
            seriesData: currentYearRes.data.tickets || new Array(12).fill(0),
            profit: currentYearRes.data.profit,
            lastYearTotal: previousYearRes.data.profit,
        });

        // Fetch event overview data for current year
        const dailyEventsRes = await axiosInstance.get("/events/counts/daily/", {
            params: { year }
        });
        // Set event overview state with response data
        setDailyEventsOverview(dailyEventsRes.data["event_counts"]);

        // Fetch events data separately
        const eventsRes = await axiosInstance.get("/events/", {
            params: { year: year }
        });
        // Set events and filtered events states with response data
        setEvents(eventsRes.data);
        setFilteredEvents(eventsRes.data);
    };


    /**
     * Handles year selection change for event information.
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
     * Filters the list of events based on the current search query.
     *
     * This function normalizes the query by trimming whitespace and converting to lowercase,
     * then filters the original events array to include only those events where the title,
     * event date, location, or status fields contain the query substring.
     * If the query is empty after trimming, it resets the filtered events to the full list.
     *
     * The filtered results are saved to state, and the pagination page is reset to 1.
     */
    const filterEvents = () => {
        // Normalize the query by trimming whitespace and converting to lowercase
        const q = query.trim().toLowerCase();
        // If the query is empty, reset the filtered events to the full events list and reset page
        if (!q) {
            setFilteredEvents(events);
            setPage(1);
            return;
        }

        // Filter events where any relevant field includes the query substring (case-insensitive)
        const filtered = events.filter(event =>
            (event.title.toLowerCase().includes(q)) ||
            (event.event_date.toLowerCase().includes(q)) ||
            (event.location.toLowerCase().includes(q)) ||
            (event.status.toLowerCase().includes(q))
        );

        // Update filtered events state with the filtered array
        setFilteredEvents(filtered);
        // Reset pagination to the first page after filtering
        setPage(1);
    };


    // Component JSX
    return (
        <div className="eventsPage page">
            <Sidebar />

            <div className="mainPage">
                <TopNav />

                <div className="content">
                    { /* Header Section */ }
                    <div className="head">
                        <div>
                            <h1>Event Tracking</h1>
                            <p>
                                View, manage, and edit your events with ease—track trends and gain valuable insights at a glance.
                            </p>
                        </div>

                        <span>
                            { /* Select year to filter event data */ }
                            <YearPicker
                                startYear={ new Date().getFullYear() - 5 }
                                endYear={ new Date().getFullYear() + 5 }
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

                            { /* Export full content as PDF */ }
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

                    { /* Ticket Sales Overview */ }
                    <div className="overviewItem">
                        <div className="info">
                            <h2>Total Ticket Sales</h2>
                            <h1>
                                ${ ticketsMonthlyOverview.profit.toLocaleString(undefined,
                                { minimumFractionDigits: 2, maximumFractionDigits: 2 }
                                ) }
                            </h1>

                            { /* Year-over-year comparison */ }
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

                        { /* Line chart showing monthly ticket sales */ }
                        <LineChart
                            height={ 300 }
                            margin={{ left: 0, right: 35 }}
                            series={[
                                {
                                    area: true,
                                    data: ticketsMonthlyOverview.seriesData,
                                    showMark: false,
                                    valueFormatter: (value) =>
                                        `$${ value.toLocaleString(undefined, {
                                            minimumFractionDigits: 2,
                                            maximumFractionDigits: 2,
                                        }) }`,
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
                                        if (value >= 1_000_000) return `$${ (value / 1_000_000) }m`;
                                        else if (value >= 1_000) return `$${ (value / 1_000) }k`;
                                        else return `$${ value }`;
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

                    { /* Daily Events Heatmap */ }
                    <CalendarHeatmap
                        values={ dailyEventsOverview }
                        valueType="events"
                        scaleValues={ [2, 3, 4] }
                        startDate={ new Date(selectedYear, 0, 1) }
                        endDate={ new Date(selectedYear, 11, 31) }
                    />

                    { /* Event List Table */ }
                    <div className="events">
                        <table>
                            <colgroup>
                                <col style={{ width: "20%" }} />
                                <col style={{ width: "15%" }} />
                                <col style={{ width: "20%" }} />
                                <col style={{ width: "15%" }} />
                                <col style={{ width: "15%" }} />
                                <col style={{ width: "15%" }} />
                            </colgroup>

                            <thead>
                            <tr>
                                <th className="title">
                                    <MdOutlineEventNote className="icon"/>Events
                                </th>
                                <th>
                                    { /* Total number of events */ }
                                    <div className="numEvents">
                                        { filteredEvents.length } Events
                                    </div>
                                </th>
                                <th colSpan={ 2 }>
                                    { /* Search bar for event filtering */ }
                                    <SearchBar
                                        onChange={ (val) => setQuery(val) }
                                        value={ query }
                                        onClick={ () => filterEvents() }
                                    />
                                </th>
                                <th>
                                    { /* Button for more specific event filtering */ }
                                    { /* TODO Filter functionality */ }
                                    <Button className="headBtn">
                                        <VscListFilter className="icon"/>
                                        Filters
                                    </Button>
                                </th>
                                <th>
                                    { /* Button to download event data as CSV */ }
                                    <Button
                                        className="headBtn"
                                        onClick={ async () => {
                                            await handleDownloadCSV(events, "events")
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
                                <th>Title</th>
                                <th>Event Date</th>
                                <th>Location</th>
                                <th>Status</th>
                                <th>Tickets Sold</th>
                                <th>Actions</th>
                            </tr>
                            </thead>

                            <tbody>
                            { /* Render paginated list of events */ }
                            { paginatedEvents.map((item, index) => (
                                <tr key={ index }>
                                    <td>{ item.title }</td>
                                    <td>{ item.event_date }</td>
                                    <td>{ item.location }</td>
                                    { /* Capitalize first letter of status */ }
                                    <td>{ item.status.charAt(0).toUpperCase() + item.status.slice(1) }</td>
                                    <td>{ item.tickets_sold } / { item.max_attendees }</td>
                                    <td className="btns">
                                        <Button
                                            className="btn"
                                            onClick={ () => navigate(`/events/${ item.id }`) }
                                        >
                                            View
                                        </Button>
                                    </td>
                                </tr>
                            )) }

                            { /* Render empty rows to keep table height consistent */ }
                            { Array.from({ length: perPage - paginatedEvents.length }).map((_, index) => (
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
                </div>
            </div>
        </div>
    );
};


export default Events;
