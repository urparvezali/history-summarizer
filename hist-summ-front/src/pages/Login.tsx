import { useAuth } from "../context/AuthContext";
import { useNavigate } from "react-router-dom";

const Login: React.FC = () => {
	const { login } = useAuth();
	const navigate = useNavigate();

	const handleLogin = () => {
		// Simulating JWT response
		const fakeToken = "fake-jwt-token";
		login(fakeToken);
		navigate("/dashboard");
	};

	return (
		<div>
			<h2>Login</h2>
			<button onClick={handleLogin}>Login</button>
		</div>
	);
};

export default Login;
