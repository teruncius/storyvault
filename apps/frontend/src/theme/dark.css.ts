import { createTheme } from "@vanilla-extract/css";
import { vars } from "@storyvault/frontend/theme/vars.css";

export const darkTheme = createTheme(vars, {
    color: {
        text: "#fff",
        backdrop: "#09090b",
        background: "#18181b",
        border: "#fff",
    },
});
