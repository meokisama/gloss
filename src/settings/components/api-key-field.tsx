import { Input } from "@/components/ui/input";
import { Row } from "@/settings/components/row";
import { useSettings } from "@/settings/settings-context";

/** Reads and writes the key of the current base URL, so each provider keeps its own. */
export function ApiKeyField() {
  const { settings, update } = useSettings();

  const setKey = (key: string) => {
    const apiKeys = { ...settings.apiKeys, [settings.baseUrl]: key };
    if (!key) delete apiKeys[settings.baseUrl];
    update({ apiKeys });
  };

  return (
    <Row label="API key" hint="Stored in plain text. Leave empty for local servers." htmlFor="apiKey" width="wide">
      <Input
        id="apiKey"
        type="password"
        className="font-mono"
        value={settings.apiKeys[settings.baseUrl] ?? ""}
        onChange={(e) => setKey(e.target.value)}
      />
    </Row>
  );
}
