import { style } from "@vanilla-extract/css";
import { vars } from "@sv/fe/theme/vars.css";

export const container = style({
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    minHeight: "100vh",
    backgroundColor: vars.color.background,
    color: vars.color.text,
    textAlign: "center",
    padding: "2rem",
});

export const title = style({
    fontSize: "6rem",
    fontWeight: "bold",
    color: vars.color.text,
    margin: 0,
    lineHeight: 1,
});

export const subtitle = style({
    fontSize: "1.5rem",
    marginTop: "1rem",
    marginBottom: "2rem",
    opacity: 0.8,
});

export const link = style({
    display: "inline-block",
    padding: "1rem 2rem",
    backgroundColor: vars.color.backdrop,
    color: vars.color.text,
    textDecoration: "none",
    borderRadius: "8px",
    fontWeight: "bold",
    transition: "opacity 0.2s",
    ":hover": {
        opacity: 0.9,
    },
});
