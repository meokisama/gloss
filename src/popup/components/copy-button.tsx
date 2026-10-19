import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { Check, Copy } from "lucide-react";
import { useEffect, useState } from "react";

import { Button } from "@/components/ui/button";

export function CopyButton({ text }: { text: string }) {
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    if (!copied) return;
    const timer = setTimeout(() => setCopied(false), 1200);
    return () => clearTimeout(timer);
  }, [copied]);

  const copy = () => writeText(text).then(() => setCopied(true));

  return (
    <Button variant="ghost" size="icon" aria-label="Copy translation" title="Copy translation" onClick={copy}>
      {copied ? <Check className="text-ok" /> : <Copy />}
    </Button>
  );
}
