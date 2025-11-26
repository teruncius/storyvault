import { MutationCache, QueryCache, QueryClient } from "@tanstack/react-query";

export const queryClient = new QueryClient({
    defaultOptions: {
        queries: {
            retry: shouldRetry,
            staleTime: 5 * 60 * 1000,
            refetchOnWindowFocus: false,
        },
        mutations: {
            retry: false,
        },
    },
    queryCache: new QueryCache({
        onError: handleUnauthorizedError,
    }),
    mutationCache: new MutationCache({
        onError: handleUnauthorizedError,
    }),
});

function shouldRetry(failureCount: number, error: unknown): boolean {
    // do not retry on client errors
    if (
        error instanceof HttpError &&
        error.status >= 400 &&
        error.status < 500
    ) {
        return false;
    }

    // retry on server errors
    return failureCount < 1;
}

function handleUnauthorizedError(error: unknown): void {
    if (
        error instanceof HttpError &&
        error.status === 401 &&
        !error.skipRedirect
    ) {
        queryClient.clear();
        window.location.replace("/login");
    }
}

export class HttpError extends Error {
    readonly status: number;
    readonly skipRedirect?: boolean;

    constructor(message: string, status: number, skipRedirect?: boolean) {
        super(message);
        this.status = status;
        this.skipRedirect = skipRedirect;
    }

    static fromResponse(response: Response, skipRedirect?: boolean): HttpError {
        return new HttpError(
            response.statusText,
            response.status,
            skipRedirect,
        );
    }
}
