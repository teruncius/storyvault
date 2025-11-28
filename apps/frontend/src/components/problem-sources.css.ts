import { vars } from "@sv/fe/theme/vars.css";
import { style } from "@vanilla-extract/css";

export const container = style({
    background: vars.color.backdrop,
    borderRadius: "1rem",
});

export const title = style({
    fontSize: "1.25rem",
    fontWeight: "700",
    color: vars.color.text,
    margin: "0 0 1rem 0",
});

export const list = style({
    listStyle: "none",
    padding: 0,
    margin: 0,
    display: "flex",
    flexDirection: "column",
    gap: "1rem",
});

export const item = style({
    background: vars.color.background,
    borderRadius: "0.5rem",
    padding: "0.75rem 1rem",
    color: vars.color.text,
    display: "flex",
    flexDirection: "row",
    gap: "0.5rem",
    justifyContent: "space-between",
    alignItems: "center",
    cursor: "pointer",
    border: `1px solid transparent`,
    transition: "border-color 0.2s ease",
    ":hover": {
        borderColor: vars.color.primary,
    },
});

export const selected = style({
    borderColor: vars.color.primary,
});

export const sourceName = style({
    fontWeight: "500",
    fontSize: "1rem",
    overflow: "hidden",
    textOverflow: "ellipsis",
    whiteSpace: "nowrap",
});

export const count = style({
    background: vars.color.backdrop,
    borderRadius: "12px",
    padding: "0.25rem",
    fontSize: "1rem",
    fontWeight: "700",
    minWidth: "2rem",
    textAlign: "center",
});
