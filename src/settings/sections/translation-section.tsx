import { Button } from "@/components/ui/button";
import { NumberField, SelectField, TextAreaField } from "@/settings/components/fields";
import { Group } from "@/settings/components/group";
import { LanguageField } from "@/settings/components/language-field";
import { Page } from "@/settings/components/page";
import { useSettings } from "@/settings/settings-context";
import { api } from "@/shared/api";

export function TranslationSection() {
  const { styles, update } = useSettings();

  const resetPrompt = async () => update({ systemPrompt: (await api.defaultSettings()).systemPrompt });

  return (
    <Page title="Translation" description="Languages, tone and the instructions sent to the model.">
      <Group title="Languages">
        <LanguageField name="targetLang" label="Translate into" />
        <LanguageField name="fallbackLang" label="Fallback" hint="Used when the text is already in the target language." />
      </Group>
      <Group title="Output">
        <SelectField name="style" label="Default style" options={styles.map((s) => [s.id, s.label])} />
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
          hint="Placeholders: {target}, {fallback}, {style} (the style plus extra instructions)."
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
