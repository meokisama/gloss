import { useEffect, useState } from "react";

import type { Request } from "@/popup/use-popup-session";
import { api, errorText } from "@/shared/api";

/** `streaming` holds the reply so far; only `done` is final and safe to copy or paste. */
export type TranslationResult = { status: "idle" | "loading" } | { status: "streaming" | "done" | "error"; text: string };

/** Runs whenever `request` changes identity, so passing a copy retries. */
export function useTranslation(request: Request | null): TranslationResult {
  const [result, setResult] = useState<TranslationResult>({ status: "idle" });

  useEffect(() => {
    if (!request?.text) return setResult({ status: "idle" });
    let settled = false;
    let partial = "";
    const settle = (next: TranslationResult) => {
      if (settled) return;
      settled = true;
      setResult(next);
    };
    const onDelta = (delta: string) => {
      if (settled) return;
      partial += delta;
      // The final text is trimmed, so leading whitespace would only make the first line jump.
      const text = partial.trimStart();
      if (text) setResult({ status: "streaming", text });
    };
    setResult({ status: "loading" });
    api.translate(request.text, request.style, request.mode, { onDelta }).then(
      (text) => settle({ status: "done", text }),
      (e) => settle({ status: "error", text: errorText(e) }),
    );
    return () => {
      settled = true;
    };
  }, [request]);

  return result;
}
