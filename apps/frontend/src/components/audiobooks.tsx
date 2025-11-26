import * as styles from "@sv/fe/components/audiobooks.css";
import { useStore } from "@sv/fe/hooks/store";
import type { Audiobook } from "@sv/fe/types/audiobook";

interface Props {
    audiobooks: Audiobook[];
}

export function Audiobooks(props: Props) {
    const { play } = useStore();
    return (
        <div className={styles.container}>
            {props.audiobooks.map((audiobook) => (
                <button
                    className={styles.tile}
                    key={audiobook.id}
                    onClick={() => play(audiobook.id)}
                >
                    <img
                        src={audiobook.cover_url}
                        alt={audiobook.title}
                        className={styles.image}
                    />
                    <div className={styles.title}>{audiobook.title}</div>
                    <div className={styles.subtitle}>
                        {audiobook.author}, {audiobook.year}
                    </div>
                </button>
            ))}
        </div>
    );
}
