import { vars } from "@sv/fe/theme/vars.css";
import { style, styleVariants } from "@vanilla-extract/css";

export const container = style({
    display: "flex",
    flexDirection: "column",
    minHeight: "100vh",
    background: vars.color.backdrop,
    scrollbarGutter: "stable",
});

export const header = style({
    display: "flex",
    justifyContent: "center",
    alignItems: "center",
    height: 64,
    backgroundColor: vars.color.background,
    borderBottom: `1px solid ${vars.color.border}`,
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
    borderTop: `1px solid ${vars.color.border}`,
    padding: "0 1rem 0 1rem",
    position: "sticky",
    zIndex: 1,
    bottom: 0,
});

export const center = styleVariants({
    header: {
        display: "flex",
        flexDirection: "row",
        width: 1200,
        justifyContent: "space-between",
    },
    main: {
        display: "flex",
        flexDirection: "column",
        width: 1200,
    },
    footer: {
        display: "flex",
        flexDirection: "column",
        width: 1200,
    },
});

export const area = styleVariants({
    left: {
        display: "flex",
        flexDirection: "row",
        alignItems: "center",
        gap: "1.5rem",
    },
    right: {
        display: "flex",
        flexDirection: "row",
        gap: "1rem",
        alignItems: "center",
    },
});

export const nav = style({
    display: "flex",
    flexDirection: "row",
    gap: "1rem",
});

export const navLink = style({
    color: vars.color.text,
    fontSize: "1rem",
    fontWeight: "bold",
    textDecoration: "none",
    ":hover": {
        textDecoration: "underline",
    },
});

export const toggleTheme = style({
    cursor: "pointer",
    background: "transparent",
    border: `1px solid ${vars.color.border}`,
    color: vars.color.text,
    fontSize: "1rem",
    fontWeight: "bold",
    padding: "0.5rem",
    borderRadius: "0.25rem",
});

export const logout = style({
    cursor: "pointer",
    background: "transparent",
    border: `1px solid ${vars.color.border}`,
    color: vars.color.text,
    fontSize: "1rem",
    fontWeight: "bold",
    padding: "0.5rem 1rem 0.5rem 1rem",
    borderRadius: "0.25rem",
});

export const userInfo = style({
    display: "grid",
    gridTemplateAreas: `
        "avatar name"
        "avatar email"
    `,
    gap: "0 0.5rem",
});

export const avatar = style({
    gridArea: "avatar",
});

export const userName = style({
    color: vars.color.text,
    fontSize: "0.875rem",
    fontWeight: "bold",
    gridArea: "name",
    alignSelf: "end",
});

export const userEmail = style({
    color: vars.color.text,
    fontSize: "0.75rem",
    opacity: 0.7,
    gridArea: "email",
    alignSelf: "start",
});
