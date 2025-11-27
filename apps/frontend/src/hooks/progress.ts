import { HttpError } from "@sv/fe/lib/query-client";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { convertSecondsToISO8601 } from "../lib/iso8601";

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
    const queryClient = useQueryClient();
    return useMutation({
        mutationFn: async ({
            id,
            event_type,
            position,
        }: UpdatePositionRequest) => {
            console.log(id, event_type, position);
            const response = await fetch(
                `http://localhost:3000/api/audiobook/${id}/position`,
                {
                    method: "PUT",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify({
                        event_type,
                        position_iso: convertSecondsToISO8601(position),
                    }),
                    credentials: "include",
                },
            );
            if (!response.ok) {
                throw HttpError.fromResponse(response);
            }
        },
        onSuccess: (_data, variables) => {
            queryClient.invalidateQueries({
                queryKey: ["audiobooks", variables.id],
            });
            queryClient.invalidateQueries({ queryKey: ["audiobooks"] });
        },
    });
}
