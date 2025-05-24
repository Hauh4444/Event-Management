// External Libraries
import { useEffect, useState } from "react";
import { Navigate, Outlet } from "react-router-dom";

// Internal Modules
import { useAuth } from "@/ContextAPI/Auth/AuthContext.js";


const PrivateRoute = () => {
    const auth = useAuth();

    const [isAuthChecked, setIsAuthChecked] = useState(false);

    useEffect(() => {
        if (!isAuthChecked) {
            auth.checkAuthStatus().finally(() => {
                setIsAuthChecked(true);
            });
        }
    }, [auth, isAuthChecked]);

    if (!isAuthChecked) {
        return null;
    }

    if (!auth.user) return <Navigate to="/auth" />;

    return <Outlet/>;
};


export default PrivateRoute;
