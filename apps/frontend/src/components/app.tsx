import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "@storyvault/frontend/lib/query-client";
import { Audiobooks } from "@storyvault/frontend/components/audiobooks";
import { useAudiobooks } from "@storyvault/frontend/hooks/audiobooks";

export function App() {
    return (
        <QueryClientProvider client={queryClient}>
            <Content />
        </QueryClientProvider>
    );
}

function Content() {
    const { data: audiobooks } = useAudiobooks();
    return (
        <>
            <header className="fixed top-0 left-0 right-0 z-10 p-4 bg-gray-600 shadow-lg">
                <div className="container mx-auto px-4">
                    <h1 className="text-xl font-bold text-white">StoryVault</h1>
                </div>
            </header>

            <main className="pt-20 pb-16 min-h-screen">
                <div className="container mx-auto px-4">
                    {audiobooks && <Audiobooks audiobooks={audiobooks} />}
                </div>
            </main>

            <footer className="fixed bottom-0 left-0 right-0 z-10 p-4 bg-gray-600 shadow-lg">
                <div className="container mx-auto px-4">
                    <p className="text-white">StoryVault</p>
                </div>
            </footer>
        </>
    );
}
