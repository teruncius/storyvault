import { Theme, Icons } from "@sv/fe/theme/theme-provider";
import { createContext } from "react";

export const ThemeContext = createContext({
    theme: Theme.DARK,
    icon: Icons[Theme.DARK],
    toggle: () => {},
});
