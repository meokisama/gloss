import type { ComponentProps } from "react";

import { cn } from "@/lib/utils";

export function InputGroup({ className, ...props }: ComponentProps<"div">) {
  return <div className={cn("relative [&>input]:pr-8", className)} {...props} />;
}

/** Sits inside the input's right edge. Clicks pass through to the input unless they land on a button. */
export function InputGroupAddon({ className, ...props }: ComponentProps<"div">) {
  return (
    <div
      className={cn("pointer-events-none absolute inset-y-0 right-1 flex items-center text-xs text-faint [&>button]:pointer-events-auto", className)}
      {...props}
    />
  );
}
