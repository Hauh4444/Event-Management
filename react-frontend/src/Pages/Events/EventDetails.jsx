// External Libraries
import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { Button } from "@mui/material";

// External Icons
import {
    FaMapMarkerAlt,
    FaUsers,
    FaCalendarAlt,
    FaClock,
    FaUserTie,
    FaShareAlt,
    FaRegComments,
    FaFileAlt,
    FaQuestionCircle,
    FaUniversalAccess,
    FaShieldAlt,
    FaTags,
    FaRegClock,
} from "react-icons/fa";

// Internal Components
import Sidebar from "@/Components/Sidebar/Sidebar.jsx";
import TopNav from "@/Components/TopNav/TopNav.jsx";

// Internal Utilities
import axiosInstance from "@/API/axiosInstance.js";

// Stylesheets
import "./EventDetails.css";


/**
 * Event Details Page Component
 *
 * Displays detailed information about a single event, including event overview,
 * organizer details, agenda, speakers, FAQs, attachments, comments, and related events.
 * Also features a countdown timer, map embed, social sharing buttons, and navigation
 * to edit the event or related events.
 *
 * @component
 * @returns { JSX.Element } The rendered EventDetails page component.
 */
// TODO Display attendance and ticket sale information
const EventDetails = () => {
    // React hooks
    const navigate = useNavigate();
    const { id: eventId } = useParams();

    // State variables
    // TODO Display category and map embed
    const [event, setEvent] = useState({
        title: "",
        description: "",
        event_date: "",
        start_time: "",
        end_time: "",
        location: "",
        category_id: 0,
        status: "",
        organizer_id: 0,
        price: 0,
        tickets_sold: 0,
        attendees: 0,
        max_attendees: 0,
        contact_email: "",
        contact_phone: "",
        registration_deadline: "",
        is_virtual: 0,
        image: "",
        map_embed_url: "",
        accessibility_info: "",
        safety_guidelines: ""
    });
    const [eventDetails, setEventDetails] = useState({
        organizer: { name: "", logo: "", website: "" },
        agenda: [{ start_time: "", title: "", speaker: "" }],
        speakers: [{ name: "", bio: "", photo: "" }],
        faqs: [{ question: "", answer: "" }],
        attachments: [{ name: "", url: "" }],
        comments: [{ message: "" }],
        related_events: [],
    });

    // Countdown timer
    const eventDateTime = event ? new Date(`${ event.event_date }T${ event.start_time }`) : null;
    const timeLeftMs = eventDateTime - new Date();
    const daysLeft = Math.floor(timeLeftMs / (1000 * 60 * 60 * 24));
    const hoursLeft = Math.floor((timeLeftMs / (1000 * 60 * 60)) % 24);
    const minutesLeft = Math.floor((timeLeftMs / (1000 * 60)) % 60);
    const countdown = timeLeftMs > 0 ?
        `${ daysLeft }d ${ hoursLeft }h ${ minutesLeft }m left` :
        "Event started or concluded";


    /**
     * Fetches event detail data from API and updates states.
     *
     * @typedef { Object } Event
     * @property { number } id
     * @property { string } title
     * @property { string } description
     * @property { string } event_date
     * @property { string } location
     * @property { string } status
     * @property { number } tickets_sold
     * @property { number } max_attendees
     *
     * @typedef { Object } EventDetails
     * @property { string } organizer_name
     * @property { string } organizer_email
     * @property { string } contact_phone
     *
     * @returns { Promise<void> }
     */
    const fetchData = async () => {
        // Fetch event overview details based on event id parameter
        const eventResponse = await axiosInstance.get(`/events/${ eventId }/`);
        // Set event state with response data
        setEvent(eventResponse.data);

        // Fetch event organizer details based on event id parameter
        const eventDetailsResponse = await axiosInstance.get(`/events/${ eventId }/details/`);
        // Set event state with response data
        setEventDetails(eventDetailsResponse.data);
    }


    /**
     * Handles fetching data on component mount.
     */
    useEffect(() => {
        fetchData().catch((err) => console.error(err));
    }, []);


    /**
     * Helper function for date formatting.
     *
     * @param dateStr
     * @returns { string }
     */
    const formatDate = (dateStr) =>
        new Date(dateStr).toLocaleDateString(undefined, {
            year: "numeric",
            month: "long",
            day: "numeric",
        });


    /**
     * Helper function for time formatting.
     *
     * @param timeStr
     * @returns { string }
     */
    const formatTime = (timeStr) => {
        const [hours, minutes] = timeStr.split(":");
        const date = new Date();
        date.setHours(parseInt(hours), parseInt(minutes));
        return date.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit", hour12: true });
    };


    /**
     * Helper function for datetime formatting.
     *
     * @param dateTimeStr
     * @returns { string }
     */
    const formatDateTime = (dateTimeStr) => {
        const date = new Date(dateTimeStr);
        return date.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit", hour12: true });
    };


    /**
     * Helper function for handling canceling an event.
     */
    const handleCancel = async () => {
        // Set canceled event based on event state
        let canceledEvent = { ...event, status: "canceled" };

        // PUT updated event to backend
        await axiosInstance.put(`/events/${ eventId }/`, canceledEvent).catch((err) => console.error(err));

        // Fetch new canceled event data
        fetchData().catch((err) => console.error(err));
    }


    /**
     * Helper function for handling sharing an event.
     *
     * @param social
     */
    const handleShare = (social) => {
        // TODO Proper sharing of events
        alert("Shared to " + social);
    }


    // Component JSX
    return (
        <div className="eventDetailsPage page">
            <Sidebar />
            <div className="mainPage">
                <TopNav />
                <div className="content">
                    { event && (
                        <div className="event">
                            { /* Event Banner/Image */ }
                            { /* TODO Better banner styling */ }
                            { event.image && (
                                <div className="eventBanner">
                                    <img
                                        src={ `${ import.meta.env.VITE_BACKEND_STATIC_URL }/${ event.image }` }
                                        alt={ event.title }
                                    />
                                </div>
                            ) }

                            { /* Event Header Information */ }
                            <div
                                className="header"
                            >
                                <div className="titleRow">
                                    <h1 className="title">{ event.title }</h1>

                                    { timeLeftMs > 0 && (
                                        <div className="actions">
                                            <Button
                                                className="btn edit"
                                                onClick={ () => navigate(`/events/${ event.id }/edit`) }
                                            >
                                                Edit
                                            </Button>

                                            { event.status !== "canceled" && (
                                                <Button
                                                    className="btn cancel"
                                                    onClick={ () => handleCancel() }
                                                >
                                                    Cancel
                                                </Button>
                                            ) }
                                        </div>
                                    ) }
                                </div>

                                { /* Formatted Event Meta Information */ }
                                <p className="meta">
                                    <FaCalendarAlt /> { formatDate(event.event_date) } &nbsp;•&nbsp;
                                    <FaClock /> { formatTime(event.start_time) } - { formatTime(event.end_time) } &nbsp;•&nbsp;
                                    <FaMapMarkerAlt /> { event.location }
                                </p>

                                { /* Countdown to Event Start Time */ }
                                <p className="countdown">
                                    <FaRegClock /> { countdown }
                                </p>
                            </div>

                            <div className="body">
                                { /* Event Description */ }
                                <p className="description">
                                    { event.description.split('\n').map((line, idx) => (
                                        <span key={ idx }>
                                            { line }
                                            <br />
                                        </span>
                                    )) }
                                </p>

                                { /* Organizer Info */ }
                                <section className="organizerSection">
                                    <h2><FaUserTie /> Organizer</h2>

                                    <div className="organizerInfo">
                                        { eventDetails.organizer.logo && (
                                            <img
                                                className="organizerLogo"
                                                src={
                                                    `${ import.meta.env.VITE_BACKEND_STATIC_URL }/` +
                                                    `${ eventDetails.organizer.logo }`
                                                }
                                                alt={ eventDetails.organizer.name }
                                            />
                                        ) }

                                        <p>{ eventDetails.organizer.name }</p>

                                        { eventDetails.organizer.website && (
                                            <a href={ eventDetails.organizer.website } target="_blank" rel="noreferrer">
                                                Visit Website
                                            </a>
                                        ) }
                                    </div>
                                </section>

                                { /* Map or Directions */ }
                                { event.map_embed_url && (
                                    <section className="mapSection">
                                        <h2><FaMapMarkerAlt /> Location Map</h2>

                                        <iframe
                                            src={ event.map_embed_url }
                                            width="100%"
                                            height="300"
                                            style={{ border: 0 }}
                                            allowFullScreen=""
                                            loading="lazy"
                                            title="Event Location Map"
                                        ></iframe>
                                    </section>
                                ) }

                                { /* Event Details Grid */ }
                                <div className="infoGrid">
                                    <div className="item">
                                        <span className="label"><FaTags className="icon" /> Event Status: </span>
                                        <span>{ event.status.charAt(0).toUpperCase() + event.status.slice(1) }</span>
                                    </div>
                                    <div className="item">
                                        <span className="label"><FaUsers className="icon" /> Event Format: </span>
                                        <span>{ event.is_virtual ? "Virtual" : "In-Person" }</span>
                                    </div>
                                    <div className="item">
                                        <span className="label"><FaFileAlt className="icon" /> Ticket Price: </span>
                                        <span>${ event.price.toFixed(2) }</span>
                                    </div>
                                    <div className="item">
                                        <span className="label"><FaCalendarAlt className="icon" /> Registration Deadline: </span>
                                        <span>{ formatDate(event.registration_deadline) }</span>
                                    </div>
                                    <div className="item">
                                        <span className="label"><FaUsers className="icon" /> Max Attendees: </span>
                                        <span>{ event.max_attendees }</span>
                                    </div>
                                    <div className="item">
                                        <span className="label">Email: </span>
                                        <span><a href={ `mailto:${ event.contact_email }` }>{ event.contact_email }</a></span>
                                    </div>
                                    <div className="item">
                                        <span className="label">Phone: </span>
                                        <span><a href={ `tel:${ event.contact_phone }` }>{ event.contact_phone }</a></span>
                                    </div>
                                    <div className="item">
                                        <span className="label"><FaUniversalAccess className="icon" /> Accessibility Info: </span>
                                        <span>{ event.accessibility_info || "Not Provided" }</span>
                                    </div>
                                    <div className="item">
                                        <span className="label"><FaShieldAlt className="icon" /> Safety Guidelines: </span>
                                        <span>{ event.safety_guidelines || "Not Provided" }</span>
                                    </div>
                                </div>

                                { /* Agenda/Schedule */ }
                                { eventDetails.agenda && (
                                    <section className="agendaSection">
                                        <h2>Event Agenda</h2>

                                        <ul>
                                            { eventDetails.agenda.map((item, i) => (
                                                <li key={ i }>
                                                    <strong>
                                                        { formatDateTime(item.start_time) }:
                                                    </strong> { item.title } - { item.speaker }
                                                </li>
                                            )) }
                                        </ul>
                                    </section>
                                ) }

                                { /* Speakers */ }
                                { eventDetails.speakers && (
                                    <section className="speakersSection">
                                        <h2>Speakers</h2>

                                        <div className="speakersGrid">
                                            { eventDetails.speakers.map((speaker, i) => (
                                                <div key={ i } className="speakerCard">
                                                    { speaker.photo &&
                                                        <img
                                                            className="speakerImage"
                                                            src={
                                                                `${ import.meta.env.VITE_BACKEND_STATIC_URL }/` +
                                                                `${ speaker.photo }`
                                                            }
                                                            alt={ speaker.name }
                                                        />
                                                    }

                                                    <div className="speakerInfo">
                                                        <h4>{ speaker.name }</h4>
                                                        <p>{ speaker.bio }</p>
                                                    </div>
                                                </div>
                                            )) }
                                        </div>
                                    </section>
                                ) }

                                { /* FAQs */ }
                                { eventDetails.faqs && (
                                    <section className="faqSection">
                                        <h2><FaQuestionCircle /> FAQs</h2>

                                        { eventDetails.faqs.map((faq, i) => (
                                            <div key={ i } className="faqItem">
                                                <strong>{ faq.question }</strong>
                                                <p>{ faq.answer }</p>
                                            </div>
                                        )) }
                                    </section>
                                ) }

                                { /* Comments/Discussion */ }
                                <section className="commentsSection">
                                    <h2><FaRegComments /> Comments</h2>

                                    { eventDetails.comments.length === 0 ? (
                                        <p>No comments yet.</p>
                                    ) : (
                                        eventDetails.comments.map((comment, i) => (
                                            <div key={ i } className="comment">
                                                <p>{ comment.message }</p>
                                            </div>
                                        ))
                                    ) }
                                </section>

                                { /* Attachments/Resources */ }
                                { eventDetails.attachments && (
                                    <section className="attachmentsSection">
                                        <h2><FaFileAlt /> Resources</h2>

                                        <ul>
                                            { eventDetails.attachments.map((file, i) => (
                                                <li key={ i }>
                                                    <a href={ file.url } target="_blank" rel="noreferrer">
                                                        { file.name }
                                                    </a>
                                                </li>
                                            )) }
                                        </ul>
                                    </section>
                                ) }

                                { /* Social Sharing */ }
                                <section className="socialSection">
                                    <h2><FaShareAlt /> Share this event</h2>

                                    <div>
                                        <Button className="btn" onClick={ () => handleShare("facebook") }>
                                            Facebook
                                        </Button>
                                        <Button className="btn" onClick={ () => handleShare("twitter") }>
                                            Twitter
                                        </Button>
                                        <Button className="btn" onClick={ () => handleShare("linkedin") }>
                                            LinkedIn
                                        </Button>
                                    </div>
                                </section>

                                { /* Related Events */ }
                                { eventDetails.related_events.length > 0 && (
                                    <section className="relatedEventsSection">
                                        <h2>Related Events</h2>

                                        <ul>
                                            { eventDetails.related_events.map((rel, i) => (
                                                <li
                                                    key={ i }
                                                    onClick={ () => navigate(`/events/${ rel.id }`) }
                                                    style={{ cursor: "pointer" }}
                                                >
                                                    { rel.title } - { formatDate(rel.event_date) }
                                                </li>
                                            )) }
                                        </ul>
                                    </section>
                                ) }
                            </div>
                        </div>
                    ) }
                </div>
            </div>
        </div>
    );
};


export default EventDetails;
