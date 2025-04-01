import { useEffect, useRef, useState } from "react";
import { useAuth } from "../context/auth";
import { useNavigate } from "react-router-dom";


interface LinkType {
	id: number,
	user_id: number,
	url: string,
	keywords: string,
}

const Dashboard: React.FC = () => {
	const { token, logout, authfetch } = useAuth();
	const navigate = useNavigate();

	const [searchkey, set_searchkey] = useState("");
	const [from, set_from] = useState("");
	const [links, set_links] = useState<Array<LinkType>>([]);

	useEffect(() => {
		authfetch(`http://localhost:8000/links/fetch/10`, {
			method: "GET"
		}).then(res => {
			if (res.status == 401 || !token) navigate("/login");
			if (!res.ok) {
				throw new Error("Data cant be fetched");
			}
			return res.json();
		}).then(data => {
			// set_userinfo(JSON.stringify(data));
			set_links(data);
		}).catch(err => {
			console.log(`${err}`);
		})
	}, []);

	const timeoutId = useRef<number | null>(null);
	const handle_search = () => {
		if (timeoutId.current) {
			clearTimeout(timeoutId.current);
		}
		timeoutId.current = setTimeout(() => {
			let fquery = from ? `&from=${from}` : "";
			authfetch(`http://localhost:8000/links/fetch?searchkey=${searchkey}${fquery}`, {
				method: "GET",
				headers: {
					"Content-Type": "application/json"
				}
			}).then(res => {
				if (!token || res.status == 401) {
					navigate("/login");
				}
				if (!res.ok) {
					throw new Error("Something wrong");
				}
				return res.json();
			}).then(data => {
				console.log(data);
				set_links(data);
			}).catch(err => {
				console.log(`${err}`);
			})
		}, 500);
	}

	return (
		<div>
			<div>
				<button onClick={() => {
					logout();
					navigate("/login");
				}}>Logout</button>
			</div>
			<div className="range">
				<input type="date" onChange={e => set_from(e.target.value)} />
			</div>
			<div className="searchbox">
				<input type="text" placeholder="Search" onChange={e => set_searchkey(e.target.value)} value={searchkey} onKeyUp={handle_search} />
			</div>

			<div>
				{links.map((link) => (
					<div key={link.id}>
						{link.id} - {link.url} - {link.user_id} - {link.keywords}
					</div>
				))}
			</div>
		</div>
	);
};

export default Dashboard;