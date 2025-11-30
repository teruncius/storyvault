import { create } from "zustand";
import type { Audiobook } from "@sv/fe/types/audiobook";

interface PlayerState {
    currentAudiobook: string | null;
    play: (id: string) => void;
    durations: Record<string, number>;
    overrideDuration: (audiobooks: Audiobook[]) => void;
    setDuration: (id: string, duration: number) => void;
}

export const useStore = create<PlayerState>((set, get) => ({
    currentAudiobook: null,
    play: (id: string) => {
        set({ currentAudiobook: id });
    },
    durations: {},
    overrideDuration: (audiobooks: Audiobook[]) => {
        const durations: Record<string, number> = {};
        for (const audiobook of audiobooks) {
            if (audiobook.positionSeconds) {
                durations[audiobook.id] = audiobook.positionSeconds;
            }
        }
        console.debug("overrideDuration", durations);
        set({ durations });
    },
    setDuration: (id: string, duration: number) => {
        console.debug("setDuration", id, duration);
        const durations = {
            ...get().durations,
            [id]: duration,
        };
        set({ durations });
    },
}));
