import { useLogin } from "@sv/fe/hooks/user";
import { useState } from "react";

export function LoginPage() {
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");

    const { mutate, isPending } = useLogin();

    const handleLogin = (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        mutate({ email, password });
    };

    return (
        <form onSubmit={handleLogin}>
            <input
                type="text"
                placeholder="Email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
            />
            <input
                type="password"
                placeholder="Password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
            />
            <button type="submit" disabled={isPending}>
                Login
            </button>
        </form>
    );
}
