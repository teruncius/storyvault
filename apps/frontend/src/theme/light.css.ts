import { createTheme } from "@vanilla-extract/css";
import { vars } from "@sv/fe/theme/vars.css";

export const lightTheme = createTheme(vars, {
    color: {
        text: "#09090b",
        backdrop: "#f4f4f5ff",
        background: "#ffffff",
        border: "#e4e4e7",
        primary: "#16a34a",
    },
});
