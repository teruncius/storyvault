import { vars } from "@storyvault/frontend/theme/vars.css";
import { style } from "@vanilla-extract/css";

export const avatar = style({
    borderRadius: "50%",
    background: vars.color.backdrop,
    width: 40,
    height: 40,
    display: "flex",
    justifyContent: "center",
    alignItems: "center",
    fontSize: "1rem",
    fontWeight: "bold",
    color: vars.color.text,
});
