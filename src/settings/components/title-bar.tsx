import { getCurrentWindow } from "@tauri-apps/api/window";
import { Minus, Square, X, type LucideIcon } from "lucide-react";

import { cn } from "@/lib/utils";
import logo from "../../../src-tauri/icons/128x128.png";

const win = getCurrentWindow();

function WindowButton({ label, icon: Icon, onClick, danger }: { label: string; icon: LucideIcon; onClick: () => void; danger?: boolean }) {
  return (
    <button
      type="button"
      aria-label={label}
      title={label}
      onClick={onClick}
      className={cn(
        "grid h-9 w-11 cursor-default place-items-center text-muted transition-colors",
        danger ? "hover:bg-[#c42b1c] hover:text-white" : "hover:bg-hover hover:text-ink",
      )}
    >
      <Icon className="size-3.5" strokeWidth={1.5} />
    </button>
  );
}

export function TitleBar() {
  return (
    <header data-tauri-drag-region className="flex h-9 shrink-0 items-center justify-between">
      <span className="pointer-events-none flex items-center gap-2 pl-3 text-sm font-semibold tracking-tight">
        <img src={logo} alt="" className="size-4.5" />
        Gloss
      </span>
      <div className="flex">
        <WindowButton label="Minimize" icon={Minus} onClick={() => win.minimize()} />
        <WindowButton label="Maximize" icon={Square} onClick={() => win.toggleMaximize()} />
        <WindowButton label="Close" icon={X} onClick={() => win.close()} danger />
      </div>
    </header>
  );
}
