import { style } from "@vanilla-extract/css";

export const twocol = style({
    display: "flex",
    flexDirection: "row",
    gap: "1rem",
});

export const sidebar = style({
    flex: 1,
    display: "flex",
    flexDirection: "column",
    gap: "1rem",
    minWidth: 0,
});

export const content = style({
    flex: 3,
});

export const emptyState = style({
    display: "flex",
    justifyContent: "center",
    alignItems: "center",
    padding: "3rem",
    fontSize: "1.25rem",
    color: "#666",
    textAlign: "center",
});
