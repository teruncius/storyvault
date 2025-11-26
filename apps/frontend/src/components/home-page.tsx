import { Audiobooks } from "@sv/fe/components/audiobooks";
import { useAudiobooks } from "@sv/fe/hooks/audiobooks";

export function HomePage() {
    const { data: audiobooks } = useAudiobooks();
    return <Audiobooks audiobooks={audiobooks || []} />;
}
