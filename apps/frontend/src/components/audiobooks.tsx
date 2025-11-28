import { AudiobookCover } from "@sv/fe/components/audiobook-cover";
import { convertISO8601ToSeconds } from "@sv/fe/lib/iso8601";
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
                        position={durations[audiobook.id] || "PT0S"}
                        duration={audiobook.durationIso}
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
    position: string | null;
    duration: string;
}

function ProgressBar({ position, duration }: ProgressBarProps) {
    const width =
        (convertISO8601ToSeconds(position || "PT0S") /
            convertISO8601ToSeconds(duration)) *
        100;
    return (
        <div className={styles.progress}>
            <div
                className={styles.progressFill}
                style={{ width: `${width}%` }}
            />
        </div>
    );
}
