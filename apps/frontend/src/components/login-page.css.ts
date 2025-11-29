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
