export const API_URL = import.meta.env.VITE_API_URL;

export const ENDPOINTS = {
    health: "/api/health",
    auth: {
        login: "/api/auth/login",
        logout: "/api/auth/logout",
        me: "/api/auth/me",
        register: "/api/auth/register",
    },
    audiobook: {
        list: "/api/audiobook",
        detail: "/api/audiobook/{id}",
        position: "/api/audiobook/{id}/position",
    },
    activity: {
        list: "/api/activity",
    },
    problem: {
        list: "/api/problem",
    },
};

export function getApiUrl(endpoint: string, ...params: string[]) {
    return (
        API_URL +
        endpoint.replace(/\{\w+\}/g, (param) => params.shift() || param)
    );
}
