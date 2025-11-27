import { useQuery } from "@tanstack/react-query";
import type { Problem } from "@sv/fe/types/problem";

export function useProblems() {
    return useQuery({
        queryKey: ["problems"],
        queryFn: async () => {
            const response = await fetch("http://localhost:3000/api/problem", {
                credentials: "include",
            });
            if (!response.ok) {
                throw new Error("Failed to fetch problems");
            }
            return (await response.json()) as Problem[];
        },
    });
}
