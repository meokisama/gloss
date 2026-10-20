import { useCallback, useEffect, useState } from "react";

import type { Request } from "@/popup/use-popup-session";
import type { TranslationResult } from "@/popup/use-translation";
import { api, errorText } from "@/shared/api";
import type { Settings } from "@/shared/settings";

/** Writing sessions only: Enter pastes the translation over the selection; "replace" mode does it as soon as it arrives. */
export function useReplace(request: Request | null, result: TranslationResult, mode: Settings["writeMode"] | undefined) {
  const [error, setError] = useState<string | null>(null);
  const text = request?.writeLang && result.status === "done" ? result.text : null;

  const replace = useCallback(() => {
    if (text === null) return;
    setError(null);
    api.replaceSelection(text).catch((e) => setError(errorText(e)));
  }, [text]);

  useEffect(() => setError(null), [result]);
  useEffect(() => {
    if (mode === "replace") replace();
  }, [mode, replace]);
  useEffect(() => {
    if (text === null) return;
    // A focused button handles Enter itself.
    const onKey = (e: KeyboardEvent) => e.key === "Enter" && !e.isComposing && !(e.target instanceof HTMLButtonElement) && replace();
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [text, replace]);

  return { replace: text === null ? null : replace, error };
}
