import * as styles from "@sv/fe/components/audiobook-cover.css";

interface Props {
    cover_url: string;
    title: string;
    width: number;
    style?: React.CSSProperties;
}

export function AudiobookCover({ cover_url, title, width, style }: Props) {
    const url = new URL(cover_url);
    url.searchParams.set("width", width.toString());

    return (
        <img
            className={styles.image}
            src={url.toString()}
            style={style}
            alt={title}
        />
    );
}
