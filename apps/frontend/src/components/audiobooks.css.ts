import { vars } from "@sv/fe/theme/vars.css";
import { globalStyle, style } from "@vanilla-extract/css";

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
    padding: "0.5rem",
    background: vars.color.background,
    border: 0,
    borderRadius: "0.5rem",
    overflow: "hidden",
    cursor: "pointer",
    ":hover": {
        textDecoration: "underline",
    },
});

export const text = style({});

export const title = style({
    fontWeight: "bold",
    textTransform: "uppercase",
    textOverflow: "ellipsis",
    textDecoration: "inherit",
    overflow: "hidden",
    whiteSpace: "nowrap",
    color: vars.color.text,
    fontSize: "0.75rem",
    textAlign: "center",
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
    opacity: 0.7,
    textAlign: "center",
});

export const coverContainer = style({
    position: "relative",
    width: "100%",
});

export const qualityIndicators = style({
    position: "absolute",
    top: "0.5rem",
    left: "0.5rem",
    display: "flex",
    gap: "0.25rem",
    zIndex: 10,
    opacity: 0,
    transition: "opacity 0.2s",
});

export const qualityCircle = style({
    width: "0.5rem",
    height: "0.5rem",
    borderRadius: "50%",
    border: "1px solid rgba(0, 0, 0, 0.3)",
});

export const progress = style({
    position: "absolute",
    bottom: "0.5rem",
    left: "0.5rem",
    right: "0.5rem",
    width: "auto",
    height: "0.25rem",
    overflow: "hidden",
    backgroundColor: "rgba(0, 0, 0, 0.5)",
    borderRadius: "0.125rem",
});

export const progressFill = style({
    height: "100%",
    backgroundColor: vars.color.primary,
});

export const menuButton = style({
    position: "absolute",
    top: "0.5rem",
    right: "0.5rem",
    width: "2rem",
    height: "2rem",
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    backgroundColor: "rgba(0, 0, 0, 0.6)",
    color: "white",
    border: "none",
    borderRadius: "50%",
    cursor: "pointer",
    fontSize: "1.25rem",
    fontWeight: "bold",
    zIndex: 10,
    opacity: 0,
    transition: "opacity 0.2s, background-color 0.2s",
    ":hover": {
        backgroundColor: "rgba(0, 0, 0, 0.8)",
    },
});

globalStyle(`${tile}:hover ${menuButton}`, {
    opacity: 1,
});

globalStyle(`${tile}:hover ${qualityIndicators}`, {
    opacity: 1,
});

export const dropdown = style({
    position: "absolute",
    top: "3rem",
    right: "0.5rem",
    backgroundColor: vars.color.background,
    border: `1px solid ${vars.color.border}`,
    borderRadius: "0.5rem",
    boxShadow: "0 4px 12px rgba(0, 0, 0, 0.15)",
    minWidth: "9rem",
    zIndex: 20,
    overflow: "hidden",
});

export const dropdownHeader = style({
    padding: "0.75rem 1rem",
    fontSize: "0.75rem",
    fontWeight: "bold",
    textTransform: "uppercase",
    color: vars.color.text,
    opacity: 0.7,
    borderBottom: `1px solid ${vars.color.border}`,
});

export const dropdownItem = style({
    width: "100%",
    padding: "0.75rem 1rem",
    textAlign: "left",
    backgroundColor: "transparent",
    border: "none",
    color: vars.color.text,
    fontSize: "0.875rem",
    cursor: "pointer",
    transition: "background-color 0.2s",
    ":hover": {
        backgroundColor: vars.color.primary,
        color: "white",
    },
});
