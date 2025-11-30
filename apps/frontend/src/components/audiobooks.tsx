import { AudiobookCover } from "@sv/fe/components/audiobook-cover";
import { useStore } from "@sv/fe/hooks/store";
import * as styles from "@sv/fe/components/audiobooks.css";
import type { Audiobook } from "@sv/fe/types/audiobook";

interface Props {
    audiobooks: Audiobook[];
}

export function Audiobooks(props: Props) {
    const { play } = useStore();
    const { durations } = useStore();
    return (
        <div className={styles.container}>
            {props.audiobooks.map((audiobook) => (
                <button
                    className={styles.tile}
                    key={audiobook.id}
                    onClick={() => play(audiobook.id)}
                >
                    <AudiobookCover
                        coverUrl={audiobook.coverUrl}
                        title={audiobook.title}
                        width={200}
                    />
                    <ProgressBar
                        position={durations[audiobook.id] || 0}
                        duration={audiobook.runtimeSeconds}
                    />
                    <div className={styles.text}>
                        <div className={styles.title}>{audiobook.title}</div>
                        <div className={styles.subtitle}>
                            <>
                                {audiobook.authors.join(", ")} -{" "}
                                {audiobook.year}
                            </>
                        </div>
                    </div>
                </button>
            ))}
        </div>
    );
}

interface ProgressBarProps {
    position: number;
    duration: number;
}

function ProgressBar({ position, duration }: ProgressBarProps) {
    const width = Math.floor((position / duration) * 100);
    return (
        <div className={styles.progress}>
            <div
                className={styles.progressFill}
                style={{ width: `${width}%` }}
            />
        </div>
    );
}
