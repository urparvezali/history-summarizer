import { useAuth } from "../context/AuthContext";
import { useNavigate } from "react-router-dom";

const Profile: React.FC = () => {
	const { logout } = useAuth();
	const navigate = useNavigate();

	return (
		<div>
			<h2>Profile Page (Protected)</h2>
			<p>Welcome! This is your profile.</p>
			<button onClick={() => { logout(); navigate("/"); }}>Logout</button>
		</div>
	);
};

export default Profile;
