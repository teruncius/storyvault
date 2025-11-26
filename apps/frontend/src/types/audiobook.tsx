export interface Audiobook {
    id: string;
    title: string;
    author: string;
    year: string;
    cover_url: string;
    stream_url: string;
    position_iso: string | null;
    duration_iso: string;
}
