import type { ReactNode } from "react";

import { cn } from "@/lib/utils";

const WIDTHS = { auto: "", narrow: "w-28", medium: "w-44", wide: "w-60" };

interface RowProps {
  label: string;
  hint?: ReactNode;
  htmlFor?: string;
  width?: keyof typeof WIDTHS;
  /** Control goes under the label instead of beside it. */
  stacked?: boolean;
  action?: ReactNode;
  children: ReactNode;
}

export function Row({ label, hint, htmlFor, width = "auto", stacked, action, children }: RowProps) {
  const text = (
    <div className="min-w-0 flex-1">
      <label htmlFor={htmlFor} className="text-sm text-ink">
        {label}
      </label>
      {hint && <p className="mt-0.5 text-xs text-muted">{hint}</p>}
    </div>
  );

  if (stacked) {
    return (
      <div className="flex flex-col gap-2.5 px-4 py-3">
        <div className="flex items-start gap-4">
          {text}
          {action}
        </div>
        {children}
      </div>
    );
  }

  return (
    <div className="flex items-center gap-6 px-4 py-3">
      {text}
      <div className={cn("shrink-0", WIDTHS[width])}>{children}</div>
    </div>
  );
}
