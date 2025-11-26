import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "@storyvault/frontend/lib/query-client";
import type { PropsWithChildren } from "react";
import { darkTheme } from "@storyvault/frontend/theme/dark.css";
import { useAuth } from "@storyvault/frontend/hooks/user";
import { BrowserRouter, Navigate, Outlet, Route } from "react-router";
import { Routes } from "react-router";
import { LoginPage } from "@storyvault/frontend/components/login-page";
import { HomePage } from "@storyvault/frontend/components/home-page";
import { Layout } from "@storyvault/frontend/components/layout";

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
    const { data: user } = useAuth();
    const login = user ? <Navigate to="/" /> : <LoginPage />;

    return (
        <BrowserRouter>
            <Routes>
                <Route path="/login" element={login} />
                <Route path="/" element={<ProtectedRoute />}>
                    <Route index element={<HomePage />} />
                </Route>
            </Routes>
        </BrowserRouter>
    );
}

function ProtectedRoute() {
    const { data: user, isLoading } = useAuth();

    if (isLoading) {
        return <div>Loading...</div>;
    }

    if (!user) {
        return <Navigate to="/login" />;
    }

    return (
        <Layout>
            <Outlet />
        </Layout>
    );
}
