import { useNavigate } from "react-router-dom";

const Register: React.FC = () => {
	const navigate = useNavigate();

	return (
		<div>
			<h2>Register</h2>
			<button onClick={() => navigate("/login")}>Go to Login</button>
		</div>
	);
};

export default Register;
