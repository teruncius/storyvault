import * as styles from "@storyvault/frontend/components/audiobooks.css";
import type { Audiobook } from "@storyvault/frontend/types/audiobook";

interface Props {
    audiobooks: Audiobook[];
}

export function Audiobooks(props: Props) {
    return (
        <div className={styles.container}>
            {props.audiobooks.map((audiobook) => (
                <div className={styles.tile} key={audiobook.id}>
                    <img
                        src={audiobook.cover_url}
                        alt={audiobook.title}
                        className={styles.image}
                    />
                    <div className={styles.title}>{audiobook.title}</div>
                </div>
            ))}
        </div>
    );
}
