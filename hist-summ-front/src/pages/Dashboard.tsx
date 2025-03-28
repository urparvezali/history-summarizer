import { useEffect, useState } from "react";
import { useAuth } from "../context/AuthContext";
import { useNavigate } from "react-router-dom";

const Dashboard: React.FC = () => {
	const { id, token, logout, authfetch } = useAuth();
	const navigate = useNavigate();
	const [userinfo, set_userinfo] = useState("");

	useEffect(() => {
		authfetch(`http://localhost:8000/users/15104`, {
			method: "GET"
		}).then(res => {
			if (!res.ok) {
				throw new Error("Data cant be fetched");
			}
			return res.json();
		}).then(data => {
			set_userinfo(JSON.stringify(data));
		}).catch(err => {
			console.log(`${err}`);
		})
	}, []);

	return (
		<div>
			<h2>Dashboard (Protected)</h2>
			<button onClick={() => { logout(); navigate("/"); }}>Logout</button>
			<div>
				{userinfo}
			</div>
			{id} <br />
			{token}
		</div>
	);
};

export default Dashboard;