import { useState } from "react";
import * as styles from "./login-page.css";
import { LoginForm } from "./login-form";
import { RegisterForm } from "./register-form";

type AuthMode = "login" | "register";

export function LoginPage() {
    const [mode, setMode] = useState<AuthMode>("login");

    return (
        <div className={styles.container}>
            {mode === "login" ? (
                <LoginForm onRegisterClick={() => setMode("register")} />
            ) : (
                <RegisterForm onLoginClick={() => setMode("login")} />
            )}
        </div>
    );
}
