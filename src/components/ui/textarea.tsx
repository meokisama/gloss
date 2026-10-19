import type { ComponentProps } from "react";

import { cn } from "@/lib/utils";

export function Textarea({ className, ...props }: ComponentProps<"textarea">) {
  return (
    <textarea
      className={cn(
        "focus-ring w-full resize-y rounded-md border border-line-strong bg-field px-2.5 py-2 text-sm leading-relaxed text-ink placeholder:text-faint focus-visible:border-accent",
        className,
      )}
      {...props}
    />
  );
}
