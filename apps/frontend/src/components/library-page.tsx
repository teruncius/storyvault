import { Audiobooks } from "@sv/fe/components/audiobooks";
import { useAudiobooksSearch } from "@sv/fe/hooks/audiobooks";
import { useSearchParams } from "react-router-dom";

export function LibraryPage() {
    const [searchParams] = useSearchParams();
    const search = searchParams.get("query") || "";

    const { data: audiobooks } = useAudiobooksSearch({ search });

    return <Audiobooks audiobooks={audiobooks || []} />;
}
