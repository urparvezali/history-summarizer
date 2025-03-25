import { useAuth } from "../context/AuthContext";
import { useNavigate } from "react-router-dom";

const Dashboard: React.FC = () => {
	const { logout } = useAuth();
	const navigate = useNavigate();

	return (
		<div>
			<h2>Dashboard (Protected)</h2>
			<button onClick={() => { logout(); navigate("/"); }}>Logout</button>
		</div>
	);
};

export default Dashboard;
