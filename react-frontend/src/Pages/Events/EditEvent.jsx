// External Libraries
import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { Button, FormControl, InputLabel, MenuItem, Select, TextField } from "@mui/material";

// Internal Components
import Sidebar from "@/Components/Sidebar/Sidebar.jsx";
import TopNav from "@/Components/TopNav/TopNav.jsx";

// Internal Utilities
import axiosInstance from "@/API/axiosInstance.js";

// Stylesheets
import "./EditEvent.css";


/**
 * Edit Event Page Component
 *
 * Provides an interface for editing the details of an existing event.
 * The form includes fields for general information, scheduling, location,
 * contact details, ticketing, accessibility, and safety guidelines.
 * It fetches the current event data and event detail sections (agenda,
 * speakers, FAQs, and attachments) from the backend on component mount,
 * and allows users to submit changes which update the event information
 * via an API call.
 *
 * @component
 * @returns { JSX.Element } The rendered EditEvent page component.
 */
const EditEvent = () => {
    // React hooks
    const navigate = useNavigate();
    const { id: eventId } = useParams();

    // State variables
    // TODO Finish setting up updating of entire event: image, map_embed_url (image file picker)
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
        agenda: [],
        speakers: [],
        faqs: [],
        attachments: [],
    });
    const [categories, setCategories] = useState([]);


    /**
     * Navigate to event details page if event has concluded
     */
    useEffect(() => {
        // Calculate time left until start of event
        const eventDateTime = event ? new Date(`${ event.event_date }T${ event.start_time }`) : null;
        const timeLeftMs = eventDateTime - new Date();
        // Navigate to view event if event has started or concluded
        if (timeLeftMs <= 0) navigate(`/events/${ eventId }`);
    }, [event, eventId, navigate]);


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
     * @typedef { Object } Category
     * @property { number } id
     * @property { string } name
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

        // Fetch category data
        const categoriesResponse = await axiosInstance.get("/categories/");
        // Set category state with response data
        setCategories(categoriesResponse.data);
    }


    /**
     * Handles fetching data on component mount.
     */
    useEffect(() => {
        fetchData().catch((err) => console.error(err));
    }, []);


    /**
     * Handles changing data in form and setting event state.
     *
     * @param e
     */
    const handleChange = (e) => {
        const { name, value } = e.target;

        // Set event with new data parsing bool data as int
        setEvent(prev => ({
            ...prev,
            [name]: name === "is_virtual" ? parseInt(value) : value
        }));
    };


    /**
     * Handles submitting updated event.
     *
     * @param e
     */
    const handleSubmit = async (e) => {
        e.preventDefault();

        // PUT updated event to backend
        await axiosInstance.put(`/events/${ eventId }/`, event).catch((err) => console.error(err));

        // PUT updated event details to backend
        await axiosInstance.put(`/events/${ eventId }/details/`, eventDetails)
            .then(() => navigate(`/events/${ eventId }/`))
            .catch((err) => console.error(err));
    };


    // Component JSX
    return (
        <div className="editEventPage page">
            <Sidebar />

            <div className="mainPage">
                <TopNav />

                <div className="content">
                    { /* Page Header */ }
                    <h1>Edit Event</h1>

                    { /* Event Form */ }
                    <form className="editForm" onSubmit={ handleSubmit }>
                        { /* General Details */ }
                        <section>
                            <TextField
                                fullWidth
                                label="Title"
                                name="title"
                                id="title"
                                value={ event.title }
                                onChange={ handleChange }
                            />
                            <TextField
                                fullWidth
                                label="Description"
                                name="description"
                                id="description"
                                value={ event.description }
                                onChange={ handleChange }
                                multiline
                                rows={ 12 }
                            />
                        </section>

                        { /* Time and Location Details */ }
                        <section className="row">
                            <TextField
                                type="date"
                                label="Event Date"
                                name="event_date"
                                id="event_date"
                                value={ event.event_date }
                                onChange={ handleChange }
                            />
                            <TextField
                                type="time"
                                label="Start Time"
                                name="start_time"
                                id="start_time"
                                value={ event.start_time }
                                onChange={ handleChange }
                            />
                            <TextField
                                type="time"
                                label="End Time"
                                name="end_time"
                                id="end_time"
                                value={ event.end_time }
                                onChange={ handleChange }
                            />
                            <TextField
                                label="Location"
                                name="location"
                                id="location"
                                value={ event.location }
                                onChange={ handleChange }
                            />
                        </section>

                        { /* Contact Information */ }
                        <section className="row">
                            <TextField
                                label="Contact Email"
                                name="contact_email"
                                id="contact_email"
                                value={ event.contact_email }
                                onChange={ handleChange }
                            />
                            <TextField
                                label="Contact Phone"
                                name="contact_phone"
                                id="contact_phone"
                                value={ event.contact_phone }
                                onChange={ handleChange }
                            />
                        </section>

                        { /* Event Category and Status Information */ }
                        <section className="row">
                            { categories.length > 0 && (
                                <FormControl fullWidth>
                                    <InputLabel id="event_category">Category</InputLabel>
                                    <Select
                                        labelId="event_category"
                                        label="Category"
                                        name="category_id"
                                        id="category_id"
                                        value={ categories ? event.category_id : 0 }
                                        onChange={ handleChange }
                                        size="medium"
                                        variant="outlined"
                                    >
                                        { categories.map((category, index) => (
                                            <MenuItem value={ category.id } key={ index }>{ category.name }</MenuItem>
                                        )) }
                                    </Select>
                                </FormControl>
                            ) }

                            <FormControl fullWidth>
                                <InputLabel id="event_status">Status</InputLabel>
                                <Select
                                    labelId="event_status"
                                    label="Status"
                                    name="status"
                                    id="status"
                                    value={ event.status }
                                    onChange={ handleChange }
                                    size="medium"
                                    variant="outlined"
                                >
                                    <MenuItem value="upcoming">Upcoming</MenuItem>
                                    <MenuItem value="canceled">Canceled</MenuItem>
                                </Select>
                            </FormControl>
                        </section>

                        { /* Event Attending Details */ }
                        <section className="row">
                            <TextField
                                label="Ticket Price"
                                name="price"
                                id="price"
                                type="number"
                                value={ event.price }
                                onChange={ handleChange }
                            />
                            <TextField
                                label="Max Attendees"
                                name="max_attendees"
                                id="max_attendees"
                                type="number"
                                value={ event.max_attendees }
                                onChange={ handleChange }
                            />
                            <TextField
                                type="date"
                                label="Registration Deadline"
                                name="registration_deadline"
                                id="registration_deadline"
                                value={ event.registration_deadline }
                                onChange={ handleChange }
                            />
                            <FormControl fullWidth>
                                <InputLabel id="event_type">Type</InputLabel>
                                <Select
                                    labelId="event_type"
                                    label="Type"
                                    name="is_virtual"
                                    id="is_virtual"
                                    value={ event.is_virtual }
                                    onChange={ handleChange }
                                    size="medium"
                                    variant="outlined"
                                >
                                    <MenuItem value={ 1 }>Virtual</MenuItem>
                                    <MenuItem value={ 0 }>In Person</MenuItem>
                                </Select>
                            </FormControl>
                        </section>

                        { /* Other Information */ }
                        <section className="row otherInfo">
                            <TextField
                                fullWidth
                                label="Accessibility Info"
                                name="accessibility_info"
                                id="accessibility_info"
                                value={ event.accessibility_info || "" }
                                onChange={ handleChange }
                                multiline
                                rows={ 3 }
                            />
                            <TextField
                                fullWidth
                                label="Safety Guidelines"
                                name="safety_guidelines"
                                id="safety_guidelines"
                                value={ event.safety_guidelines || "" }
                                onChange={ handleChange }
                                multiline
                                rows={ 3 }
                            />
                        </section>

                        { /* TODO Setup edit event details */ }

                        { /* Submit Button */ }
                        <Button
                            type="submit"
                            className="btn"
                        >
                            Save Changes
                        </Button>
                    </form>
                </div>
            </div>
        </div>
    );
};


export default EditEvent;
