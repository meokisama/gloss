import { CornerDownLeft } from "lucide-react";

import { Button } from "@/components/ui/button";

interface ReplaceBarProps {
  lang: string;
  onReplace: (() => void) | null;
  error: string | null;
}

export function ReplaceBar({ lang, onReplace, error }: ReplaceBarProps) {
  return (
    <div className="mt-3 border-t border-line pt-3 text-xs">
      <div className="flex items-center gap-2">
        <span className="min-w-0 flex-1 text-muted">Into {lang}</span>
        {onReplace && (
          <Button variant="default" title="Replace the selection (Enter)" onClick={onReplace}>
            Replace
            <CornerDownLeft className="size-3.5" />
          </Button>
        )}
      </div>
      {error && <p className="mt-2 wrap-break-word text-danger select-text">{error}</p>}
    </div>
  );
}
