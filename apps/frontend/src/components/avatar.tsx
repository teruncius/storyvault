import * as styles from "@storyvault/frontend/components/avatar.css";

interface Props {
    firstName: string;
    lastName: string;
}

export function Avatar({ firstName, lastName }: Props) {
    return (
        <div className={styles.avatar}>
            <>{firstName[0]}</>
            <>{lastName[0]}</>
        </div>
    );
}
