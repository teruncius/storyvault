import { darkTheme } from "@sv/fe/theme/dark.css";
import { lightTheme } from "@sv/fe/theme/light.css";
import { createContext, useCallback, useContext, useState } from "react";
import type { PropsWithChildren } from "react";

const Theme = {
    DARK: darkTheme,
    LIGHT: lightTheme,
} as const;

const Icons = {
    [Theme.DARK]: "☼",
    [Theme.LIGHT]: "☼",
} as const;

const ThemeContext = createContext({
    theme: Theme.DARK,
    icon: Icons[Theme.DARK],
    toggle: () => { },
});

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

export const useTheme = () => {
    return useContext(ThemeContext);
};
