import { Button } from "@/components/ui/button";
import { NumberField, SegmentedField, SelectField, TextAreaField } from "@/settings/components/fields";
import { Group } from "@/settings/components/group";
import { LanguageField } from "@/settings/components/language-field";
import { Page } from "@/settings/components/page";
import { useSettings } from "@/settings/settings-context";
import { api } from "@/shared/api";

export function TranslationSection() {
  const { styles, update } = useSettings();

  const resetPrompt = async () => update({ systemPrompt: (await api.defaultSettings()).systemPrompt });
  const styleOptions = styles.map((s): [string, string] => [s.id, s.label]);

  return (
    <Page title="Translation" description="Languages, tone and the instructions sent to the model.">
      <Group title="Selected text">
        <LanguageField name="targetLang" label="Translate into" hint="For text you read, with the translate shortcut." />
        <SelectField name="style" label="Style" width="medium" options={styleOptions} />
      </Group>
      <Group title="Your writing">
        <LanguageField name="writeLang" label="Translate into" hint="For text you typed, with the writing shortcut." />
        <SelectField name="writeStyle" label="Style" width="medium" options={styleOptions} />
        <SegmentedField
          name="writeMode"
          label="After translating"
          hint="Preview waits for Enter before replacing what you typed."
          options={[
            ["preview", "Preview"],
            ["replace", "Replace"],
          ]}
        />
      </Group>
      <Group title="Output">
        <NumberField name="maxChars" label="Max selection length" min={100} max={100000} unit="ch" />
        <TextAreaField
          name="customInstructions"
          label="Extra instructions"
          rows={2}
          placeholder="e.g. Keep game terms in English. Use southern Vietnamese wording."
        />
      </Group>
      <Group title="Advanced">
        <TextAreaField
          name="systemPrompt"
          label="System prompt"
          hint="Placeholders: {target}, {style} (the style plus extra instructions)."
          rows={8}
          mono
          action={
            <Button variant="ghost" onClick={resetPrompt}>
              Reset
            </Button>
          }
        />
      </Group>
    </Page>
  );
}
