import { ToggleGroup as ToggleGroupPrimitive } from "radix-ui";
import type { ComponentProps } from "react";

import { cn } from "@/lib/utils";

export function ToggleGroup({ className, ...props }: ComponentProps<typeof ToggleGroupPrimitive.Root>) {
  return <ToggleGroupPrimitive.Root className={cn("inline-flex items-center gap-0.5", className)} {...props} />;
}

export function ToggleGroupItem({ className, ...props }: ComponentProps<typeof ToggleGroupPrimitive.Item>) {
  return (
    <ToggleGroupPrimitive.Item
      className={cn(
        "focus-ring h-6 shrink-0 rounded-md px-2.5 text-xs font-medium text-muted transition-colors hover:text-ink data-[state=on]:bg-hover data-[state=on]:text-ink",
        className,
      )}
      {...props}
    />
  );
}
