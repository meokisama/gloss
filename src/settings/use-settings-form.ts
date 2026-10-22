import { listen } from "@tauri-apps/api/event";
import { useCallback, useEffect, useState } from "react";

import { api, errorText } from "@/shared/api";
import type { Settings, StylePreset } from "@/shared/settings";

export type Status = { kind: "idle" | "busy" | "ok" | "error"; text: string };

const TEST_TEXT = "The quick brown fox jumps over the lazy dog.";

export function useSettingsForm() {
  const [settings, setSettings] = useState<Settings | null>(null);
  const [styles, setStyles] = useState<StylePreset[]>([]);
  const [languages, setLanguages] = useState<string[]>([]);
  const [status, setStatus] = useState<Status>({ kind: "idle", text: "" });

  useEffect(() => {
    Promise.all([api.getSettings(), api.stylePresets(), api.languages()]).then(([s, presets, langs]) => {
      setSettings(s);
      setStyles(presets);
      setLanguages(langs);
    });
  }, []);

  const update = useCallback((patch: Partial<Settings>) => setSettings((s) => (s ? { ...s, ...patch } : s)), []);

  // The tray changes saved settings; merging only its fields keeps unsaved edits here.
  useEffect(() => {
    const unlisten = listen<Partial<Settings>>("settings:patch", ({ payload }) => update(payload));
    return () => void unlisten.then((stop) => stop());
  }, [update]);

  const run = async (busyText: string, action: () => Promise<string>) => {
    setStatus({ kind: "busy", text: busyText });
    try {
      setStatus({ kind: "ok", text: await action() });
    } catch (e) {
      setStatus({ kind: "error", text: errorText(e) });
    }
  };

  const save = () => settings && run("Saving...", () => api.saveSettings(settings).then(() => "Saved."));
  const test = () => settings && run("Translating a test sentence...", () => api.translate(TEST_TEXT, settings.style, "translate", { settings }));

  return { settings, styles, languages, status, update, save, test };
}
