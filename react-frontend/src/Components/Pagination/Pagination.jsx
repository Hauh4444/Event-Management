// External Libraries
import { Button, Pagination, PaginationItem } from "@mui/material";

// External Icons
import { FaArrowLeft, FaArrowRight } from "react-icons/fa6";

// Stylesheets
import "./Pagination.css";


/**
 * CustomPagination component.
 *
 * Displays pagination controls with previous and next buttons,
 * along with numbered page buttons for direct navigation.
 *
 * @param {Object} props - Component props.
 * @param {number} props.pageCount - Total number of pages available.
 * @param {number} props.page - Current active page number.
 * @param {(event: React.ChangeEvent<unknown>, page: number) => void} props.onChange - Callback fired when page changes.
 *
 * @component
 * @returns {JSX.Element} The rendered pagination component.
 */
const CustomPagination = ({ pageCount, page, onChange }) => {
    // Component JSX
    return (
        <div className="pagination">
            <Button
                className="btn"
                disabled={ page === 1 }
                onClick={ () => onChange(null, page - 1) }
            >
                <FaArrowLeft className="icon" style={{ marginRight: "10px" }}/> Previous
            </Button>

            <Pagination
                count={ pageCount }
                page={ page }
                onChange={ onChange }
                color="primary"
                sx={{
                    "& .MuiPaginationItem-page": {
                        width: "40px",
                        height: "40px",
                        margin: "0 5px",
                        borderRadius: "5px",
                        "&:hover, &.Mui-selected, &.Mui-selected:hover": {
                            backgroundColor: "#ebf0fa",
                            color: "#3b5faf",
                        },
                    },
                }}
                renderItem={ (item) => (
                    item.type === 'previous' || item.type === 'next'
                        ? null
                        : <PaginationItem {...item} />
                ) }
            />

            <Button
                className="btn"
                disabled={ page === pageCount }
                onClick={ () => onChange(null, page + 1) }
            >
                Next <FaArrowRight className="icon" style={{ marginLeft: "10px" }}/>
            </Button>
        </div>
    )
};


export default CustomPagination;
