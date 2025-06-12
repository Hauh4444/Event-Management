// Internal Modules
import CustomThemeProvider from "@/ContextAPI/Theme/ThemeProvider.jsx";
import PublicRoutes from "@/Routes/PublicRoutes.jsx";

// Stylesheets
import "./App.css";


// TODO rgba(53, 54, 52, alpha) needs to be replaced with theme colors
const App = () => {
    return (
      <CustomThemeProvider>
          <PublicRoutes />
      </CustomThemeProvider>
    )
}


export default App
