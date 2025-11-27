import { vars } from "@sv/fe/theme/vars.css";
import { style } from "@vanilla-extract/css";

export const container = style({
    display: "flex",
    flexDirection: "row",
    justifyContent: "space-between",
    gap: "1rem",
    padding: 0,
    margin: 0,
});

export const caption = style({
    display: "grid",
    gridTemplateAreas: `
        "logo title"
        "logo subtitle"
    `,
    gap: "0.5rem",
});

export const image = style({
    gridArea: "logo",
});

export const title = style({
    color: vars.color.text,
    display: "flex",
    alignItems: "center",
    fontWeight: "bold",
    textTransform: "uppercase",
    fontSize: "0.75rem",
    gridArea: "title",
});

export const subtitle = style({
    color: vars.color.text,
    display: "flex",
    alignItems: "center",
    fontWeight: "normal",
    textTransform: "capitalize",
    fontSize: "0.75rem",
    gridArea: "subtitle",
});

export const player = style({});
