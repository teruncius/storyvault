import { useQuery } from "@tanstack/react-query";
import type { Audiobook } from "@sv/fe/types/audiobook";
import { ENDPOINTS, getApiUrl } from "@sv/fe/lib/config";
import { useStore } from "@sv/fe/hooks/store";

export function useAudiobooks() {
    const { overrideDuration } = useStore();
    return useQuery({
        queryKey: ["audiobooks"],
        queryFn: async () => {
            const response = await fetch(getApiUrl(ENDPOINTS.audiobook.list), {
                credentials: "include",
            });
            const audiobooks = (await response.json()) as Audiobook[];
            overrideDuration(audiobooks);
            return audiobooks;
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
