import { createThemeContract } from "@vanilla-extract/css";

export const vars = createThemeContract({
    color: {
        text: null,
        background: null,
        backdrop: null,
    },
});
