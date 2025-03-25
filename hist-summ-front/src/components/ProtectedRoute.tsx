import { Navigate } from "react-router-dom";
import { useAuth } from "../context/AuthContext";
import { JSX } from "react";

// Props for ProtectedRoute
interface ProtectedRouteProps {
	children: JSX.Element;
}

const ProtectedRoute: React.FC<ProtectedRouteProps> = ({ children }) => {
	const { token } = useAuth();
	return token ? children : <Navigate to="/login" />;
};

export default ProtectedRoute;
