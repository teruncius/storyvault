import {
    container,
    image,
    tile,
    title,
} from "@storyvault/frontend/components/audiobooks.css";
import type { Audiobook } from "@storyvault/frontend/types/audiobook";

interface Props {
    audiobooks: Audiobook[];
}

export function Audiobooks(props: Props) {
    return (
        <div className={container}>
            {props.audiobooks.map((audiobook) => (
                <div className={tile} key={audiobook.id}>
                    <img
                        src={audiobook.cover_url}
                        alt={audiobook.title}
                        className={image}
                    />
                    <div className={title}>{audiobook.title}</div>
                </div>
            ))}
        </div>
    );
}
