
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import { AuthProvider, Protected } from "./context/auth";

import Home from "./pages/Home";
import Login from "./pages/Login";
import Register from "./pages/Register";
import Dashboard from "./pages/Dashboard";
// import Profile from "./pages/Profile";

const App: React.FC = () => {
	return (
		<AuthProvider>
			<Router>
				<Routes>
					<Route path="/" element={<Home />} />
					<Route path="/login" element={<Login />} />
					<Route path="/register" element={<Register />} />
					<Route path="/dashboard" element={<Dashboard />} />
					<Route element={<Protected />}>
						{/* <Route path="/profile" element={<Profile/>}/> */}
					</Route>
				</Routes>
			</Router>
		</AuthProvider>
	);
};

export default App;
