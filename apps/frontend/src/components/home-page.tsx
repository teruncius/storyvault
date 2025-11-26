import { useAudiobooks } from "@storyvault/frontend/hooks/audiobooks";
import { Audiobooks } from "./audiobooks";

export function HomePage() {
    const { data: audiobooks } = useAudiobooks();
    return <Audiobooks audiobooks={audiobooks || []} />;
}
