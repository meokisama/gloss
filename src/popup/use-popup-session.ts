import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

import { api } from "@/shared/api";
import type { Settings } from "@/shared/settings";

export interface Request {
  text: string;
  style: string;
}

/** One session per hotkey press: fresh settings plus the captured text. */
export function usePopupSession() {
  const [settings, setSettings] = useState<Settings | null>(null);
  const [request, setRequest] = useState<Request | null>(null);
  const [sessionId, setSessionId] = useState(0);

  useEffect(() => {
    const unlisten = listen<string>("popup:source", async ({ payload }) => {
      const fresh = await api.getSettings();
      setSettings(fresh);
      setRequest({ text: payload, style: fresh.style });
      setSessionId((id) => id + 1);
    });
    return () => void unlisten.then((stop) => stop());
  }, []);

  const setStyle = (style: string) => setRequest((r) => r && { ...r, style });
  const retry = () => setRequest((r) => r && { ...r });

  return { settings, request, sessionId, setStyle, retry };
}
