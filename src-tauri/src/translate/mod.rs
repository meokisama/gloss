mod anthropic;
mod gemini;
mod openai;
mod prompt;

use std::time::Duration;

use serde_json::Value;

use crate::settings::{ProviderKind, Settings};

pub use prompt::{STYLES, StylePreset};

pub async fn translate(s: &Settings, text: &str, style: &str) -> Result<String, String> {
    if text.chars().count() > s.max_chars {
        return Err(format!("Selection is longer than {} characters (see Settings).", s.max_chars));
    }
    if s.api_key.is_empty() && s.provider != ProviderKind::Openai {
        return Err("No API key. Open Settings from the tray icon.".into());
    }
    let system = prompt::system_prompt(s, style);
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(s.timeout_secs.max(5)))
        .build()
        .map_err(|e| e.to_string())?;

    let request = match s.provider {
        ProviderKind::Openai => openai::request(&client, s, &system, text),
        ProviderKind::Anthropic => anthropic::request(&client, s, &system, text),
        ProviderKind::Gemini => gemini::request(&client, s, &system, text),
    };
    let response = request.send().await.map_err(|e| format!("Request failed: {e}"))?;
    let status = response.status();
    let body: Value = response.json().await.unwrap_or(Value::Null);
    if !status.is_success() {
        let message = body["error"]["message"].as_str().map(str::to_string).unwrap_or_else(|| body.to_string());
        return Err(format!("HTTP {}: {message}", status.as_u16()));
    }

    let output = match s.provider {
        ProviderKind::Openai => openai::parse(&body),
        ProviderKind::Anthropic => anthropic::parse(&body),
        ProviderKind::Gemini => gemini::parse(&body),
    }?;
    let output = output.trim();
    if output.is_empty() { Err(format!("Empty response: {body}")) } else { Ok(output.to_string()) }
}

fn base_url(s: &Settings) -> &str {
    s.base_url.trim_end_matches('/')
}
