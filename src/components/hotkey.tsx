import { Kbd, KbdGroup } from "@/components/ui/kbd";

/** Renders an accelerator such as "Ctrl+Shift+KeyT" as key caps. */
export function Hotkey({ accelerator }: { accelerator: string }) {
  return (
    <KbdGroup>
      {accelerator.split("+").map((key) => (
        <Kbd key={key}>{key.replace(/^(Key|Digit)/, "")}</Kbd>
      ))}
    </KbdGroup>
  );
}
