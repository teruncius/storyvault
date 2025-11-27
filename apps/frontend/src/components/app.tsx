import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "@sv/fe/lib/query-client";
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
import { ThemeProvider } from "@sv/fe/components/theme";
import { NotFoundPage } from "@sv/fe/components/not-found-page";

export function App() {
    return (
        <QueryClientProvider client={queryClient}>
            <ThemeProvider>
                <Content />
            </ThemeProvider>
        </QueryClientProvider>
    );
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
                <Route path="*" element={<NotFoundPage />} />
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
