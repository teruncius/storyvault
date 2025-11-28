import { useQuery } from "@tanstack/react-query";
import type { Audiobook } from "@sv/fe/types/audiobook";
import { ENDPOINTS, getApiUrl } from "@sv/fe/lib/config";

export function useAudiobooks() {
    return useQuery({
        queryKey: ["audiobooks"],
        queryFn: async () => {
            const response = await fetch(getApiUrl(ENDPOINTS.audiobook.list), {
                credentials: "include",
            });
            return (await response.json()) as Audiobook[];
        },
    });
}

export function useAudiobook(id: string | null) {
    return useQuery({
        queryKey: ["audiobooks", id],
        queryFn: async () => {
            const response = await fetch(
                getApiUrl(ENDPOINTS.audiobook.detail, id!),
                {
                    credentials: "include",
                },
            );
            return (await response.json()) as Audiobook;
        },
        enabled: !!id,
    });
}
