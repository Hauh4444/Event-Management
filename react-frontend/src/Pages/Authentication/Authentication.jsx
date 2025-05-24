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


const Authentication = () => {
    const navigate = useNavigate();

    const { login, register } = useAuth();

    const [info, setInfo] = useState({ username: "", password: "" });
    const [isSignIn, setIsSignIn] = useState(true);

    const formItems = [
        { label: "Username", name: "username", type: "text" },
        { label: "Password", name: "password", type: "password" },
    ]

    const handleSubmit = async (e) => {
        e.preventDefault();

        let success;

        if (isSignIn) success = await login(info);
        else success = await register(info);

        if (success) navigate("/dashboard");
    };

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
                            onSubmit={ (e) => handleSubmit(e) }
                        >
                            { formItems.map((item, index) => (
                                <TextField
                                    className="input"
                                    key={ index }
                                    label={ item.label }
                                    name={ item.name }
                                    type={ item.type }
                                    variant="outlined"
                                    onChange={ (e) => setInfo({ ...info, [e.target.name]: e.target.value }) }
                                    fullWidth
                                />
                            )) }

                            <Button
                                className="btn"
                                type="submit"
                            >
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
    )
}


export default Authentication;