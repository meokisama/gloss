mod anthropic;
mod gemini;
mod openai;
mod prompt;

use std::time::Duration;

use reqwest::{Client, RequestBuilder};
use serde_json::Value;

use crate::settings::{ApiFormat, Settings};

pub use prompt::{STYLES, StylePreset, explain_prompt, system_prompt};

pub async fn complete(s: &Settings, system: &str, text: &str) -> Result<String, String> {
    if text.chars().count() > s.max_chars {
        return Err(format!("Selection is longer than {} characters (see Settings).", s.max_chars));
    }
    let client = client(s)?;
    let request = match s.api_format {
        ApiFormat::Openai => openai::request(&client, s, system, text),
        ApiFormat::Anthropic => anthropic::request(&client, s, system, text),
        ApiFormat::Gemini => gemini::request(&client, s, system, text),
    };
    let body = send(request).await?;

    let output = match s.api_format {
        ApiFormat::Openai => openai::parse(&body),
        ApiFormat::Anthropic => anthropic::parse(&body),
        ApiFormat::Gemini => gemini::parse(&body),
    }?;
    let output = output.trim();
    if output.is_empty() { Err(format!("Empty response: {body}")) } else { Ok(output.to_string()) }
}

pub async fn list_models(s: &Settings) -> Result<Vec<String>, String> {
    let client = client(s)?;
    let request = match s.api_format {
        ApiFormat::Openai => openai::models_request(&client, s),
        ApiFormat::Anthropic => anthropic::models_request(&client, s),
        ApiFormat::Gemini => gemini::models_request(&client, s),
    };
    let body = send(request).await?;
    Ok(match s.api_format {
        ApiFormat::Openai => openai::parse_models(&body),
        ApiFormat::Anthropic => anthropic::parse_models(&body),
        ApiFormat::Gemini => gemini::parse_models(&body),
    })
}

fn client(s: &Settings) -> Result<Client, String> {
    if s.api_key().is_empty() && s.api_format != ApiFormat::Openai {
        return Err("No API key. Open Settings from the tray icon.".into());
    }
    Client::builder().timeout(Duration::from_secs(s.timeout_secs.max(5))).build().map_err(|e| e.to_string())
}

async fn send(request: RequestBuilder) -> Result<Value, String> {
    let response = request.send().await.map_err(|e| format!("Request failed: {e}"))?;
    let status = response.status();
    let body: Value = response.json().await.unwrap_or(Value::Null);
    if !status.is_success() {
        let message = body["error"]["message"].as_str().map(str::to_string).unwrap_or_else(|| body.to_string());
        return Err(format!("HTTP {}: {message}", status.as_u16()));
    }
    Ok(body)
}

fn base_url(s: &Settings) -> &str {
    s.base_url.trim_end_matches('/')
}

/// `key` of every object in the `list` array.
fn strings(list: &Value, key: &str) -> Vec<String> {
    let items = list.as_array().map(Vec::as_slice).unwrap_or_default();
    items.iter().filter_map(|item| item[key].as_str()).map(str::to_string).collect()
}
