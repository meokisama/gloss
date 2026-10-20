import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Row } from "@/settings/components/row";
import { useSettings } from "@/settings/settings-context";
import { useModels } from "@/settings/use-models";

export function ModelField() {
  const { settings, update } = useSettings();
  const { list, load } = useModels(settings);
  const models = list.status === "done" ? list.models : [];
  // Radix shows a value only if its item is rendered, so the saved model stays listed before the fetch or if the provider omits it.
  const options = !settings.model || models.includes(settings.model) ? models : [settings.model, ...models];
  const notice =
    list.status === "error" ? list.text : list.status !== "done" ? "Loading models…" : models.length === 0 ? "The provider listed no models." : null;

  return (
    <Row label="Model" htmlFor="model" width="wide">
      <Select value={settings.model} onValueChange={(model) => update({ model })} onOpenChange={(open) => open && load()}>
        <SelectTrigger id="model" className="font-mono">
          <SelectValue placeholder="Choose a model" />
        </SelectTrigger>
        <SelectContent>
          {options.map((model) => (
            <SelectItem key={model} value={model} className="font-mono">
              {model}
            </SelectItem>
          ))}
          {notice && <p className="max-w-56 px-2 py-1.5 text-xs text-muted">{notice}</p>}
        </SelectContent>
      </Select>
    </Row>
  );
}
