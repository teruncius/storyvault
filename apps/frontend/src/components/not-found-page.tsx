import { Link } from "react-router-dom";
import * as styles from "@sv/fe/components/not-found-page.css";

export function NotFoundPage() {
    return (
        <div className={styles.container}>
            <h1 className={styles.title}>404</h1>
            <p className={styles.subtitle}>
                Oops! The page you're looking for doesn't exist.
            </p>
            <Link to="/" className={styles.link}>
                Home
            </Link>
        </div>
    );
}
