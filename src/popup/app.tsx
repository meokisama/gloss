import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { useEffect, useRef, useState, type CSSProperties } from "react";

import { ReplaceBar } from "@/popup/components/replace-bar";
import { ResultView } from "@/popup/components/result-view";
import { Toolbar } from "@/popup/components/toolbar";
import { useAutoResize } from "@/popup/use-auto-resize";
import { useDismiss } from "@/popup/use-dismiss";
import { usePopupSession } from "@/popup/use-popup-session";
import { useReplace } from "@/popup/use-replace";
import { useTranslation } from "@/popup/use-translation";
import { api } from "@/shared/api";
import type { StylePreset } from "@/shared/settings";
import { useTheme } from "@/shared/use-theme";

export function App() {
  const { settings, request, sessionId, setStyle, retry } = usePopupSession();
  const result = useTranslation(request);
  const { replace, error: replaceError } = useReplace(request, result, settings?.writeMode);
  const [styles, setStyles] = useState<StylePreset[]>([]);
  const [pinned, setPinned] = useState(false);
  const cardRef = useRef<HTMLDivElement>(null);
  const movedRef = useRef(false);

  useEffect(() => void api.stylePresets().then(setStyles), []);
  useEffect(() => void (movedRef.current = false), [sessionId]);
  useEffect(() => {
    if (result.status === "done" && settings?.autoCopy) void writeText(result.text);
  }, [result, settings?.autoCopy]);

  useTheme(settings?.theme);
  useDismiss(pinned || !settings?.closeOnBlur);
  useAutoResize(cardRef, movedRef);

  const hotkeys = settings && { translate: settings.hotkey, write: settings.writeHotkey, explain: settings.explainHotkey };
  const cardStyle = settings && ({ "--font-size": `${settings.fontSize}px`, "--max-height": `${settings.popupMaxHeight}px` } as CSSProperties);

  return (
    <div
      ref={cardRef}
      style={cardStyle ?? undefined}
      className="flex max-h-(--max-height) flex-col overflow-hidden rounded-xl border border-line-strong bg-surface text-ink"
    >
      {settings && hotkeys && request && (
        <>
          <Toolbar
            styles={styles}
            style={request.mode === "explain" ? null : request.style}
            onStyle={setStyle}
            copyText={result.status === "done" ? result.text : null}
            onRetry={retry}
            pinned={pinned}
            onPin={() => setPinned((p) => !p)}
            onClose={api.hidePopup}
            onDragStart={() => (movedRef.current = true)}
          />
          <div className="overflow-y-auto px-4 pt-3 pb-4 text-(length:--font-size)">
            {settings.showSource && request.text && (
              <p className="mb-3 line-clamp-3 border-l-2 border-accent/50 pl-2.5 text-[0.9em] wrap-break-word whitespace-pre-wrap text-muted">
                {request.text}
              </p>
            )}
            <ResultView result={result} hotkey={hotkeys[request.mode]} markdown={request.mode === "explain"} />
            {request.mode === "write" && <ReplaceBar lang={settings.writeLang} onReplace={replace} error={replaceError} />}
          </div>
        </>
      )}
    </div>
  );
}
