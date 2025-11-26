import logo from "@storyvault/frontend/assets/logo.webp";
import * as styles from "@storyvault/frontend/components/logo.css";
import { Link } from "react-router";

export function Logo() {
    return (
        <Link className={styles.link} to="/">
            <img
                className={styles.logo}
                src={logo}
                alt="StoryVault"
                width={32}
                height={32}
            />
        </Link>
    );
}
