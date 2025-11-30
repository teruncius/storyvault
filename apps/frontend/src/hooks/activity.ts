import { useQuery } from "@tanstack/react-query";
import type { Activity } from "@sv/fe/types/activity";
import { ENDPOINTS, getApiUrl } from "@sv/fe/lib/config";

export function useActivity() {
    return useQuery({
        queryKey: ["activity"],
        queryFn: async () => {
            const response = await fetch(getApiUrl(ENDPOINTS.activity.list), {
                credentials: "include",
            });
            return (await response.json()) as Activity[];
        },
    });
}
