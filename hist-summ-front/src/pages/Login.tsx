import { useAuth } from "../context/AuthContext";
import { useNavigate } from "react-router-dom";
import { useState } from "react";

const Login: React.FC = () => {
	const { id, token, login } = useAuth();
	const navigate = useNavigate();
	const [email, setEmail] = useState("");
	const [password, setPassword] = useState("");

	const handle_login = async () => {
		let res = await login(email, password);
		if(res) navigate("/dashboard");
	};

	return (
		<div>
			<div className="box">
				<input type="email" onChange={e => setEmail(e.target.value)} value={email} />
			</div>
			<div className="box">
				<input type="password" onChange={e => setPassword(e.target.value)} value={password} />
			</div>
			<div className="button">
				<button onClick={handle_login}>Login</button>
			</div>
			{id} <br />
			{token}
		</div>
	);
};

export default Login;
