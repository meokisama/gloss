import { useEffect, useState } from "react";

import type { Request } from "@/popup/use-popup-session";
import { api, errorText } from "@/shared/api";

export type TranslationResult = { status: "idle" | "loading" } | { status: "done" | "error"; text: string };

/** Runs whenever `request` changes identity, so passing a copy retries. */
export function useTranslation(request: Request | null): TranslationResult {
  const [result, setResult] = useState<TranslationResult>({ status: "idle" });

  useEffect(() => {
    if (!request?.text) return setResult({ status: "idle" });
    let stale = false;
    setResult({ status: "loading" });
    api.translate(request.text, request.style, undefined, request.writeLang ?? undefined).then(
      (text) => !stale && setResult({ status: "done", text }),
      (e) => !stale && setResult({ status: "error", text: errorText(e) }),
    );
    return () => {
      stale = true;
    };
  }, [request]);

  return result;
}
