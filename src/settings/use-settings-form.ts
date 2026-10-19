import { useCallback, useEffect, useState } from "react";

import { api, errorText } from "@/shared/api";
import type { Settings, StylePreset } from "@/shared/settings";

export type Status = { kind: "idle" | "busy" | "ok" | "error"; text: string };

const TEST_TEXT = "The quick brown fox jumps over the lazy dog.";

export function useSettingsForm() {
  const [settings, setSettings] = useState<Settings | null>(null);
  const [styles, setStyles] = useState<StylePreset[]>([]);
  const [status, setStatus] = useState<Status>({ kind: "idle", text: "" });

  useEffect(() => {
    Promise.all([api.getSettings(), api.stylePresets()]).then(([s, presets]) => {
      setSettings(s);
      setStyles(presets);
    });
  }, []);

  const update = useCallback((patch: Partial<Settings>) => setSettings((s) => (s ? { ...s, ...patch } : s)), []);

  const run = async (busyText: string, action: () => Promise<string>) => {
    setStatus({ kind: "busy", text: busyText });
    try {
      setStatus({ kind: "ok", text: await action() });
    } catch (e) {
      setStatus({ kind: "error", text: errorText(e) });
    }
  };

  const save = () => settings && run("Saving...", () => api.saveSettings(settings).then(() => "Saved."));
  const test = () => settings && run("Translating a test sentence...", () => api.translate(TEST_TEXT, settings.style, settings));

  return { settings, styles, status, update, save, test };
}
