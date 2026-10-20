import { ApiKeyField } from "@/settings/components/api-key-field";
import { NumberField, SelectField, TextField } from "@/settings/components/fields";
import { Group } from "@/settings/components/group";
import { ModelField } from "@/settings/components/model-field";
import { Page } from "@/settings/components/page";
import { ProviderField } from "@/settings/components/provider-field";
import { useSettings } from "@/settings/settings-context";

export function ProviderSection() {
  const { settings } = useSettings();

  return (
    <Page title="Provider" description="The AI service that does the translating.">
      <Group title="Connection">
        <ProviderField />
        <TextField name="baseUrl" label="Base URL" mono />
        <ModelField />
        <ApiKeyField />
      </Group>
      <Group title="Request">
        {settings.apiFormat === "anthropic" && (
          <SelectField
            name="effort"
            label="Effort"
            options={[
              ["", "Model default"],
              ["low", "Low (fastest)"],
              ["medium", "Medium"],
              ["high", "High"],
            ]}
          />
        )}
        <NumberField name="timeoutSecs" label="Timeout" min={5} max={600} unit="s" />
      </Group>
    </Page>
  );
}
