import { useLogin } from "@sv/fe/hooks/user";
import { useState } from "react";
import * as styles from "./login-page.css";

export function LoginPage() {
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");

    const { mutate, isPending, isError, error } = useLogin();

    const handleLogin = (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        mutate({ email, password });
    };

    return (
        <div className={styles.container}>
            <form onSubmit={handleLogin} className={styles.form}>
                <h1 className={styles.title}>StoryVault</h1>

                {isError && (
                    <div className={styles.error}>
                        {error?.message || "Login failed. Please try again."}
                    </div>
                )}

                <div className={styles.inputGroup}>
                    <label htmlFor="email" className={styles.label}>
                        Email
                    </label>
                    <input
                        id="email"
                        type="email"
                        placeholder="Enter your email"
                        value={email}
                        onChange={(e) => setEmail(e.target.value)}
                        className={styles.input}
                        required
                        autoComplete="email"
                    />
                </div>

                <div className={styles.inputGroup}>
                    <label htmlFor="password" className={styles.label}>
                        Password
                    </label>
                    <input
                        id="password"
                        type="password"
                        placeholder="Enter your password"
                        value={password}
                        onChange={(e) => setPassword(e.target.value)}
                        className={styles.input}
                        required
                        autoComplete="current-password"
                    />
                </div>

                <button type="submit" disabled={isPending} className={styles.button}>
                    {isPending ? "Signing in..." : "Sign In"}
                </button>
            </form>
        </div>
    );
}
