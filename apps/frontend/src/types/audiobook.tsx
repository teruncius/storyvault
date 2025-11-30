export interface Audiobook {
    id: string;
    title: string;
    authors: string[];
    year: string;
    coverUrl: string;
    streamUrl: string;
    positionSeconds: number | null;
    runtimeSeconds: number;
}
