import { cva, type VariantProps } from "class-variance-authority";
import type { ComponentProps } from "react";

import { cn } from "@/lib/utils";

const buttonVariants = cva(
  "focus-ring inline-flex shrink-0 items-center justify-center gap-1.5 rounded-md text-sm font-medium transition-colors disabled:pointer-events-none disabled:opacity-50 [&_svg]:shrink-0",
  {
    variants: {
      variant: {
        default: "bg-ink text-surface hover:opacity-85",
        outline: "border border-line-strong bg-field text-ink hover:bg-hover",
        ghost: "text-muted hover:bg-hover hover:text-ink aria-pressed:text-accent aria-pressed:hover:text-accent",
      },
      size: {
        default: "h-7 px-3",
        icon: "size-7 [&_svg]:size-[15px]",
      },
    },
    defaultVariants: { variant: "outline", size: "default" },
  },
);

export function Button({ className, variant, size, type = "button", ...props }: ComponentProps<"button"> & VariantProps<typeof buttonVariants>) {
  return <button type={type} className={cn(buttonVariants({ variant, size }), className)} {...props} />;
}
