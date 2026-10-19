import { getCurrentWindow } from "@tauri-apps/api/window";
import { useEffect } from "react";

import { api } from "@/shared/api";

/** Escape always hides; losing focus hides unless `keepOnBlur`. */
export function useDismiss(keepOnBlur: boolean) {
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => e.key === "Escape" && api.hidePopup();
    window.addEventListener("keydown", onKey);
    const unlisten = getCurrentWindow().onFocusChanged(({ payload: focused }) => {
      if (!focused && !keepOnBlur) api.hidePopup();
    });
    return () => {
      window.removeEventListener("keydown", onKey);
      void unlisten.then((stop) => stop());
    };
  }, [keepOnBlur]);
}
