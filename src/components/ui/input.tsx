import type { ComponentProps } from "react";

import { cn } from "@/lib/utils";

export function Input({ className, ...props }: ComponentProps<"input">) {
  return (
    <input
      className={cn(
        "focus-ring h-7 w-full rounded-md border border-line-strong bg-field px-2.5 text-sm text-ink tabular-nums placeholder:text-faint focus-visible:border-accent",
        className,
      )}
      {...props}
    />
  );
}
