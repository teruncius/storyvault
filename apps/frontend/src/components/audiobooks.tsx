import { AudiobookCover } from "@sv/fe/components/audiobook-cover";
import { useStore } from "@sv/fe/hooks/store";
import { useDeletePosition } from "@sv/fe/hooks/position";
import * as styles from "@sv/fe/components/audiobooks.css";
import type { Audiobook } from "@sv/fe/types/audiobook";
import { useState } from "react";

interface Props {
    audiobooks: Audiobook[];
}

export function Audiobooks(props: Props) {
    return (
        <div className={styles.container}>
            {props.audiobooks.map((audiobook) => (
                <AudiobookTile key={audiobook.id} audiobook={audiobook} />
            ))}
        </div>
    );
}

interface AudiobookTileProps {
    audiobook: Audiobook;
}

function AudiobookTile({ audiobook }: AudiobookTileProps) {
    const { play } = useStore();
    const { durations } = useStore();
    const [showMenu, setShowMenu] = useState(false);
    const deletePosition = useDeletePosition();

    const handleResetProgress = (e: React.MouseEvent) => {
        e.stopPropagation();
        setShowMenu(false);
        deletePosition.mutate({ id: audiobook.id });
    };

    const toggleMenu = (e: React.MouseEvent) => {
        e.stopPropagation();
        setShowMenu(!showMenu);
    };

    return (
        <div
            role="button"
            className={styles.tile}
            onClick={() => play(audiobook.id)}
        >
            <div className={styles.coverContainer}>
                <AudiobookCover
                    coverUrl={audiobook.coverUrl}
                    title={audiobook.title}
                    width={200}
                />
                <ProgressBar
                    position={durations[audiobook.id] || 0}
                    duration={audiobook.runtimeSeconds}
                />
                <button
                    className={styles.menuButton}
                    onClick={toggleMenu}
                    aria-label="More options"
                >
                    ⋯
                </button>
                {showMenu && (
                    <div className={styles.dropdown}>
                        <div className={styles.dropdownHeader}>
                            More options
                        </div>
                        <button
                            className={styles.dropdownItem}
                            onClick={handleResetProgress}
                        >
                            Reset progress
                        </button>
                    </div>
                )}
            </div>
            <div className={styles.text}>
                <div className={styles.title}>{audiobook.title}</div>
                <div className={styles.subtitle}>
                    <>
                        {audiobook.authors.join(", ")} - {audiobook.year}
                    </>
                </div>
            </div>
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
