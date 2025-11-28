import { ThemeContext } from "@sv/fe/theme/theme-context";
import { useContext } from "react";

export const useTheme = () => {
    return useContext(ThemeContext);
};
