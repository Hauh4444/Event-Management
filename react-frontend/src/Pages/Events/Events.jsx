// External Libraries
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { Button } from "@mui/material";
import { areaElementClasses, LineChart, lineElementClasses } from "@mui/x-charts/LineChart";
import { jsPDF as JsPDF } from "jspdf";
import html2canvas from "html2canvas";

// External Icons
import { AiOutlineExport } from "react-icons/ai";
import { FaArrowUpLong, FaArrowDownLong, FaInfinity } from "react-icons/fa6";
import { VscListFilter } from "react-icons/vsc";
import { MdOutlineEventNote } from "react-icons/md";

// Internal Modules
import Sidebar from "@/Components/Sidebar/Sidebar.jsx";
import TopNav from "@/Components/TopNav/TopNav.jsx";
import YearPicker from "@/Components/YearPicker/YearPicker.jsx";
import CalendarHeatmap from "@/Components/CalendarHeatmap/CalendarHeatmap.jsx";
import SearchBar from "@/Components/SearchBar/SearchBar.jsx";
import CustomPagination from "@/Components/Pagination/Pagination.jsx";
import axiosInstance from "@/API/axiosInstance.js";

// Stylesheets
import "./Events.css";


// TODO Register Event Functionality


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
const Events = () => {
    // React hooks
    const navigate = useNavigate();

    // State variables
    const [eventsOverview, setEventsOverview] = useState([]);
    const [ticketsOverview, setTicketsOverview] = useState({
        seriesData: new Array(12).fill(0),
        xAxisData: Array.from({ length: 12 }, (_, i) => new Date(2025, i, 1)),
        profit: 0,
        lastYearTotal: 0,
    });
    const [selectedYear, setSelectedYear] = useState(new Date().getFullYear());
    const [events, setEvents] = useState([]);
    const [filteredEvents, setFilteredEvents] = useState([]);
    const [page, setPage] = useState(1);
    const [query, setQuery] = useState("");

    // Derived constants
    const perPage = 6;
    const paginatedEvents = filteredEvents.slice((page - 1) * perPage, page * perPage);
    const pageCount = Math.ceil(filteredEvents.length / perPage);
    const isLastYearZero = ticketsOverview.lastYearTotal === 0;
    const salesChange = isLastYearZero
        ? ""
        : (((ticketsOverview.profit - ticketsOverview.lastYearTotal) / ticketsOverview.lastYearTotal) * 100)
            .toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 });
    const isIncrease = isLastYearZero || (parseFloat(salesChange) > 0);


    /**
     * Fetches ticket overview and events data from API and updates state.
     * Uses Promise all to fetch current and previous year ticket data concurrently.
     *
     * @typedef { Object } TicketsOverviewResponse
     * @property { number[] } tickets
     * @property { number } profit
     *
     * @typedef { Object } EventItem
     * @property { string } title
     * @property { string } event_date
     * @property { string } location
     * @property { string } status
     * @property { number } tickets_sold
     * @property { number } max_attendees
     *
     * @typedef { Array<EventItem> } EventsResponseData
     *
     * @returns { Promise<void> }
     */
    const fetchData = async (year) => {
        // Fetch event overview data for current year
        const res = await axiosInstance.get("/overview/events/", { params: { year } });
        // Set event overview state with response data
        setEventsOverview(res.data["event_counts"]);

        // Fetch ticket overview data for current year and last year in parallel
        const [currentYearResponse, previousYearResponse] = await Promise.all([
            axiosInstance.get("/overview/tickets/", { params: { year: year } }),
            axiosInstance.get("/overview/tickets/", { params: { year: year - 1 } }),
        ]);

        // Extract last year tickets array, fallback to empty array if undefined
        const lastYearTickets = previousYearResponse.data.tickets || [];
        // Sum all tickets sold last year
        const lastYearTotal = lastYearTickets.reduce((total, count) => total + count, 0);

        // Extract current year tickets monthly data, fallback to zero array if undefined
        const currentYearTickets = currentYearResponse.data.tickets || new Array(12).fill(0);

        // Generate month labels for x-axis for current year
        const xAxisDates = Array.from({ length: 12 }, (_, monthIndex) =>
            new Date(year, monthIndex, 1));

        // Update tickets overview state
        setTicketsOverview({
            seriesData: currentYearTickets,
            xAxisData: xAxisDates,
            profit: currentYearResponse.data.profit,
            lastYearTotal: lastYearTotal,
        });

        // Fetch events data separately
        const eventsResponse = await axiosInstance.get("/events/", {
            params: { year: year }
        });
        setEvents(eventsResponse.data);
        setFilteredEvents(eventsResponse.data);
    };


    /**
     * Handles year selection change for event information.
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


    /**
     * Exports the current content of the .content container as a PDF.
     * Uses html2canvas to capture the DOM content as a canvas,
     * then uses jsPDF to create a multipage PDF with the captured image.
     */
    const handleExportPDF = async () => {
        const input = document.querySelector(".content");
        if (!input) return;

        // Add class to body to indicate export in progress (for styling)
        document.body.classList.add("exporting");

        try {
            // Capture the content area to a high-resolution canvas
            const canvas = await html2canvas(input, {
                scale: 3,         // increase scale for higher resolution
                useCORS: true,    // allow cross-origin images
                allowTaint: true, // allow tainted canvas for cross-origin images
                letterRendering: true,
                backgroundColor: "#ffffff",
                ignoreElements: (element) => element.classList.contains("empty-row"), // ignore empty placeholder rows
            });

            const pdf = new JsPDF("p", "pt", "a4");
            const pdfWidth = pdf.internal.pageSize.getWidth();
            const pdfHeight = pdf.internal.pageSize.getHeight();
            const padding = 10;

            // Calculate image width and scale to fit page width with padding
            const imgWidth = pdfWidth - padding * 2;
            const scale = imgWidth / canvas.width;

            // Calculate height in pixels that fits on one PDF page after scaling
            const pageHeightPx = pdfHeight / scale;

            let yPos = 0;

            // Loop to create pages if content height exceeds one page
            while (yPos < canvas.height) {
                // Create a temporary canvas for the current page slice
                const pageCanvas = document.createElement("canvas");
                pageCanvas.width = canvas.width;
                pageCanvas.height = Math.min(pageHeightPx, canvas.height - yPos);

                const ctx = pageCanvas.getContext("2d");

                // Draw the relevant slice of the main canvas onto the page canvas
                ctx.drawImage(
                    canvas,
                    0, yPos,
                    canvas.width, pageCanvas.height,
                    0, 0,
                    canvas.width, pageCanvas.height
                );

                // Add the image slice to the PDF
                pdf.addImage(
                    pageCanvas.toDataURL("image/png"),
                    "PNG",
                    padding,
                    padding,
                    imgWidth,
                    pageCanvas.height * scale
                );

                yPos += pageCanvas.height;

                // Add a new page if there is more content remaining
                if (yPos < canvas.height) pdf.addPage();
            }

            // Save the generated PDF file
            pdf.save("events-overview.pdf");
        } catch (err) {
            console.error("PDF generation failed:", err);
        } finally {
            // Remove exporting class regardless of success or failure
            document.body.classList.remove("exporting");
        }
    };


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
                    <div className="head">
                        <div>
                            <h1>
                                Event Tracking
                            </h1>

                            <p>
                                View, manage, and edit your events with ease—track trends and gain valuable insights at a glance.
                            </p>
                        </div>

                        <span>
                            { /* Year select for event information */ }
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
                            <Button className="btn" onClick={ () => handleExportPDF() }>
                                <AiOutlineExport className="icon" />
                                Export
                            </Button>
                        </span>
                    </div>

                    <div className="overviewItem">
                        <div className="info">
                            <h2>
                                Total Ticket Sales
                            </h2>

                            { /* Display total profit formatted with 2 decimals */ }
                            <h1>
                                ${ ticketsOverview.profit.toLocaleString(undefined,
                                { minimumFractionDigits: 2, maximumFractionDigits: 2 }
                            ) }
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

                        { /* Line chart visualizing monthly ticket sales */ }
                        <LineChart
                            height={ 300 }
                            margin={{ left: 0, right: 35 }}
                            series={[
                                {
                                    area: true,
                                    data: ticketsOverview.seriesData,
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
                                    data: ticketsOverview.xAxisData,
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
                                    stroke: "#3b5faf",
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
                                    stroke: "rgba(53, 54, 52, 0.1)",
                                    strokeDasharray: "5 5",
                                },
                            }}
                        >
                            { /* Gradient fill for area chart */ }
                            <defs>
                                <linearGradient id="areaGradient" x1="0%" y1="0%" x2="0%" y2="100%">
                                    <stop offset="0%" stopColor="#3b5faf" stopOpacity="0.25" />
                                    <stop offset="100%" stopColor="#3b5faf" stopOpacity="0" />
                                </linearGradient>
                            </defs>
                        </LineChart>
                    </div>

                    <CalendarHeatmap
                        values={ eventsOverview }
                        startDate={ new Date(selectedYear, 0, 1) }
                        endDate={ new Date(selectedYear, 11, 31) }
                    />

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
                                    <MdOutlineEventNote className="titleIcon"/>Events
                                </th>
                                <th>
                                    { /* Total number of events */}
                                    <div className="numEvents">
                                        { filteredEvents.length } Events
                                    </div>
                                </th>
                                <th colSpan={3}>
                                    { /* Search bar for event filtering */}
                                    <SearchBar
                                        onChange={ (val) => setQuery(val) }
                                        value={ query }
                                        onClick={ () => filterEvents() }
                                    />
                                </th>
                                <th>
                                    { /* Button for more specific event filtering */ }
                                    { /* TODO Filter functionality */ }
                                    <Button className="filterBtn">
                                        <VscListFilter className="icon" style={{ marginRight: "10px" }}/>
                                        Filters
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
                            { paginatedEvents.map((item, index) => (
                                <tr key={ index }>
                                    <td>{ item.title }</td>
                                    <td>{ item.event_date }</td>
                                    <td>{ item.location }</td>
                                    { /* Capitalize first letter of status */}
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

                            { /* Render empty rows to keep table height consistent */}
                            { Array.from({ length: perPage - paginatedEvents.length }).map((_, index) => (
                                <tr key={`placeholder-${ index }`}>
                                    <td colSpan="6" className="empty-row">&nbsp;</td>
                                </tr>
                            )) }
                            </tbody>

                            <tfoot>
                            { /* Render pagination */}
                            <tr>
                                <td colSpan="6">
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
