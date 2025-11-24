import { vars } from "@storyvault/frontend/theme/vars.css";
import { style } from "@vanilla-extract/css";

export const container = style({
    display: "grid",
    gridTemplateColumns: "repeat(8, 1fr)",
    gap: "1rem",
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
