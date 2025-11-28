export interface Audiobook {
    id: string;
    title: string;
    authors: string[];
    year: string;
    coverUrl: string;
    streamUrl: string;
    positionIso: string | null;
    durationIso: string;
}
