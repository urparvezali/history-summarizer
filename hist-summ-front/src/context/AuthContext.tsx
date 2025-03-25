import { createContext, useState, useContext, ReactNode } from "react";

// Define authentication context types
interface AuthContextType {
	token: string | null;
	login: (token: string) => void;
	logout: () => void;
}

// Create Auth Context
const AuthContext = createContext<AuthContextType | undefined>(undefined);

// AuthProvider Props
interface AuthProviderProps {
	children: ReactNode;
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
	const [token, setToken] = useState<string | null>(localStorage.getItem("token"));

	// Login function
	const login = (newToken: string) => {
		localStorage.setItem("token", newToken);
		setToken(newToken);
	};

	// Logout function
	const logout = () => {
		localStorage.removeItem("token");
		setToken(null);
	};

	return (
		<AuthContext.Provider value={{ token, login, logout }}>
			{children}
		</AuthContext.Provider>
	);
};

// Custom hook to use auth context
export const useAuth = (): AuthContextType => {
	const context = useContext(AuthContext);
	if (!context) {
		throw new Error("useAuth must be used within an AuthProvider");
	}
	return context;
};
