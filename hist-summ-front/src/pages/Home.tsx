import { Link } from "react-router-dom";

const Home: React.FC = () => {
	return (
		<div>
			<h2>Welcome to the Blog</h2>
			<Link to="/login">Login</Link> | <Link to="/register">Register</Link>
		</div>
	);
};

export default Home;
