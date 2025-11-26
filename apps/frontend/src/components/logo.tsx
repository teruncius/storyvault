import logo from "@sv/fe/assets/logo.webp";
import * as styles from "@sv/fe/components/logo.css";
import { Link } from "react-router-dom";

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
