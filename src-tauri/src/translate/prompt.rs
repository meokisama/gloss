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
    StylePreset {
        id: "original",
        label: "Match original",
        instruction: "keep the tone and formality of the original; do not make it more polite or more casual.",
    },
    StylePreset { id: "formal", label: "Formal", instruction: "formal and polite, suitable for business or official writing." },
    StylePreset { id: "casual", label: "Casual", instruction: "casual and conversational, like talking to a friend." },
    StylePreset {
        id: "chatting",
        label: "Chatting",
        instruction: "fast chat between friends: all lowercase, no apostrophes (its, dont, im), little or no punctuation, as short \
                      as possible. Use the abbreviations native speakers of the target language type in chat wherever one fits \
                      (in English: afaik, tbh, idk, imo, btw, ngl, u, ur, rn, pls, thx).",
    },
    StylePreset {
        id: "learner",
        label: "Learner",
        instruction: "matching the tone of the original. After the translation, add a blank line and up to three short notes on idioms, \
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
        .replace("{style}", &style_text)
}
