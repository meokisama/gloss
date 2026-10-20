import type { ReactNode } from "react";

import { Input } from "@/components/ui/input";
import { InputGroup, InputGroupAddon } from "@/components/ui/input-group";
import { Switch } from "@/components/ui/switch";
import { Textarea } from "@/components/ui/textarea";
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";
import { cn } from "@/lib/utils";
import { OptionsSelect, type Options } from "@/settings/components/options-select";
import { Row } from "@/settings/components/row";
import { useSettings } from "@/settings/settings-context";
import type { Settings } from "@/shared/settings";

type KeysOf<T> = { [K in keyof Settings]: Settings[K] extends T ? K : never }[keyof Settings];

interface FieldProps<T> {
  name: KeysOf<T>;
  label: string;
  hint?: ReactNode;
}

export function TextField({ name, label, hint, mono, ...input }: FieldProps<string> & { type?: string; placeholder?: string; mono?: boolean }) {
  const { settings, update } = useSettings();
  return (
    <Row label={label} hint={hint} htmlFor={name} width="wide">
      <Input
        id={name}
        className={cn(mono && "font-mono")}
        {...input}
        value={settings[name] as string}
        onChange={(e) => update({ [name]: e.target.value })}
      />
    </Row>
  );
}

type TextAreaProps = FieldProps<string> & { rows: number; placeholder?: string; mono?: boolean; action?: ReactNode };

export function TextAreaField({ name, label, hint, action, mono, ...input }: TextAreaProps) {
  const { settings, update } = useSettings();
  return (
    <Row label={label} hint={hint} htmlFor={name} action={action} stacked>
      <Textarea
        id={name}
        className={cn(mono && "font-mono")}
        {...input}
        value={settings[name] as string}
        onChange={(e) => update({ [name]: e.target.value })}
      />
    </Row>
  );
}

export function NumberField({ name, label, hint, min, max, unit }: FieldProps<number> & { min: number; max: number; unit: string }) {
  const { settings, update } = useSettings();
  return (
    <Row label={label} hint={hint} htmlFor={name} width="narrow">
      <InputGroup>
        <Input id={name} type="number" min={min} max={max} value={settings[name]} onChange={(e) => update({ [name]: e.target.valueAsNumber || 0 })} />
        <InputGroupAddon className="pr-1.5">{unit}</InputGroupAddon>
      </InputGroup>
    </Row>
  );
}

export function SelectField({ name, label, hint, options, width = "wide" }: FieldProps<string> & { options: Options; width?: "medium" | "wide" }) {
  const { settings, update } = useSettings();
  return (
    <Row label={label} hint={hint} htmlFor={name} width={width}>
      <OptionsSelect id={name} value={settings[name] as string} onChange={(value) => update({ [name]: value })} options={options} />
    </Row>
  );
}

export function SegmentedField({ name, label, hint, options }: FieldProps<string> & { options: Options }) {
  const { settings, update } = useSettings();
  return (
    <Row label={label} hint={hint} htmlFor={name}>
      <ToggleGroup
        id={name}
        type="single"
        value={settings[name] as string}
        onValueChange={(value) => value && update({ [name]: value })}
        className="h-7 rounded-md bg-hover p-0.5"
      >
        {options.map(([value, text]) => (
          <ToggleGroupItem key={value} value={value} className="h-full rounded-[5px] data-[state=on]:bg-field data-[state=on]:shadow-sm">
            {text}
          </ToggleGroupItem>
        ))}
      </ToggleGroup>
    </Row>
  );
}

export function SwitchField({ name, label, hint }: FieldProps<boolean>) {
  const { settings, update } = useSettings();
  return (
    <Row label={label} hint={hint} htmlFor={name}>
      <Switch id={name} checked={settings[name]} onCheckedChange={(checked) => update({ [name]: checked })} />
    </Row>
  );
}
