import * as styles from "@sv/fe/components/layout.css";
import { Avatar } from "@sv/fe/components/avatar";
import type { PropsWithChildren } from "react";
import { Logo } from "@sv/fe/components/logo";
import { useLogout } from "@sv/fe/hooks/user";
import { Player } from "@sv/fe/components/player";
import type { User } from "@sv/fe/types/user";

interface Props extends PropsWithChildren {
    user: User;
}

export function Layout({ children, user }: Props) {
    const logout = useLogout();
    return (
        <div className={styles.container}>
            <header className={styles.header}>
                <div className={styles.center.header}>
                    <div className={styles.area.left}>
                        <Logo />
                    </div>
                    <div className={styles.area.right}>
                        <button
                            className={styles.logout}
                            onClick={() => logout.mutate()}
                        >
                            Logout
                        </button>
                        <Avatar
                            name={{
                                firstName: user.first_name,
                                lastName: user.last_name,
                            }}
                        />
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
