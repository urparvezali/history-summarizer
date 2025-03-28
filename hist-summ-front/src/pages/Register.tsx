import { useState } from "react";
import { useNavigate } from "react-router-dom";

const Register: React.FC = () => {
	const navigate = useNavigate();
	const [name, setName] = useState("");
	const [email, setEmail] = useState("");
	const [password, setPassword] = useState("");
	const handle_submit = async () => {
		let data = {
			email: email,
			password: password,
			name: name
		};
		try {
			await fetch("http://localhost:8000/users/signup", {
				method: "POST",
				headers: {
					"Content-Type": "application/json"
				},
				body: JSON.stringify(data)
			});
			console.log("user inserted");
			navigate("/login");
		} catch (error) {
			console.log("Something wrong");
		}
	}
	return (
		<div>
			<h2>Register</h2>
			<div className="box">
				<input type="text" onChange={e => setName(e.target.value)} value={name} />
			</div>
			<div className="box">
				<input type="email" onChange={e => setEmail(e.target.value)} value={email} />
			</div>
			<div className="box">
				<input type="password" onChange={e => setPassword(e.target.value)} value={password} />
			</div>
			<div className="button">
				<button onClick={handle_submit}>Register</button>
			</div>
		</div>
	);
};

export default Register;
