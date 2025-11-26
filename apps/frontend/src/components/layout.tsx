import * as styles from "@storyvault/frontend/components/layout.css";
import { Avatar } from "@storyvault/frontend/components/avatar";
import { useAuth } from "@storyvault/frontend/hooks/user";
import type { PropsWithChildren } from "react";
import { Logo } from "@storyvault/frontend/components/logo";
import { useLogout } from "@storyvault/frontend/hooks/user";

export function Layout({ children }: PropsWithChildren) {
    const { data: user } = useAuth();
    const logout = useLogout();
    return (
        <div className={styles.container}>
            <header className={styles.header}>
                <div className={styles.center.header}>
                    <div className={styles.xxx.left}>
                        <Logo />
                    </div>
                    <div className={styles.xxx.right}>
                        {user && (
                            <>
                                <button
                                    className={styles.logout}
                                    onClick={() => logout.mutate()}
                                >
                                    Logout
                                </button>
                                <Avatar
                                    firstName={user.first_name}
                                    lastName={user.last_name}
                                />
                            </>
                        )}
                    </div>
                </div>
            </header>
            <main className={styles.main}>
                <div className={styles.center.main}>{children}</div>
            </main>
            <footer className={styles.footer}>
                <div className={styles.center.footer}></div>
            </footer>
        </div>
    );
}
