import { useRegister } from "@sv/fe/hooks/user";
import { useState } from "react";
import * as styles from "@sv/fe/components/form.css";

interface RegisterFormProps {
    onLoginClick: () => void;
}

export function RegisterForm({ onLoginClick }: RegisterFormProps) {
    const [email, setEmail] = useState("");
    const [firstName, setFirstName] = useState("");
    const [lastName, setLastName] = useState("");
    const [password, setPassword] = useState("");
    const [confirmPassword, setConfirmPassword] = useState("");
    const [validationError, setValidationError] = useState<string | null>(null);

    const { mutate, isPending, isError, error } = useRegister();

    const handleRegister = (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        setValidationError(null);

        if (password !== confirmPassword) {
            setValidationError("Passwords do not match");
            return;
        }

        mutate(
            { email, password, firstName, lastName },
            {
                onSuccess: () => {
                    // Optionally redirect or show success message
                    // For now, let's switch to login or assume the user knows to login
                    onLoginClick();
                },
            },
        );
    };

    return (
        <form onSubmit={handleRegister} className={styles.form}>
            <h1 className={styles.title}>Create Account</h1>

            {(isError || validationError) && (
                <div className={styles.error}>
                    {validationError ||
                        error?.message ||
                        "Registration failed. Please try again."}
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
                <label htmlFor="firstName" className={styles.label}>
                    First Name
                </label>
                <input
                    id="firstName"
                    type="text"
                    placeholder="Enter your first name"
                    value={firstName}
                    onChange={(e) => setFirstName(e.target.value)}
                    className={styles.input}
                    required
                    autoComplete="given-name"
                />
            </div>

            <div className={styles.inputGroup}>
                <label htmlFor="lastName" className={styles.label}>
                    Last Name
                </label>
                <input
                    id="lastName"
                    type="text"
                    placeholder="Enter your last name"
                    value={lastName}
                    onChange={(e) => setLastName(e.target.value)}
                    className={styles.input}
                    required
                    autoComplete="family-name"
                />
            </div>

            <div className={styles.inputGroup}>
                <label htmlFor="password" className={styles.label}>
                    Password
                </label>
                <input
                    id="password"
                    type="password"
                    placeholder="Create a password"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                    className={styles.input}
                    required
                    autoComplete="new-password"
                />
            </div>

            <div className={styles.inputGroup}>
                <label htmlFor="confirmPassword" className={styles.label}>
                    Confirm Password
                </label>
                <input
                    id="confirmPassword"
                    type="password"
                    placeholder="Confirm your password"
                    value={confirmPassword}
                    onChange={(e) => setConfirmPassword(e.target.value)}
                    className={styles.input}
                    required
                    autoComplete="new-password"
                />
            </div>

            <button
                type="submit"
                disabled={isPending}
                className={styles.button}
            >
                {isPending ? "Creating Account..." : "Sign Up"}
            </button>

            <div className={styles.footer}>
                Already have an account?{" "}
                <button
                    type="button"
                    onClick={onLoginClick}
                    className={styles.link}
                >
                    Sign in
                </button>
            </div>
        </form>
    );
}
