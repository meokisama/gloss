import { invoke } from "@tauri-apps/api/core";

import type { Settings, StylePreset } from "@/shared/settings";

export const api = {
  getSettings: () => invoke<Settings>("get_settings"),
  defaultSettings: () => invoke<Settings>("default_settings"),
  stylePresets: () => invoke<StylePreset[]>("style_presets"),
  saveSettings: (settings: Settings) => invoke<void>("save_settings", { new: settings }),
  languages: () => invoke<string[]>("languages"),
  translate: (text: string, style: string, settings?: Settings, writeLang?: string) =>
    invoke<string>("translate", { text, style, settings, writeLang }),
  replaceSelection: (text: string) => invoke<void>("replace_selection", { text }),
  listModels: (settings: Settings) => invoke<string[]>("list_models", { settings }),
  resizePopup: (height: number, keepPosition: boolean) => invoke<void>("resize_popup", { height, keepPosition }),
  hidePopup: () => invoke<void>("hide_popup"),
};

export const errorText = (e: unknown) => (typeof e === "string" ? e : e instanceof Error ? e.message : String(e));
