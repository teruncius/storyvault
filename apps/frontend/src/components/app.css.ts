import { vars } from "@storyvault/frontend/theme/vars.css";
import { style } from "@vanilla-extract/css";

export const container = style({
    display: "flex",
    flexDirection: "column",
    minHeight: "100vh",
    background: vars.color.backdrop,
});

export const header = style({
    display: "flex",
    justifyContent: "center",
    alignItems: "center",
    height: 64,
    backgroundColor: vars.color.background,
    color: vars.color.text,
    fontSize: "1.5rem",
    fontWeight: "bold",
    padding: "0 1rem 0 1rem",
    position: "sticky",
    zIndex: 1,
    top: 0,
});

export const main = style({
    display: "flex",
    justifyContent: "center",
    flexGrow: 1,
    minHeight: 0,
    padding: "1rem",
});

export const footer = style({
    display: "flex",
    justifyContent: "center",
    alignItems: "center",
    height: 64,
    backgroundColor: vars.color.background,
    padding: "0 1rem 0 1rem",
    position: "sticky",
    zIndex: 1,
    bottom: 0,
});

export const center = style({
    display: "flex",
    flexDirection: "column",
    width: 1200,
});
