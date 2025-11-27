import * as styles from "@sv/fe/components/layout.css";
import { Avatar } from "@sv/fe/components/avatar";
import type { PropsWithChildren } from "react";
import { Logo } from "@sv/fe/components/logo";
import { useLogout } from "@sv/fe/hooks/user";
import { Player } from "@sv/fe/components/player";
import type { User } from "@sv/fe/types/user";
import { Link } from "react-router-dom";
import { useTheme } from "@sv/fe/components/theme";

interface Props extends PropsWithChildren {
    user: User;
}

export function Layout({ children, user }: Props) {
    const logout = useLogout();
    const { toggle, icon } = useTheme();
    return (
        <div className={styles.container}>
            <header className={styles.header}>
                <div className={styles.center.header}>
                    <div className={styles.area.left}>
                        <Logo />
                        <nav className={styles.nav}>
                            <Link to="/" className={styles.navLink}>
                                Home
                            </Link>
                            <Link to="/problems" className={styles.navLink}>
                                Problems
                            </Link>
                        </nav>
                    </div>
                    <div className={styles.area.right}>
                        <div className={styles.userInfo}>
                            <div className={styles.avatar}>
                                <Avatar name={{
                                    firstName: user.first_name,
                                    lastName: user.last_name,
                                }} />
                            </div>
                            <div className={styles.userName}>
                                {user.first_name} {user.last_name}
                            </div>
                            <div className={styles.userEmail}>{user.email}</div>
                        </div>
                        <button
                            className={styles.toggleTheme}
                            onClick={toggle}
                        >
                            {icon}
                        </button>
                        <button
                            className={styles.logout}
                            onClick={() => logout.mutate()}
                        >
                            Logout
                        </button>
                    </div>
                </div>
            </header>
            <main className={styles.main}>
                <div className={styles.center.main}>{children}</div>
            </main>
            <footer className={styles.footer}>
                <div className={styles.center.footer}>
                    <Player />
                </div>
            </footer>
        </div>
    );
}
