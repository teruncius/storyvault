import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import type { User } from "@sv/fe/types/user";
import { HttpError } from "@sv/fe/lib/query-client";

const AUTH_QUERY_KEY = "auth/me";

export function useAuth() {
    return useQuery({
        queryKey: [AUTH_QUERY_KEY],
        queryFn: async () => {
            const response = await fetch("http://localhost:3000/auth/me", {
                credentials: "include",
            });
            if (response.status == 401) {
                return null;
            }
            if (!response.ok) {
                throw HttpError.fromResponse(response);
            }
            return (await response.json()) as User;
        },
    });
}

interface LoginInput {
    email: string;
    password: string;
}

export function useLogin() {
    const queryClient = useQueryClient();
    return useMutation({
        mutationFn: async ({ email, password }: LoginInput) => {
            const response = await fetch("http://localhost:3000/auth/login", {
                method: "POST",
                body: JSON.stringify({ email, password }),
                headers: {
                    "Content-Type": "application/json",
                },
                credentials: "include",
            });
            if (!response.ok) {
                throw HttpError.fromResponse(response);
            }
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: [AUTH_QUERY_KEY] });
        },
    });
}

export function useLogout() {
    const queryClient = useQueryClient();
    return useMutation({
        mutationFn: async () => {
            const response = await fetch("http://localhost:3000/auth/logout", {
                method: "POST",
                credentials: "include",
            });
            if (!response.ok) {
                throw HttpError.fromResponse(response);
            }
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: [AUTH_QUERY_KEY] });
        },
    });
}
