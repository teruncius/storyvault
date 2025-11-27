import { style, keyframes } from "@vanilla-extract/css";
import { vars } from "@sv/fe/theme/vars.css";

const spin = keyframes({
    "0%": { transform: "rotate(0deg)" },
    "100%": { transform: "rotate(360deg)" },
});

export const container = style({
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    minHeight: "100vh",
    backgroundColor: vars.color.background,
    color: vars.color.text,
});

export const spinner = style({
    width: "50px",
    height: "50px",
    border: `4px solid ${vars.color.border}`,
    borderTop: `4px solid ${vars.color.primary}`,
    borderRadius: "50%",
    animation: `${spin} 1s linear infinite`,
});
