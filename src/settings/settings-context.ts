import { createContext, useContext } from "react";

import type { Settings, StylePreset } from "@/shared/settings";

interface SettingsContextValue {
  settings: Settings;
  styles: StylePreset[];
  update: (patch: Partial<Settings>) => void;
}

export const SettingsContext = createContext<SettingsContextValue | null>(null);

export function useSettings() {
  const value = useContext(SettingsContext);
  if (!value) throw new Error("useSettings outside SettingsContext");
  return value;
}
