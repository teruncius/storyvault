import { vars } from "@sv/fe/theme/vars.css";
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
            gridTemplateColumns: "repeat(5, 1fr)",
        },
        "screen and (min-width: 1200px)": {
            gridTemplateColumns: "repeat(6, 1fr)",
        },
    },
});

export const tile = style({
    display: "flex",
    flexDirection: "column",
    gap: "0.25rem",
    overflow: "hidden",
    padding: "0.5rem",
    background: vars.color.background,
    border: 0,
    borderRadius: "0.5rem",
    cursor: "pointer",
    ":hover": {
        textDecoration: "underline",
    },
});

export const title = style({
    fontWeight: "bold",
    textTransform: "uppercase",
    textOverflow: "ellipsis",
    textDecoration: "inherit",
    overflow: "hidden",
    whiteSpace: "nowrap",
    color: vars.color.text,
    fontSize: "0.75rem",
});

export const subtitle = style({
    fontWeight: "normal",
    textTransform: "capitalize",
    textOverflow: "ellipsis",
    textDecoration: "inherit",
    overflow: "hidden",
    whiteSpace: "nowrap",
    color: vars.color.text,
    fontSize: "0.75rem",
});

export const progress = style({
    width: "100%",
    height: "0.5rem",
    overflow: "hidden",
    backgroundColor: vars.color.background,
    borderRadius: "0.25rem",
});

export const progressFill = style({
    height: "100%",
    backgroundColor: vars.color.primary,
});
