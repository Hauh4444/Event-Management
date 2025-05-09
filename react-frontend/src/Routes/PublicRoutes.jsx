// External Libraries
import { Route, Routes } from "react-router-dom";

// Public Internal Modules
import Home from "@/Pages/Home.jsx";


const PublicRoutes = () => {

    return (
        <Routes>
            <Route path="/" element={ <Home /> } />
        </Routes>
    )
};

export default PublicRoutes;
