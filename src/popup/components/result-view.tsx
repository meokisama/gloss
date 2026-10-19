import { CircleAlert } from "lucide-react";

import { Hotkey } from "@/components/hotkey";
import type { TranslationResult } from "@/popup/use-translation";

const SKELETON_WIDTHS = ["w-11/12", "w-full", "w-3/5"];

export function ResultView({ result, hotkey }: { result: TranslationResult; hotkey: string }) {
  switch (result.status) {
    case "idle":
      return (
        <p className="text-muted">
          Select some text, then press <Hotkey accelerator={hotkey} />
        </p>
      );
    case "loading":
      return (
        <div role="status" aria-label="Translating" className="flex flex-col gap-2.5 py-1">
          {SKELETON_WIDTHS.map((w) => (
            <span key={w} className={`h-[0.65em] animate-pulse rounded-sm bg-line ${w}`} />
          ))}
        </div>
      );
    case "error":
      return (
        <p className="flex gap-2 text-danger">
          <CircleAlert className="mt-[0.2em] size-[1em] shrink-0" />
          <span className="wrap-break-word whitespace-pre-wrap select-text">{result.text}</span>
        </p>
      );
    case "done":
      return <p className="leading-relaxed wrap-break-word whitespace-pre-wrap select-text">{result.text}</p>;
  }
}
