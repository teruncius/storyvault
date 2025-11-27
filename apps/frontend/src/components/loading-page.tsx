import * as styles from "./loading-page.css";

export function LoadingPage() {
    return (
        <div className={styles.container}>
            <div className={styles.spinner} />
        </div>
    );
}