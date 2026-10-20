import { useState } from "react";

import { api, errorText } from "@/shared/api";
import type { Settings } from "@/shared/settings";

export type ModelList = { status: "idle" | "loading" } | { status: "done"; models: string[] } | { status: "error"; text: string };

/** Module scope, so a list survives tab switches and hiding the window; errors are not kept, so they retry. */
const cache = new Map<string, string[]>();

/** Fetches on demand, not per keystroke, once per API format, base URL and key. */
export function useModels(settings: Settings) {
  const key = [settings.apiFormat, settings.baseUrl, settings.apiKeys[settings.baseUrl] ?? ""].join("\n");
  const [state, setState] = useState<{ key: string; list: ModelList }>({ key: "", list: { status: "idle" } });
  const cached = cache.get(key);
  const list: ModelList = cached ? { status: "done", models: cached } : state.key === key ? state.list : { status: "idle" };

  const load = () => {
    if (list.status === "loading" || list.status === "done") return;
    setState({ key, list: { status: "loading" } });
    const settle = (next: ModelList) => setState((prev) => (prev.key === key ? { key, list: next } : prev));
    api.listModels(settings).then(
      (models) => {
        cache.set(key, models);
        settle({ status: "done", models });
      },
      (e) => settle({ status: "error", text: errorText(e) }),
    );
  };

  return { list, load };
}
