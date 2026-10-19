import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";

/** Radix reserves the empty string for "nothing selected", so an empty option value is swapped for this. */
const EMPTY = "__empty";

export type Options = [value: string, label: string][];

interface OptionsSelectProps {
  id: string;
  value: string;
  onChange: (value: string) => void;
  options: Options;
}

export function OptionsSelect({ id, value, onChange, options }: OptionsSelectProps) {
  return (
    <Select value={value || EMPTY} onValueChange={(v) => onChange(v === EMPTY ? "" : v)}>
      <SelectTrigger id={id}>
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        {options.map(([v, label]) => (
          <SelectItem key={v} value={v || EMPTY}>
            {label}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}
