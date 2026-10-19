import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";
import type { StylePreset } from "@/shared/settings";

interface StylePickerProps {
  styles: StylePreset[];
  style: string;
  onStyle: (style: string) => void;
}

/** A chip row instead of a dropdown: the popup window is only as tall as its content, so a menu would be clipped. */
export function StylePicker({ styles, style, onStyle }: StylePickerProps) {
  return (
    <ToggleGroup
      type="single"
      aria-label="Style"
      value={style}
      onValueChange={(v) => v && onStyle(v)}
      onWheel={(e) => (e.currentTarget.scrollLeft += e.deltaY)}
      data-tauri-drag-region
      className="min-w-0 overflow-x-auto scrollbar-none"
    >
      {styles.map((s) => (
        <ToggleGroupItem key={s.id} value={s.id} className="px-2">
          {s.label}
        </ToggleGroupItem>
      ))}
    </ToggleGroup>
  );
}
