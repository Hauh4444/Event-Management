// External Libraries
import axios from "axios";


// Axios instance with base URL, JSON headers, and credentials for cookies/auth
const axiosInstance = axios.create({
    baseURL: import.meta.env.VITE_BACKEND_API_URL,
    headers: { "Content-Type": "application/json" },
    withCredentials: true,
});


export default axiosInstance;