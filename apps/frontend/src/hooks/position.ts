import { HttpError } from "@sv/fe/lib/query-client";
import { useMutation } from "@tanstack/react-query";
import { ENDPOINTS, getApiUrl } from "@sv/fe/lib/config";
import { useStore } from "@sv/fe/hooks/store";

export const EventType = {
    PLAY: "PLAY",
    PAUSE: "PAUSE",
    STOP: "STOP",
    SEEK: "SEEK",
} as const;

export type EventType = (typeof EventType)[keyof typeof EventType];

interface UpdatePositionRequest {
    id: string;
    eventType: EventType;
    positionSeconds: number;
}

export function useUpdatePosition() {
    const { setDuration } = useStore();
    return useMutation({
        mutationFn: async ({
            id,
            eventType,
            positionSeconds,
        }: UpdatePositionRequest) => {
            const response = await fetch(
                getApiUrl(ENDPOINTS.audiobook.position, id),
                {
                    method: "PUT",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify({
                        eventType,
                        positionSeconds,
                    }),
                    credentials: "include",
                    keepalive: true,
                },
            );
            if (!response.ok) {
                throw HttpError.fromResponse(response);
            }
            setDuration(id, positionSeconds);
        },
    });
}
