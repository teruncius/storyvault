import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "@storyvault/frontend/index.css.ts";
import { App } from "@storyvault/frontend/components/app";

createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <App />
    </StrictMode>,
);
