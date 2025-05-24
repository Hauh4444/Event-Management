// External Libraries
import { Route, Routes } from "react-router-dom";

// Internal Auth Modules
import AuthProvider from "@/ContextAPI/Auth/AuthProvider.jsx";
import PrivateRoute from "@/Routes/PrivateRoute.jsx";

// Internal Public Modules
import Authentication from "@/Pages/Authentication/Authentication.jsx";

// Internal Private Modules
import Analytics from "@/Pages/Analytics/Analytics.jsx";
import Attendees from "@/Pages/Attendees/Attendees.jsx";
import Dashboard from "@/Pages/Dashboard/Dashboard.jsx";
import Events from "@/Pages/Events/Events.jsx";
import Settings from "@/Pages/Settings/Settings.jsx";
import Support from "@/Pages/Support/Support.jsx";


const PublicRoutes = () => {

    return (
        <AuthProvider>
            <Routes>
                <Route element={ <PrivateRoute /> }>
                    <Route path="/analytics" element={ <Analytics /> } />
                    <Route path="/attendees" element={ <Attendees /> } />
                    <Route path="/dashboard" element={ <Dashboard /> } />
                    <Route path="/events" element={ <Events /> } />
                    <Route path="/settings" element={ <Settings /> } />
                    <Route path="/support" element={ <Support /> } />
                </Route>

                <Route path="/auth" element={ <Authentication /> } />
            </Routes>
        </AuthProvider>
    )
};

export default PublicRoutes;
