import {
    Card,
    CardContent,
    CardHeader,
    CardTitle,
} from "@storyvault/frontend/components/ui/card";
import { AspectRatio } from "@storyvault/frontend/components/ui/aspect-ratio";
import type { Audiobook } from "@storyvault/frontend/types/audiobook";

interface Props {
    audiobooks: Audiobook[];
}

export function Audiobooks(props: Props) {
    return (
        <div className="grid grid-cols-8 gap-4">
            {props.audiobooks.map((audiobook) => (
                <Card key={audiobook.id}>
                    <CardContent className="p-0">
                        <AspectRatio ratio={1 / 1}>
                            <img
                                src={audiobook.cover_url}
                                alt={audiobook.title}
                                className="h-full w-full object-cover"
                            />
                        </AspectRatio>
                    </CardContent>
                    <CardHeader>
                        <CardTitle className="text-base truncate">
                            {audiobook.title}
                        </CardTitle>
                    </CardHeader>
                </Card>
            ))}
        </div>
    );
}
