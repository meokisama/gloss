import { invoke } from "@tauri-apps/api/core";

import type { Settings, StylePreset } from "@/shared/settings";

export const api = {
  getSettings: () => invoke<Settings>("get_settings"),
  defaultSettings: () => invoke<Settings>("default_settings"),
  stylePresets: () => invoke<StylePreset[]>("style_presets"),
  saveSettings: (settings: Settings) => invoke<void>("save_settings", { new: settings }),
  translate: (text: string, style: string, settings?: Settings) => invoke<string>("translate", { text, style, settings }),
  resizePopup: (height: number, keepPosition: boolean) => invoke<void>("resize_popup", { height, keepPosition }),
  hidePopup: () => invoke<void>("hide_popup"),
};

export const errorText = (e: unknown) => (typeof e === "string" ? e : e instanceof Error ? e.message : String(e));
