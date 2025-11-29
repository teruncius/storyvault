import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import type { User } from "@sv/fe/types/user";
import { HttpError } from "@sv/fe/lib/query-client";
import { ENDPOINTS, getApiUrl } from "@sv/fe/lib/config";

const AUTH_QUERY_KEY = "auth/me";

export function useAuth() {
    return useQuery({
        queryKey: [AUTH_QUERY_KEY],
        queryFn: async () => {
            const response = await fetch(getApiUrl(ENDPOINTS.auth.me), {
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
            const response = await fetch(getApiUrl(ENDPOINTS.auth.login), {
                method: "POST",
                body: JSON.stringify({ email, password }),
                headers: {
                    "Content-Type": "application/json",
                },
                credentials: "include",
            });
            if (!response.ok) {
                throw HttpError.fromResponse(response, true);
            }
            queryClient.invalidateQueries({ queryKey: [AUTH_QUERY_KEY] });
        },
    });
}

export function useLogout() {
    const queryClient = useQueryClient();
    return useMutation({
        mutationFn: async () => {
            const response = await fetch(getApiUrl(ENDPOINTS.auth.logout), {
                method: "POST",
                credentials: "include",
            });
            if (!response.ok) {
                throw HttpError.fromResponse(response);
            }
            queryClient.invalidateQueries({ queryKey: [AUTH_QUERY_KEY] });
        },
    });
}

export interface RegisterInput {
    email: string;
    password: string;
    firstName: string;
    lastName: string;
}

export function useRegister() {
    return useMutation({
        mutationFn: async (input: RegisterInput) => {
            const response = await fetch(getApiUrl(ENDPOINTS.auth.register), {
                method: "POST",
                body: JSON.stringify(input),
                headers: {
                    "Content-Type": "application/json",
                },
                credentials: "include",
            });
            if (!response.ok) {
                throw HttpError.fromResponse(response, true);
            }
        },
    });
}
