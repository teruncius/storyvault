import { vars } from "@storyvault/frontend/theme/vars.css";
import { style } from "@vanilla-extract/css";

export const container = style({
    display: "grid",
    gridTemplateColumns: "repeat(1, 1fr)",
    gap: "1rem",
    "@media": {
        "screen and (min-width: 384px)": {
            gridTemplateColumns: "repeat(2, 1fr)",
        },
        "screen and (min-width: 512px)": {
            gridTemplateColumns: "repeat(3, 1fr)",
        },
        "screen and (min-width: 768px)": {
            gridTemplateColumns: "repeat(4, 1fr)",
        },
        "screen and (min-width: 1024px)": {
            gridTemplateColumns: "repeat(6, 1fr)",
        },
        "screen and (min-width: 1200px)": {
            gridTemplateColumns: "repeat(8, 1fr)",
        },
    },
});

export const tile = style({
    display: "flex",
    flexDirection: "column",
    gap: "0.25rem",
    overflow: "hidden",
});

export const title = style({
    fontWeight: "bold",
    textTransform: "uppercase",
    textOverflow: "ellipsis",
    overflow: "hidden",
    whiteSpace: "nowrap",
    color: vars.color.text,
});

export const image = style({
    width: "100%",
    objectFit: "cover",
    aspectRatio: "1 / 1",
});
