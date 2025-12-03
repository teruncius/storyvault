import { useQuery } from "@tanstack/react-query";
import type { Audiobook } from "@sv/fe/types/audiobook";
import { ENDPOINTS, getApiUrl } from "@sv/fe/lib/config";
import { useStore } from "@sv/fe/hooks/store";

export function useAudiobooks() {
    const { overrideDuration } = useStore();
    return useQuery({
        queryKey: ["audiobooks", "list"],
        queryFn: async () => {
            const url = getApiUrl(ENDPOINTS.audiobook.list);
            const response = await fetch(url, {
                credentials: "include",
            });
            const audiobooks = (await response.json()) as Audiobook[];
            overrideDuration(audiobooks);
            return audiobooks;
        },
    });
}

interface SearchOptions {
    search?: string;
}

export function useAudiobooksSearch(options: SearchOptions) {
    const { overrideDuration } = useStore();
    return useQuery({
        queryKey: ["audiobooks", "list", options.search],
        queryFn: async () => {
            const url = getApiUrl(ENDPOINTS.audiobook.list);
            if (options.search) {
                url.searchParams.set("search", options.search);
            }
            const response = await fetch(url, {
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
        queryKey: ["audiobooks", "detail", id],
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
