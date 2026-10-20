import { ChevronDown } from "lucide-react";

import type { StylePreset } from "@/shared/settings";

interface StylePickerProps {
  styles: StylePreset[];
  style: string;
  onStyle: (style: string) => void;
}

/** A native select: the OS draws its list outside the window, while a Radix menu would be clipped by the content-sized popup. */
export function StylePicker({ styles, style, onStyle }: StylePickerProps) {
  return (
    <div className="relative shrink-0">
      <select
        aria-label="Style"
        value={style}
        onChange={(e) => onStyle(e.target.value)}
        className="focus-ring h-7 cursor-pointer appearance-none rounded-md bg-hover pr-7 pl-2.5 text-xs font-medium text-ink"
      >
        {styles.map((s) => (
          <option key={s.id} value={s.id}>
            {s.label}
          </option>
        ))}
      </select>
      <ChevronDown className="pointer-events-none absolute top-1/2 right-2 size-3.5 -translate-y-1/2 text-muted" />
    </div>
  );
}
