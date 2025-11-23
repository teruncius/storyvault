import { useQuery } from "@tanstack/react-query";

export function useHealth() {
    return useQuery({
        queryKey: ['health'],
        queryFn: async () => {
            const response = await fetch('http://localhost:3000/health');
            return await response.json();
        },
    });
}
