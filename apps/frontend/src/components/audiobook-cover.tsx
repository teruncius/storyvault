import * as styles from "@sv/fe/components/audiobook-cover.css";

interface Props {
    coverUrl: string;
    title: string;
    width: number;
    style?: React.CSSProperties;
}

export function AudiobookCover({ coverUrl, title, width, style }: Props) {
    const url = new URL(coverUrl);
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
