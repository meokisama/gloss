import { SegmentedField, SwitchField } from "@/settings/components/fields";
import { Group } from "@/settings/components/group";
import { HotkeyInput } from "@/settings/components/hotkey-input";
import { Page } from "@/settings/components/page";

export function GeneralSection() {
  return (
    <Page title="General" description="How you call Gloss and how it looks.">
      <Group title="Shortcuts">
        <HotkeyInput name="hotkey" label="Translate selection" hint="Into your language. Click, then press a combination with a modifier." />
        <HotkeyInput name="writeHotkey" label="Translate your writing" hint="Select what you typed; the translation can replace it." />
        <HotkeyInput
          name="explainHotkey"
          label="Explain selection"
          hint="What a term or concept is and how it works, or what a word means and how to use it."
        />
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
