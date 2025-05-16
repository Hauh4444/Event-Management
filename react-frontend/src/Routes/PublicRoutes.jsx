// External Libraries
import { Route, Routes } from "react-router-dom";

// Internal Auth Modules
import AuthProvider from "@/ContextAPI/AuthProvider.jsx";
import PrivateRoute from "@/Routes/PrivateRoute.jsx";

// Internal Public Modules
import Login from "@/Pages/Login.jsx";

// Internal Private Modules
import Dashboard from "@/Pages/Dashboard.jsx";


const PublicRoutes = () => {

    return (
        <AuthProvider>
            <Routes>
                <Route element={ <PrivateRoute /> }>
                    <Route path="/dashboard" element={ <Dashboard /> } />
                </Route>

                <Route path="/login" element={ <Login /> } />
            </Routes>
        </AuthProvider>
    )
};

export default PublicRoutes;
