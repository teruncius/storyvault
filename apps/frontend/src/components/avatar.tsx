import * as styles from "@sv/fe/components/avatar.css";

interface Props {
    name: { firstName: string; lastName: string };
}

export function Avatar({ name }: Props) {
    return (
        <div className={styles.avatar}>
            <>{name.firstName[0]}</>
            <>{name.lastName[0]}</>
        </div>
    );
}
