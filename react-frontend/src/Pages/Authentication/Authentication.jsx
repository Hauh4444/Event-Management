// External Libraries
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { Button, Card, CardContent, CardHeader, TextField } from "@mui/material";

// Internal Contexts
import { useAuth } from "@/ContextAPI/Auth/AuthContext.js";

// Internal Assets
import cityscape from "@/assets/images/cityscape.jpeg";
import momentix from "@/assets/images/momentix.png";

// Stylesheets
import "./Authentication.css";


/**
 * Authentication Page component.
 *
 * Provides user authentication UI for sign-in and sign-up workflows.
 * Handles user input state, invokes authentication context methods,
 * and redirects on successful login or registration.
 *
 * @component
 * @returns {JSX.Element} The rendered Authentication page component.
 */
const Authentication = () => {
    // React hooks
    const navigate = useNavigate();
    const { login, register } = useAuth();

    // State variables
    const [info, setInfo] = useState({ username: "", password: "" });
    const [isSignIn, setIsSignIn] = useState(true);

    // Fields to render in the form dynamically
    const formItems = [
        { label: "Username", name: "username", type: "text" },
        { label: "Password", name: "password", type: "password" },
    ];


    /**
     * Handles submission of sign-in or sign-up form.
     * Calls appropriate context method and redirects on success.
     */
    const handleSubmit = async (e) => {
        e.preventDefault();

        // Call login or register depending on current mode
        const success = isSignIn ? await login(info) : await register(info);

        if (success) navigate("/dashboard");  // Redirect on successful auth
    };


    // Component JSX
    return (
        <div className="authPage page">
            <div className="auth">
                <span className="logo">
                    <img src={ momentix } alt="Momentix" />
                    <h1>Momentix</h1>
                </span>

                <Card className="card">
                    <CardHeader
                        className="head"
                        title={ isSignIn ? "Sign In to Momentix" : "Sign Up to Momentix" }
                        slotProps={{
                            title: {
                                fontWeight: "bold"
                            }
                        }}
                    />

                    <CardContent className="content">
                        <form
                            className="form"
                            onSubmit={ handleSubmit }
                        >
                            {formItems.map(({ label, name, type }, index) => (
                                <TextField
                                    className="input"
                                    key={ index }
                                    label={ label }
                                    name={ name }
                                    type={ type }
                                    variant="outlined"
                                    onChange={ (e) => setInfo(prev => ({ ...prev, [name]: e.target.value })) }
                                    fullWidth
                                />
                            ))}

                            <Button className="btn" type="submit">
                                { isSignIn ? "Sign In" : "Sign Up" }
                            </Button>
                        </form>
                    </CardContent>
                </Card>

                <span className="switch">
                    <p>
                        { isSignIn ? "Don't have an account?" : "Have an account?" }
                    </p>

                    <Button
                        className="btn"
                        onClick={ () => setIsSignIn(!isSignIn) }
                    >
                        { isSignIn ? "Sign Up" : "Sign In" }
                    </Button>
                </span>
            </div>

            <img src={ cityscape } alt="Cityscape" />
        </div>
    );
}


export default Authentication;
