import { darkTheme } from "@sv/fe/theme/dark.css";
import { lightTheme } from "@sv/fe/theme/light.css";
import { ThemeContext } from "@sv/fe/theme/theme-context";
import { useCallback, useState } from "react";
import type { PropsWithChildren } from "react";

export const Theme = {
    DARK: darkTheme,
    LIGHT: lightTheme,
} as const;

export const Icons = {
    [Theme.DARK]: "☼",
    [Theme.LIGHT]: "☼",
} as const;

export function ThemeProvider({ children }: PropsWithChildren) {
    const [theme, setTheme] = useState(Theme.DARK);

    const toggle = useCallback(() => {
        setTheme((prev) => (prev === Theme.DARK ? Theme.LIGHT : Theme.DARK));
    }, []);

    return (
        <ThemeContext value={{ theme, toggle, icon: Icons[theme] }}>
            <div className={theme}>{children}</div>
        </ThemeContext>
    );
}
