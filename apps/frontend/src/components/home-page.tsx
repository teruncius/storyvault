import { Audiobooks } from "@sv/fe/components/audiobooks";
import { useActivity } from "@sv/fe/hooks/activity";
import { useAudiobooks } from "@sv/fe/hooks/audiobooks";
import type { Audiobook } from "@sv/fe/types/audiobook";

export function HomePage() {
    const { data: audiobooks } = useAudiobooks();
    const { data: activities } = useActivity();

    const map = audiobooks?.reduce(
        (acc, audiobook) => {
            acc[audiobook.id] = audiobook;
            return acc;
        },
        {} as Record<string, Audiobook>,
    );

    const recent = activities
        ?.map((activity) => activity.audiobook.id)
        .map((id) => map?.[id])
        .filter((audiobook) => audiobook !== undefined) as Audiobook[];

    return <Audiobooks audiobooks={recent || []} />;
}
