import { SegmentedField, SwitchField } from "@/settings/components/fields";
import { Group } from "@/settings/components/group";
import { HotkeyInput } from "@/settings/components/hotkey-input";
import { Page } from "@/settings/components/page";

export function GeneralSection() {
  return (
    <Page title="General" description="How you call Gloss and how it looks.">
      <Group title="Shortcut">
        <HotkeyInput />
      </Group>
      <Group title="App">
        <SwitchField name="launchAtLogin" label="Launch at login" />
        <SegmentedField
          name="theme"
          label="Theme"
          options={[
            ["system", "System"],
            ["light", "Light"],
            ["dark", "Dark"],
          ]}
        />
      </Group>
    </Page>
  );
}
