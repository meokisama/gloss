import { useState } from "react";

import { SelectField } from "@/settings/components/fields";
import { OptionsSelect } from "@/settings/components/options-select";
import { Row } from "@/settings/components/row";
import { PROVIDER_PRESETS } from "@/settings/presets";
import { useSettings } from "@/settings/settings-context";

const CUSTOM = "custom";

/** Settings store no preset id, so the selection is recovered from the base URL; anything unmatched is Custom. */
export function ProviderField() {
  const { settings, update } = useSettings();
  const [custom, setCustom] = useState(false);
  const matched = PROVIDER_PRESETS.find((p) => p.provider === settings.provider && p.baseUrl === settings.baseUrl);
  const selected = custom || !matched ? CUSTOM : matched.name;

  const choose = (name: string) => {
    const preset = PROVIDER_PRESETS.find((p) => p.name === name);
    setCustom(!preset);
    if (preset) update({ provider: preset.provider, baseUrl: preset.baseUrl, model: preset.model });
  };

  return (
    <>
      <Row label="Provider" htmlFor="preset" width="wide">
        <OptionsSelect
          id="preset"
          value={selected}
          onChange={choose}
          options={[...PROVIDER_PRESETS.map((p): [string, string] => [p.name, p.name]), [CUSTOM, "Custom"]]}
        />
      </Row>
      {selected === CUSTOM && (
        <SelectField
          name="provider"
          label="API format"
          options={[
            ["openai", "OpenAI-compatible"],
            ["anthropic", "Anthropic"],
            ["gemini", "Google Gemini"],
          ]}
        />
      )}
    </>
  );
}
