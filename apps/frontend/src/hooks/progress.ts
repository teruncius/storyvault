import { HttpError } from "@sv/fe/lib/query-client";
import { useMutation, useQuery } from "@tanstack/react-query";

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
    position: number;
}

export function useUpdatePosition() {
    return useMutation({
        mutationFn: async ({
            id,
            event_type,
            position,
        }: UpdatePositionRequest) => {
            console.log(id, event_type, position);
            const response = await fetch(
                `http://localhost:3000/audiobook/${id}/position`,
                {
                    method: "PUT",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify({
                        event_type,
                        position_iso: secondsToISOString(position),
                    }),
                    credentials: "include",
                },
            );
            if (!response.ok) {
                throw HttpError.fromResponse(response);
            }
        },
    });
}

interface PositionResponse {
    position_iso: string;
}

export function useAudiobookPosition(audiobookId: string | null) {
    return useQuery({
        queryKey: ["audiobook-position", audiobookId],
        queryFn: async () => {
            const response = await fetch(
                `http://localhost:3000/audiobook/${audiobookId}/position`,
                {
                    credentials: "include",
                },
            );
            if (!response.ok) {
                throw HttpError.fromResponse(response);
            }
            const data = (await response.json()) as PositionResponse;
            console.log(audiobookId, data);
            return isoStringToSeconds(data.position_iso);
        },
        enabled: !!audiobookId,
    });
}

function secondsToISOString(seconds: number) {
    return `PT${Math.abs(Math.floor(seconds))}S`;
}

function isoStringToSeconds(isoString: string) {
    const match = isoString.match(/PT(\d+)S/);
    if (!match) {
        return 0;
    }
    return parseInt(match[1], 10);
}
