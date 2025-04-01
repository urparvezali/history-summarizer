import { createContext, ReactNode, useContext, useState } from "react";
import { Navigate, Outlet } from "react-router-dom";


interface AuthContextType {
	id: string | null;
	token: string | null;
	login: (email: string, password: string) => Promise<boolean>;
	logout: () => void;
	authfetch: (url: string, options?: RequestInit) => Promise<Response>;
}

const AuthContext = createContext<AuthContextType | null>(null);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
	const [token, setToken] = useState<string | null>(localStorage.getItem("token"));
	const [id, setId] = useState<string | null>(localStorage.getItem("id"));

	const login = async (email: string, password: string): Promise<boolean> => {
		try {
			const res = await fetch("http://localhost:8000/users/login", {
				method: "POST",
				credentials: "include",
				headers: { "Content-Type": "application/json" },
				body: JSON.stringify({ email, password }),
			});

			if (!res.ok) return false;

			const data = await res.json();
			localStorage.setItem("token", data.token);
			localStorage.setItem("id", data.id);
			setToken(data.token);
			setId(data.id);
			console.log(data.id)
			return true;
		} catch (error) {
			console.error("Login Error:", error);
			return false;
		}
	};

	const logout = () => {
		localStorage.removeItem("token");
		fetch(`http://localhost:8000/users/logout`, {
			method: "POST",
			credentials: "include",
		}).then(res => {
			if (!res.ok) return null;
			res.ok
		});
		setToken(null);
		setId(null);
	};

	const refresh = async () => {
		return await fetch("http://localhost:8000/users/refresh", {
			method: "GET",
			credentials: "include",
		})
	};

	const authfetch = async (url: string, options: RequestInit = {}): Promise<Response> => {
		let res = await fetch(url, {
			...options,
			headers: {
				...options.headers,
				Authorization: `Bearer ${token}`
			}
		});

		if (res.status === 401) {
			let refresh_res = await refresh();
			if (!refresh_res.ok) {
				if (refresh_res.status == 401) {
					logout();
				}
				return refresh_res;
			}
			let data = await refresh_res.json();
			setToken(data.token);
			localStorage.setItem("token", data.token);
			setId(data.id);
			localStorage.setItem("id", data.id);

			res = await fetch(url, {
				...options,
				headers: {
					...options.headers,
					Authorization: `Bearer ${data.token}`
				}
			});
		}

		return res;
	};

	return (
		<AuthContext.Provider value={{ id, token, login, logout, authfetch }}>
			{children}
		</AuthContext.Provider>
	);
};

export const useAuth = () => {
	const auth = useContext(AuthContext);
	if (!auth) throw new Error("Auth not set");
	return auth;
};


export const Protected = () => {
	const { token } = useAuth();
	return token ? <Outlet /> : <Navigate to={"/login"} />
}