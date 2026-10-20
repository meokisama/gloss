/** Mirrors `Settings` in src-tauri/src/settings.rs (camelCase over the wire). */
export interface Settings {
  hotkey: string;
  writeHotkey: string;
  launchAtLogin: boolean;
  apiFormat: "openai" | "anthropic" | "gemini";
  baseUrl: string;
  /** Keyed by base URL. */
  apiKeys: Record<string, string>;
  model: string;
  effort: string;
  timeoutSecs: number;
  targetLang: string;
  fallbackLang: string;
  style: string;
  customInstructions: string;
  systemPrompt: string;
  maxChars: number;
  writeLang: string;
  writeMode: "preview" | "replace";
  popupPosition: "cursor" | "aboveCursor" | "center" | "topRight" | "remember";
  popupOffset: number;
  popupWidth: number;
  popupMaxHeight: number;
  fontSize: number;
  theme: "system" | "light" | "dark";
  closeOnBlur: boolean;
  autoCopy: boolean;
  showSource: boolean;
  lastPopupPos: [number, number] | null;
}

export interface StylePreset {
  id: string;
  label: string;
}
