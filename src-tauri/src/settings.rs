use std::collections::BTreeMap;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreExt;

const STORE_FILE: &str = "settings.json";
const STORE_KEY: &str = "settings";

pub const DEFAULT_SYSTEM_PROMPT: &str = "You are a translation engine. Translate the user's text into {target}. \
If the text is already in {target}, translate it into {fallback} instead.\n\
Style: {style}\n\
Output only the translation: no quotes, no preamble. Keep line breaks, code, URLs and proper names unchanged.";

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ApiFormat {
    #[default]
    Openai,
    Anthropic,
    Gemini,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PopupPosition {
    #[default]
    Cursor,
    AboveCursor,
    Center,
    TopRight,
    Remember,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Settings {
    pub hotkey: String,
    pub launch_at_login: bool,

    /// Not the provider: OpenAI, OpenRouter, Groq and the rest all speak `Openai`. The base URL tells providers apart.
    #[serde(alias = "provider")]
    pub api_format: ApiFormat,
    pub base_url: String,
    /// Keyed by base URL, so each provider keeps its own key.
    pub api_keys: BTreeMap<String, String>,
    pub model: String,
    /// Anthropic `output_config.effort`; empty means the parameter is not sent.
    pub effort: String,
    pub timeout_secs: u64,

    pub target_lang: String,
    pub fallback_lang: String,
    pub style: String,
    pub custom_instructions: String,
    pub system_prompt: String,
    pub max_chars: usize,

    pub popup_position: PopupPosition,
    pub popup_offset: u32,
    pub popup_width: u32,
    pub popup_max_height: u32,
    pub font_size: u32,
    pub theme: String,
    pub close_on_blur: bool,
    pub auto_copy: bool,
    pub show_source: bool,
    pub last_popup_pos: Option<(i32, i32)>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            hotkey: "Alt+Q".into(),
            launch_at_login: false,
            api_format: ApiFormat::Openai,
            base_url: "https://api.openai.com/v1".into(),
            api_keys: BTreeMap::new(),
            model: "gpt-5-mini".into(),
            effort: "low".into(),
            timeout_secs: 60,
            target_lang: "Vietnamese".into(),
            fallback_lang: "English".into(),
            style: "natural".into(),
            custom_instructions: String::new(),
            system_prompt: DEFAULT_SYSTEM_PROMPT.into(),
            max_chars: 5000,
            popup_position: PopupPosition::Cursor,
            popup_offset: 14,
            popup_width: 420,
            popup_max_height: 420,
            font_size: 14,
            theme: "system".into(),
            close_on_blur: true,
            auto_copy: false,
            show_source: true,
            last_popup_pos: None,
        }
    }
}

impl Settings {
    pub fn api_key(&self) -> &str {
        self.api_keys.get(&self.base_url).map_or("", String::as_str)
    }
}

pub struct SettingsState(pub Mutex<Settings>);

pub fn current(app: &AppHandle) -> Settings {
    app.state::<SettingsState>().0.lock().unwrap().clone()
}

pub fn load(app: &AppHandle) -> Settings {
    app.store(STORE_FILE)
        .ok()
        .and_then(|store| store.get(STORE_KEY))
        .and_then(|value| serde_json::from_value(value).ok())
        .unwrap_or_default()
}

pub fn persist(app: &AppHandle, settings: &Settings) -> Result<(), String> {
    let store = app.store(STORE_FILE).map_err(|e| e.to_string())?;
    store.set(STORE_KEY, serde_json::to_value(settings).map_err(|e| e.to_string())?);
    store.save().map_err(|e| e.to_string())?;
    *app.state::<SettingsState>().0.lock().unwrap() = settings.clone();
    Ok(())
}
