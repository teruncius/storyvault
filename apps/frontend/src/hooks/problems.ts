import { useQuery } from "@tanstack/react-query";
import type { Problem } from "@sv/fe/types/problem";
import { ENDPOINTS, getApiUrl } from "@sv/fe/lib/config";
import { HttpError } from "@sv/fe/lib/query-client";

export function useProblems() {
    return useQuery({
        queryKey: ["problems"],
        queryFn: async () => {
            const response = await fetch(getApiUrl(ENDPOINTS.problem.list), {
                credentials: "include",
            });
            if (!response.ok) {
                throw HttpError.fromResponse(response);
            }
            return (await response.json()) as Problem[];
        },
    });
}
