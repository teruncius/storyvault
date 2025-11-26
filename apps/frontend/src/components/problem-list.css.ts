import { vars } from "@sv/fe/theme/vars.css";
import { style } from "@vanilla-extract/css";

export const container = style({
    marginBottom: "2rem",
});

export const title = style({
    fontSize: "1.25rem",
    fontWeight: "bold",
    marginBottom: "1rem",
    color: vars.color.text,
});

export const list = style({
    display: "flex",
    flexDirection: "column",
    gap: "0.75rem",
});

export const pathGroup = style({
    display: "flex",
    flexDirection: "column",
    gap: "0.5rem",
    marginBottom: "1rem",
});

export const item = style({
    padding: "1rem",
    border: `0`,
    borderRadius: "0.5rem",
    backgroundColor: vars.color.background,
});

export const problemType = style({
    fontSize: "0.875rem",
    fontWeight: "bold",
    color: vars.color.text,
    marginBottom: "0.5rem",
});

export const path = style({
    fontSize: "0.75rem",
    color: vars.color.text,
    marginBottom: "0.25rem",
    fontFamily: "monospace",
    opacity: 0.8,
});

export const message = style({
    fontSize: "0.875rem",
    color: vars.color.text,
    opacity: 0.9,
});
