import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "@sv/fe/lib/query-client";
import type { PropsWithChildren } from "react";
import { darkTheme } from "@sv/fe/theme/dark.css";
import { useAuth } from "@sv/fe/hooks/user";
import {
    BrowserRouter,
    Navigate,
    Outlet,
    Route,
    Routes,
} from "react-router-dom";
import { LoginPage } from "@sv/fe/components/login-page";
import { HomePage } from "@sv/fe/components/home-page";
import { ProblemPage } from "@sv/fe/components/problem-page";
import { Layout } from "@sv/fe/components/layout";
import { LoadingPage } from "@sv/fe/components/loading-page";

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
    const { data: user, isLoading } = useAuth();
    const login = user ? <Navigate to="/" /> : <LoginPage />;

    if (isLoading) {
        return <LoadingPage />;
    }

    return (
        <BrowserRouter>
            <Routes>
                <Route path="/login" element={login} />
                <Route path="/" element={<ProtectedRoute />}>
                    <Route index element={<HomePage />} />
                    <Route path="problems" element={<ProblemPage />} />
                </Route>
            </Routes>
        </BrowserRouter>
    );
}

function ProtectedRoute() {
    const { data: user, isLoading } = useAuth();

    if (isLoading) {
        return <LoadingPage />;
    }

    if (!user) {
        return <Navigate to="/login" />;
    }

    return (
        <Layout user={user}>
            <Outlet />
        </Layout>
    );
}
