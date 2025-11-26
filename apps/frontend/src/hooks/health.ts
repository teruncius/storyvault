import { HttpError } from "@sv/fe/lib/query-client";
import { useQuery } from "@tanstack/react-query";

interface HealthResponse {
    status: string;
}

export function useHealth() {
    return useQuery({
        queryKey: ["health"],
        queryFn: async () => {
            const response = await fetch("http://localhost:3000/health", {
                credentials: "include",
            });
            if (!response.ok) {
                throw HttpError.fromResponse(response);
            }
            return (await response.json()) as HealthResponse;
        },
    });
}
