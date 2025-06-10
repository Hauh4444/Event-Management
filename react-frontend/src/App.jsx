// Internal Modules
import CustomThemeProvider from "@/ContextAPI/Theme/ThemeProvider.jsx";
import PublicRoutes from "@/Routes/PublicRoutes.jsx";

// Stylesheets
import "./App.css";


const App = () => {
    return (
      <CustomThemeProvider>
          <PublicRoutes />
      </CustomThemeProvider>
    )
}


export default App
