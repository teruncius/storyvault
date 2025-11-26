import { create } from "zustand";

interface PlayerState {
    currentAudiobook: string | null;
    play: (id: string) => void;
}

export const useStore = create<PlayerState>((set) => ({
    currentAudiobook: null,
    play: (id: string) => {
        set({ currentAudiobook: id });
    },
}));
