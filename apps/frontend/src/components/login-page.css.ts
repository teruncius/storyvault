import { style } from "@vanilla-extract/css";
import { vars } from "@sv/fe/theme/vars.css";

export const container = style({
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    minHeight: "100vh",
    backgroundColor: vars.color.background,
    padding: "2rem",
});

export const form = style({
    display: "flex",
    flexDirection: "column",
    gap: "1.5rem",
    width: "100%",
    maxWidth: "400px",
    padding: "2.5rem",
    backgroundColor: vars.color.backdrop,
    borderRadius: "12px",
    border: `0`,
    boxShadow: "0 4px 6px rgba(0, 0, 0, 0.1)",
});

export const title = style({
    fontSize: "2rem",
    fontWeight: "bold",
    color: vars.color.text,
    textAlign: "center",
    marginBottom: "0.5rem",
});

export const inputGroup = style({
    display: "flex",
    flexDirection: "column",
    gap: "0.5rem",
});

export const label = style({
    fontSize: "0.875rem",
    fontWeight: "500",
    color: vars.color.text,
    opacity: 0.9,
});

export const input = style({
    padding: "0.75rem 1rem",
    fontSize: "1rem",
    color: vars.color.text,
    backgroundColor: vars.color.background,
    border: `1px solid ${vars.color.border}`,
    borderRadius: "8px",
    outline: "none",
    transition: "border-color 0.2s, box-shadow 0.2s",

    ":focus": {
        borderColor: vars.color.text,
        boxShadow: `0 0 0 3px ${vars.color.border}`,
    },

    "::placeholder": {
        color: vars.color.text,
        opacity: 0.5,
    },
});

export const button = style({
    padding: "0.875rem 1.5rem",
    fontSize: "1rem",
    fontWeight: "600",
    color: vars.color.background,
    backgroundColor: vars.color.text,
    border: "none",
    borderRadius: "8px",
    cursor: "pointer",
    transition: "opacity 0.2s, transform 0.1s",
    marginTop: "0.5rem",

    ":active": {
        transform: "scale(0.98)",
    },

    ":disabled": {
        opacity: 0.5,
        cursor: "not-allowed",
        transform: "none",
    },
});

export const error = style({
    padding: "0.75rem 1rem",
    fontSize: "0.875rem",
    color: "#ef4444",
    backgroundColor: "rgba(239, 68, 68, 0.1)",
    border: "1px solid rgba(239, 68, 68, 0.3)",
    borderRadius: "8px",
    textAlign: "center",
});
