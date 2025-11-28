import { useQuery } from "@tanstack/react-query";
import type { Audiobook } from "@sv/fe/types/audiobook";

export function useAudiobooks() {
    return useQuery({
        queryKey: ["audiobooks"],
        queryFn: async () => {
            const response = await fetch(
                "http://localhost:3000/api/audiobook",
                {
                    credentials: "include",
                },
            );
            return (await response.json()) as Audiobook[];
        },
    });
}

export function useAudiobook(id: string | null) {
    return useQuery({
        queryKey: ["audiobooks", id],
        queryFn: async () => {
            const response = await fetch(
                `http://localhost:3000/api/audiobook/${id}`,
                {
                    credentials: "include",
                },
            );
            return (await response.json()) as Audiobook;
        },
        enabled: !!id,
    });
}
