import { useState, type KeyboardEvent } from "react";

import { Hotkey } from "@/components/hotkey";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { Row } from "@/settings/components/row";
import { useSettings } from "@/settings/settings-context";

const MODIFIER_KEYS = ["Control", "Shift", "Alt", "Meta"];

/** Builds an accelerator in the format tauri-plugin-global-shortcut parses, e.g. "Ctrl+Shift+KeyT". */
function toAccelerator(e: KeyboardEvent): string | null {
  if (MODIFIER_KEYS.includes(e.key)) return null;
  const parts = [e.ctrlKey && "Ctrl", e.altKey && "Alt", e.shiftKey && "Shift", e.metaKey && "Super"].filter(Boolean);
  const isFunctionKey = /^F\d+$/.test(e.code);
  if (parts.length === 0 && !isFunctionKey) return null;
  return [...parts, e.code].join("+");
}

export function HotkeyInput({ name, label, hint }: { name: "hotkey" | "writeHotkey"; label: string; hint: string }) {
  const { settings, update } = useSettings();
  const [recording, setRecording] = useState(false);

  const onKeyDown = (e: KeyboardEvent<HTMLButtonElement>) => {
    if (e.key === "Tab") return;
    e.preventDefault();
    const accelerator = e.key === "Escape" ? settings[name] : toAccelerator(e);
    if (!accelerator) return;
    update({ [name]: accelerator });
    e.currentTarget.blur();
  };

  return (
    <Row label={label} hint={hint} htmlFor={name}>
      <Button
        id={name}
        className={cn("min-w-36 font-normal", recording && "border-accent")}
        onFocus={() => setRecording(true)}
        onBlur={() => setRecording(false)}
        onKeyDown={onKeyDown}
      >
        {recording ? <span className="text-muted">Press keys…</span> : <Hotkey accelerator={settings[name]} />}
      </Button>
    </Row>
  );
}
