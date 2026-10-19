import { useEffect } from "react";

import type { Settings } from "@/shared/settings";

export function useTheme(theme: Settings["theme"] | undefined) {
  useEffect(() => {
    const root = document.documentElement;
    if (!theme || theme === "system") delete root.dataset.theme;
    else root.dataset.theme = theme;
  }, [theme]);
}
