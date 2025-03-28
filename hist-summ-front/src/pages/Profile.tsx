import { useEffect } from "react";
import { useAuth } from "../context/AuthContext";
import { useNavigate } from "react-router-dom";

const Profile: React.FC = () => {
	const { token, authfetch, logout } = useAuth();
	const navigate = useNavigate();
	useEffect(() => {
		authfetch(`localhost:8000/users/`, {
			method: "GET",
			headers: {
				Authorization: `Bearer ${token}`
			}
		})
	}, []);
	return (
		<div>
			<h2>Profile Page (Protected)</h2>
			<p>Welcome! This is your profile.</p>
			<button onClick={() => { logout(); navigate("/"); }}>Logout</button>
		</div>
	);
};

export default Profile;
