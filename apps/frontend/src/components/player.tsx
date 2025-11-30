import { useAudiobook } from "@sv/fe/hooks/audiobooks";
import { useStore } from "@sv/fe/hooks/store";
import * as styles from "@sv/fe/components/player.css";
import { useCallback, useEffect, useRef } from "react";
import { EventType, useUpdatePosition } from "@sv/fe/hooks/position";
import { AudiobookCover } from "@sv/fe/components/audiobook-cover";

export function Player() {
    const { currentAudiobook } = useStore();
    const { data: audiobook } = useAudiobook(currentAudiobook);
    const audioRef = useRef<HTMLAudioElement>(null);
    const mutation = useUpdatePosition();

    const seekToPosition = useCallback(() => {
        if (!audioRef.current) {
            console.error("No audio ref");
            return;
        }
        if (!audiobook?.positionSeconds) {
            console.error("No position");
            return;
        }
        console.debug("Setting initial position to", audiobook.positionSeconds);
        audioRef.current.currentTime = audiobook.positionSeconds;
    }, [audiobook]);

    const sendPosition = useCallback(
        (eventType: EventType) => {
            if (!audiobook) {
                console.error("No audiobook");
                return;
            }
            const positionSeconds = Math.floor(
                audioRef.current?.currentTime || 0,
            );
            console.log(
                "Sending position",
                audiobook.id,
                eventType,
                positionSeconds,
            );
            mutation.mutate({ id: audiobook.id, eventType, positionSeconds });
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
                <AudiobookCover
                    coverUrl={audiobook.coverUrl}
                    title={audiobook.title}
                    width={40}
                    style={{ gridArea: "logo" }}
                />
                <div className={styles.title}>{audiobook.title}</div>
                <div className={styles.subtitle}>
                    {audiobook.authors.join(", ")} - {audiobook.year}
                </div>
            </figcaption>
            <audio
                ref={audioRef}
                className={styles.player}
                src={audiobook.streamUrl}
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
