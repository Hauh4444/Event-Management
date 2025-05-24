// External Libraries
import { createTheme, ThemeProvider } from  "@mui/material/styles";

// Internal Modules
import PublicRoutes from "@/Routes/PublicRoutes.jsx";

// Stylesheets
import "./App.css";


const darkTheme = createTheme({ palette: { mode: "dark" } });

const lightTheme = createTheme({ palette: { mode: "light" } });


const App = () => {
    return (
      <ThemeProvider theme={ lightTheme }>
          <PublicRoutes />
      </ThemeProvider>
    )
}


export default App
