/** Mirrors `Settings` in src-tauri/src/settings.rs (camelCase over the wire). */
export interface Settings {
  hotkey: string;
  launchAtLogin: boolean;
  provider: "openai" | "anthropic" | "gemini";
  baseUrl: string;
  apiKey: string;
  model: string;
  effort: string;
  timeoutSecs: number;
  targetLang: string;
  fallbackLang: string;
  style: string;
  customInstructions: string;
  systemPrompt: string;
  maxChars: number;
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
