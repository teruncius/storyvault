import { useQuery } from "@tanstack/react-query";
import type { Audiobook } from "@storyvault/frontend/types/audiobook";

export function useAudiobooks() {
    return useQuery({
        queryKey: ["audiobooks"],
        queryFn: async () => {
            const response = await fetch("http://localhost:3000/audiobook", {
                credentials: "include",
            });
            return (await response.json()) as Audiobook[];
        },
    });
}
