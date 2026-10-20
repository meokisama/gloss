import { Pin, RotateCw, X } from "lucide-react";
import type { MouseEvent } from "react";

import { Button } from "@/components/ui/button";
import { CopyButton } from "@/popup/components/copy-button";
import { StylePicker } from "@/popup/components/style-picker";
import type { StylePreset } from "@/shared/settings";

interface ToolbarProps {
  styles: StylePreset[];
  /** Null when explaining, which has no style. */
  style: string | null;
  onStyle: (style: string) => void;
  copyText: string | null;
  onRetry: () => void;
  pinned: boolean;
  onPin: () => void;
  onClose: () => void;
  onDragStart: () => void;
}

export function Toolbar({ styles, style, onStyle, copyText, onRetry, pinned, onPin, onClose, onDragStart }: ToolbarProps) {
  const onMouseDown = (e: MouseEvent) => (e.target as HTMLElement).hasAttribute("data-tauri-drag-region") && onDragStart();

  return (
    <header data-tauri-drag-region onMouseDown={onMouseDown} className="flex h-10 shrink-0 items-center gap-0.5 border-b border-line pl-2 pr-1.5">
      {style === null ? (
        <span data-tauri-drag-region className="pl-1 text-xs font-medium text-muted">
          Explain
        </span>
      ) : (
        <StylePicker styles={styles} style={style} onStyle={onStyle} />
      )}
      <span data-tauri-drag-region className="min-w-3 flex-1 self-stretch" />
      {copyText !== null && <CopyButton text={copyText} />}
      <Button variant="ghost" size="icon" aria-label="Try again" title="Try again" onClick={onRetry}>
        <RotateCw />
      </Button>
      <span className="mx-1 h-4 w-px shrink-0 bg-line" />
      <Button variant="ghost" size="icon" aria-label="Keep open" title="Keep open" aria-pressed={pinned} onClick={onPin}>
        <Pin fill={pinned ? "currentColor" : "none"} />
      </Button>
      <Button variant="ghost" size="icon" aria-label="Close (Esc)" title="Close (Esc)" onClick={onClose}>
        <X />
      </Button>
    </header>
  );
}
