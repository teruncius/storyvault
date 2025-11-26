import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "@sv/fe/index.css.ts";
import { App } from "@sv/fe/components/app";

createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <App />
    </StrictMode>,
);
