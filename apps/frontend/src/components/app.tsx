import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "@storyvault/frontend/lib/query-client";
import { Audiobooks } from "@storyvault/frontend/components/audiobooks";
import { useAudiobooks } from "@storyvault/frontend/hooks/audiobooks";
import {
    container,
    header,
    main,
    footer,
    center,
} from "@storyvault/frontend/components/app.css";
import type { PropsWithChildren } from "react";
import { darkTheme } from "@storyvault/frontend/theme/dark.css";

export function App() {
    return (
        <QueryClientProvider client={queryClient}>
            <ThemeProvider>
                <Content />
            </ThemeProvider>
        </QueryClientProvider>
    );
}

function ThemeProvider({ children }: PropsWithChildren) {
    return <div className={darkTheme}>{children}</div>;
}

function Content() {
    const { data: audiobooks } = useAudiobooks();
    return (
        <div className={container}>
            <header className={header}>
                <div className={center}>StoryVault</div>
            </header>
            <main className={main}>
                <div className={center}>
                    {audiobooks && <Audiobooks audiobooks={audiobooks} />}
                </div>
            </main>
            <footer className={footer}>
                <div className={center}> </div>
            </footer>
        </div>
    );
}
