import * as styles from "@sv/fe/components/audiobooks.css";
import { useStore } from "@sv/fe/hooks/store";
import { convertISO8601ToSeconds } from "@sv/fe/lib/iso8601";
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
                    <ProgressBar
                        position={audiobook.position_iso}
                        duration={audiobook.duration_iso}
                    />
                    <div className={styles.title}>{audiobook.title}</div>
                    <div className={styles.subtitle}>
                        <>{audiobook.author}, {audiobook.year}</>
                    </div>
                </button>
            ))}
        </div>
    );
}

interface ProgressBarProps {
    position: string | null;
    duration: string;
}

function ProgressBar({ position, duration }: ProgressBarProps) {
    const width = (convertISO8601ToSeconds(position || "PT0S") / convertISO8601ToSeconds(duration)) * 100
    return (
        <div className={styles.progress}>
            <div
                className={styles.progressFill}
                style={{ width: `${width}%` }}
            />
        </div>
    );
}