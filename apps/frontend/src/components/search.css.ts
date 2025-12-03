import { style } from "@vanilla-extract/css";
import { vars } from "@sv/fe/theme/vars.css";

export const searchForm = style({
    position: "relative",
    display: "flex",
    alignItems: "center",
    gap: "0.5rem",
    width: "100%",
    maxWidth: "600px",
    margin: "0 auto",
});

export const searchInputWrapper = style({
    position: "relative",
    flex: 1,
    display: "flex",
    alignItems: "center",
});

export const searchIcon = style({
    position: "absolute",
    left: "1rem",
    width: "1.25rem",
    height: "1.25rem",
    color: vars.color.text,
    opacity: 0.5,
    pointerEvents: "none",
    zIndex: 1,
});

export const searchInput = style({
    width: "100%",
    padding: "0.875rem 1rem 0.875rem 3rem",
    fontSize: "1rem",
    color: vars.color.text,
    backgroundColor: vars.color.backdrop,
    border: `2px solid ${vars.color.border}`,
    borderRadius: "12px",
    outline: "none",
    transition: "all 0.3s ease",
    boxShadow: "0 2px 8px rgba(0, 0, 0, 0.05)",

    ":focus": {
        borderColor: vars.color.primary,
        boxShadow: `0 4px 12px rgba(0, 0, 0, 0.1), 0 0 0 3px ${vars.color.border}`,
        transform: "translateY(-1px)",
    },

    "::placeholder": {
        color: vars.color.text,
        opacity: 0.5,
    },
});

export const searchButton = style({
    padding: "0.875rem 1.75rem",
    fontSize: "1rem",
    fontWeight: "600",
    color: vars.color.text,
    backgroundColor: "transparent",
    border: `1px solid ${vars.color.border}`,
    borderRadius: "12px",
    cursor: "pointer",
    transition: "all 0.3s ease",
    whiteSpace: "nowrap",
    display: "flex",
    alignItems: "center",
    gap: "0.5rem",

    ":hover": {
        backgroundColor: vars.color.border,
    },

    ":active": {
        transform: "scale(0.98)",
    },

    ":disabled": {
        opacity: 0.5,
        cursor: "not-allowed",
        transform: "none",
    },
});

export const clearButton = style({
    position: "absolute",
    right: "1rem",
    width: "1.5rem",
    height: "1.5rem",
    padding: 0,
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    color: vars.color.text,
    backgroundColor: "transparent",
    border: "none",
    borderRadius: "50%",
    cursor: "pointer",
    opacity: 0.5,
    transition: "all 0.2s ease",
    zIndex: 1,

    ":hover": {
        opacity: 1,
        backgroundColor: vars.color.border,
    },

    ":active": {
        transform: "scale(0.95)",
    },
});
