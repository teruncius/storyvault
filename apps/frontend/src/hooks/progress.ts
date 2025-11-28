import { HttpError } from "@sv/fe/lib/query-client";
import { useMutation } from "@tanstack/react-query";
import { convertSecondsToISO8601 } from "../lib/iso8601";
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
    event_type: EventType;
    position: string;
}

export function useUpdatePosition() {
    const { setDuration } = useStore();
    return useMutation({
        mutationFn: async ({
            id,
            event_type,
            position,
        }: UpdatePositionRequest) => {
            const response = await fetch(
                getApiUrl(ENDPOINTS.audiobook.position, id),
                {
                    method: "PUT",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify({
                        event_type,
                        position_iso: position,
                    }),
                    credentials: "include",
                },
            );
            if (!response.ok) {
                throw HttpError.fromResponse(response);
            }
            setDuration(id, position);
        },
    });
}
