import { create } from "zustand";
import type { Audiobook } from "@sv/fe/types/audiobook";

interface PlayerState {
    currentAudiobook: string | null;
    play: (id: string) => void;
    durations: Record<string, string>;
    overrideDuration: (audiobooks: Audiobook[]) => void;
    setDuration: (id: string, duration: string) => void;
}

export const useStore = create<PlayerState>((set, get) => ({
    currentAudiobook: null,
    play: (id: string) => {
        set({ currentAudiobook: id });
    },
    durations: {},
    overrideDuration: (audiobooks: Audiobook[]) => {
        const durations: Record<string, string> = {};
        for (const audiobook of audiobooks) {
            if (audiobook.positionIso) {
                durations[audiobook.id] = audiobook.positionIso;
            }
        }
        console.log("overrideDuration", durations);
        set({ durations });
    },
    setDuration: (id: string, duration: string) => {
        console.log("setDuration", id, duration);
        const durations = {
            ...get().durations,
            [id]: duration,
        };
        set({ durations });
    },
}));
