use serde::Serialize;

use crate::settings::Settings;

#[derive(Serialize)]
pub struct StylePreset {
    pub id: &'static str,
    pub label: &'static str,
    #[serde(skip)]
    instruction: &'static str,
}

pub const STYLES: &[StylePreset] = &[
    StylePreset { id: "natural", label: "Natural", instruction: "natural and fluent, the way a native speaker would say it." },
    StylePreset { id: "formal", label: "Formal", instruction: "formal and polite, suitable for business or official writing." },
    StylePreset { id: "casual", label: "Casual", instruction: "casual and conversational, like talking to a friend." },
    StylePreset { id: "literal", label: "Literal", instruction: "literal and faithful to the original wording and structure." },
    StylePreset { id: "concise", label: "Concise", instruction: "short and concise, keeping only the essential meaning." },
    StylePreset {
        id: "technical",
        label: "Technical",
        instruction: "precise technical register; keep established English terms where translators normally do.",
    },
    StylePreset {
        id: "learner",
        label: "Learner",
        instruction: "natural. After the translation, add a blank line and up to three short notes on idioms, \
                      tricky words or grammar from the original.",
    },
];

pub fn system_prompt(s: &Settings, style: &str) -> String {
    let preset = STYLES.iter().find(|p| p.id == style).unwrap_or(&STYLES[0]);
    let extra = s.custom_instructions.trim();
    let style_text = if extra.is_empty() {
        preset.instruction.to_string()
    } else {
        format!("{}\nAdditional instructions: {extra}", preset.instruction)
    };
    s.system_prompt
        .replace("{target}", &s.target_lang)
        .replace("{fallback}", &s.fallback_lang)
        .replace("{style}", &style_text)
}
