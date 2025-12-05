import { style } from "@vanilla-extract/css";

export const image = style({
    display: "block",
    width: "100%",
    objectFit: "cover",
    aspectRatio: "1 / 1",
    borderRadius: "0.25rem",
});
