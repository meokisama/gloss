import { ChevronDown } from "lucide-react";

import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/components/ui/dropdown-menu";
import { Input } from "@/components/ui/input";
import { InputGroup, InputGroupAddon } from "@/components/ui/input-group";
import { Row } from "@/settings/components/row";
import { LANGUAGES } from "@/settings/presets";
import { useSettings } from "@/settings/settings-context";

/** Free text, since any language the model knows works; the menu only offers common ones. */
export function LanguageField({ name, label, hint }: { name: "targetLang" | "fallbackLang"; label: string; hint?: string }) {
  const { settings, update } = useSettings();
  return (
    <Row label={label} hint={hint} htmlFor={name} width="wide">
      <InputGroup>
        <Input id={name} value={settings[name]} onChange={(e) => update({ [name]: e.target.value })} />
        <InputGroupAddon>
          <DropdownMenu>
            <DropdownMenuTrigger
              aria-label="Choose a language"
              className="focus-ring grid size-6 place-items-center rounded text-muted hover:bg-hover hover:text-ink"
            >
              <ChevronDown className="size-3.5" />
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              {LANGUAGES.map((lang) => (
                <DropdownMenuItem key={lang} onSelect={() => update({ [name]: lang })}>
                  {lang}
                </DropdownMenuItem>
              ))}
            </DropdownMenuContent>
          </DropdownMenu>
        </InputGroupAddon>
      </InputGroup>
    </Row>
  );
}
