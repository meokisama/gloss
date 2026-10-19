import { Check, CircleAlert, LoaderCircle } from "lucide-react";

import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import type { Status } from "@/settings/use-settings-form";

const ICONS = {
  busy: [LoaderCircle, "animate-spin text-muted"],
  ok: [Check, "text-ok"],
  error: [CircleAlert, "text-danger"],
} as const;

export function Footer({ status, onTest }: { status: Status; onTest: () => void }) {
  const busy = status.kind === "busy";
  const [Icon, iconClass] = status.kind === "idle" ? [null, ""] : ICONS[status.kind];

  return (
    <footer className="flex shrink-0 items-start gap-2 border-t border-line px-8 py-3">
      <p
        role="status"
        className={cn(
          "flex max-h-24 min-w-0 flex-1 gap-2 overflow-y-auto py-1 text-sm whitespace-pre-wrap wrap-break-word select-text",
          status.kind === "error" ? "text-danger" : status.kind === "busy" ? "text-muted" : "text-ink",
        )}
      >
        {Icon && <Icon className={cn("mt-0.5 size-3.5 shrink-0", iconClass)} />}
        {status.text}
      </p>
      <Button onClick={onTest} disabled={busy}>
        Test
      </Button>
      <Button type="submit" variant="default" disabled={busy}>
        Save
      </Button>
    </footer>
  );
}
