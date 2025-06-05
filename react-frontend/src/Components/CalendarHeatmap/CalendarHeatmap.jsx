// External Libraries
import ReactHeatmap from "react-calendar-heatmap";
import { Tooltip } from "react-tooltip";

// Stylesheets
import "react-calendar-heatmap/dist/styles.css";
import "./CalendarHeatmap.css";


/**
 * Calendar Heatmap Component
 *
 * Renders a GitHub-style calendar heatmap for visualizing activity data over time.
 *
 * @param { Object } props
 * @param {Array<{ date: string, count: number }>} props.values - Array of objects with date and count.
 * @param { Date | string } props.startDate - Start date of the calendar.
 * @param { Date | string } props.endDate - End date of the calendar.
 * @param { Function } [props.onClick] - Callback when a day cell is clicked.
 * @param { Function } [props.tooltipDataAttr] - Custom tooltip content.
 *
 * @component
 * @returns { JSX.Element } A responsive heatmap grid displaying count-based data by date.
 */
const CalendarHeatmap = ({
    values,
    startDate,
    endDate,
    onClick = () => {},
    tooltipDataAttr = (value) =>
        value.date ? `${ value.date }: ${ value.count } events` : "",
}) => {
    return (
        <div className="calendarHeatmap">
            <ReactHeatmap
                startDate={ startDate }
                endDate={ endDate }
                values={ values }
                classForValue={ (value) => {
                    if (!value || value.count === 0) return "scale0";
                    if (value.count >= 15) return "scale4";
                    if (value.count >= 8) return "scale3";
                    if (value.count >= 3) return "scale2";
                    return "scale1";
                } }
                tooltipDataAttrs={(value) => {
                    if (!value || !value.date) return {};
                    return {
                        "data-tooltip-id": "calendar-tooltip",
                        "data-tooltip-content": tooltipDataAttr(value),
                    };
                }}
                onClick={ onClick }
                showWeekdayLabels={ true }
            />

            <Tooltip id="calendar-tooltip" />
        </div>
    );
};

export default CalendarHeatmap;
