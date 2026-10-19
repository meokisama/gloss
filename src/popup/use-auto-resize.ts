import { useEffect, type RefObject } from "react";

import { api } from "@/shared/api";

/** Keeps the window as tall as the card. `movedRef` is set once the user drags, so resizes stop re-placing it. */
export function useAutoResize(ref: RefObject<HTMLElement | null>, movedRef: RefObject<boolean>) {
  useEffect(() => {
    const element = ref.current;
    if (!element) return;
    const observer = new ResizeObserver(() => api.resizePopup(element.offsetHeight, movedRef.current));
    observer.observe(element);
    return () => observer.disconnect();
  }, [ref, movedRef]);
}
