// External Libraries
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { Button, Card, CardContent, CardHeader, TextField } from "@mui/material";

// Internal Contexts
import { useAuth } from "@/ContextAPI/AuthContext.js";


const Login = () => {
    const navigate = useNavigate();

    const { login } = useAuth();

    const [info, setInfo] = useState({ username: "", password: "" });

    const formItems = [
        { label: "Username", name: "username", type: "text" },
        { label: "Password", name: "password", type: "password" },
    ]

    const handleSubmit = async (e) => {
        e.preventDefault();
        const success = await login(info);
        if (success) navigate("/dashboard");
    };

    return (
        <div className="page">
            <Card className="loginCard">
                <CardHeader className="head" title="Login" />

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
                            Login
                        </Button>
                    </form>
                </CardContent>
            </Card>
        </div>
    )
}


export default Login;