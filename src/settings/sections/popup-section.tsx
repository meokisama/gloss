import { NumberField, SelectField, SwitchField } from "@/settings/components/fields";
import { Group } from "@/settings/components/group";
import { Page } from "@/settings/components/page";

export function PopupSection() {
  return (
    <Page title="Popup" description="Where the translation appears and how it behaves.">
      <Group title="Placement">
        <SelectField
          name="popupPosition"
          label="Position"
          options={[
            ["cursor", "Below the cursor"],
            ["aboveCursor", "Above the cursor"],
            ["center", "Center of screen"],
            ["topRight", "Top right corner"],
            ["remember", "Where I last left it"],
          ]}
        />
        <NumberField name="popupOffset" label="Distance from cursor" min={0} max={200} unit="px" />
      </Group>
      <Group title="Size">
        <NumberField name="popupWidth" label="Width" min={240} max={1200} unit="px" />
        <NumberField name="popupMaxHeight" label="Max height" min={120} max={1200} unit="px" />
        <NumberField name="fontSize" label="Font size" min={10} max={32} unit="px" />
      </Group>
      <Group title="Behavior">
        <SwitchField name="closeOnBlur" label="Close when clicking elsewhere" />
        <SwitchField name="showSource" label="Show the original text" />
        <SwitchField name="autoCopy" label="Copy the translation automatically" />
      </Group>
    </Page>
  );
}
