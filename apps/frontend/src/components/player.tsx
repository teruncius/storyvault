import { useAudiobook } from "@sv/fe/hooks/audiobooks";
import { useStore } from "@sv/fe/hooks/store";
import * as styles from "@sv/fe/components/player.css";
import { useCallback, useEffect, useRef } from "react";
import { EventType, useUpdatePosition } from "@sv/fe/hooks/progress";
import { convertISO8601ToSeconds } from "@sv/fe/lib/iso8601";

export function Player() {
    const { currentAudiobook } = useStore();
    const { data: audiobook } = useAudiobook(currentAudiobook);
    const audioRef = useRef<HTMLAudioElement>(null);

    const mutation = useUpdatePosition();

    const seekToPosition = useCallback(() => {
        if (!audioRef.current) {
            console.log("No audio ref");
            return;
        }
        if (!audiobook?.position_iso) {
            console.log("No position");
            return;
        }
        console.log("Setting position to", audiobook.position_iso);
        audioRef.current.currentTime = convertISO8601ToSeconds(
            audiobook.position_iso,
        );
    }, [audiobook]);

    const sendPosition = useCallback(
        (event_type: EventType) => {
            if (!audiobook) {
                return;
            }
            const position = audioRef.current?.currentTime || 0;
            mutation.mutate({ id: audiobook.id, event_type, position });
        },
        [audiobook, mutation],
    );

    useEffect(() => {
        const handleBeforeUnload = () => {
            sendPosition(EventType.STOP);
        };
        window.addEventListener("beforeunload", handleBeforeUnload);
        return () => {
            window.removeEventListener("beforeunload", handleBeforeUnload);
        };
    }, [sendPosition]);

    if (!audiobook) {
        return null;
    }

    return (
        <figure className={styles.container}>
            <figcaption className={styles.caption}>
                <img
                    className={styles.image}
                    src={audiobook.cover_url}
                    alt={audiobook.title}
                />
                <div className={styles.title}>{audiobook.title}</div>
                <div className={styles.subtitle}>
                    {audiobook.authors.join(", ")} - {audiobook.year}
                </div>
            </figcaption>
            <audio
                ref={audioRef}
                className={styles.player}
                src={audiobook.stream_url}
                controls
                onPlay={() => sendPosition(EventType.PLAY)}
                onPause={() => sendPosition(EventType.PAUSE)}
                onSeeked={() => sendPosition(EventType.SEEK)}
                onEnded={() => sendPosition(EventType.STOP)}
                onLoadedMetadata={seekToPosition}
            ></audio>
        </figure>
    );
}
