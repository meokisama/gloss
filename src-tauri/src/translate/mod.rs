mod anthropic;
mod gemini;
mod openai;
mod prompt;

use std::time::Duration;

use eventsource_stream::Eventsource;
use futures_util::StreamExt;
use reqwest::{Client, RequestBuilder, Response};
use serde_json::Value;

use crate::settings::{ApiFormat, Settings};

pub use prompt::{STYLES, StylePreset, explain_prompt, system_prompt};

/// Hands each piece of the reply to `on_delta` as it arrives, then returns the whole reply.
pub async fn complete(s: &Settings, system: &str, text: &str, on_delta: impl Fn(&str)) -> Result<String, String> {
    if text.chars().count() > s.max_chars {
        return Err(format!("Selection is longer than {} characters (see Settings).", s.max_chars));
    }
    let client = client(s)?;
    let request = match s.api_format {
        ApiFormat::Openai => openai::request(&client, s, system, text),
        ApiFormat::Anthropic => anthropic::request(&client, s, system, text),
        ApiFormat::Gemini => gemini::request(&client, s, system, text),
    };
    let mut events = send(request).await?.bytes_stream().eventsource();

    let mut output = String::new();
    while let Some(event) = events.next().await {
        let event = event.map_err(|e| format!("Stream failed: {e}"))?;
        // OpenAI's end marker; the other formats just close the stream.
        if event.data == "[DONE]" {
            break;
        }
        let event: Value = serde_json::from_str(&event.data).map_err(|e| format!("Bad stream event: {e}"))?;
        if let Some(message) = event["error"]["message"].as_str() {
            return Err(message.to_string());
        }
        let delta = match s.api_format {
            ApiFormat::Openai => openai::delta(&event),
            ApiFormat::Anthropic => anthropic::delta(&event),
            ApiFormat::Gemini => gemini::delta(&event),
        }?;
        if !delta.is_empty() {
            on_delta(&delta);
            output.push_str(&delta);
        }
    }
    let output = output.trim();
    if output.is_empty() { Err("Empty response.".into()) } else { Ok(output.to_string()) }
}

pub async fn list_models(s: &Settings) -> Result<Vec<String>, String> {
    let client = client(s)?;
    let request = match s.api_format {
        ApiFormat::Openai => openai::models_request(&client, s),
        ApiFormat::Anthropic => anthropic::models_request(&client, s),
        ApiFormat::Gemini => gemini::models_request(&client, s),
    };
    let body: Value = send(request).await?.json().await.unwrap_or(Value::Null);
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

async fn send(request: RequestBuilder) -> Result<Response, String> {
    let response = request.send().await.map_err(|e| format!("Request failed: {e}"))?;
    let status = response.status();
    if status.is_success() {
        return Ok(response);
    }
    let body: Value = response.json().await.unwrap_or(Value::Null);
    let message = body["error"]["message"].as_str().map(str::to_string).unwrap_or_else(|| body.to_string());
    Err(format!("HTTP {}: {message}", status.as_u16()))
}

fn base_url(s: &Settings) -> &str {
    s.base_url.trim_end_matches('/')
}

/// `key` of every object in the `list` array.
fn strings(list: &Value, key: &str) -> Vec<String> {
    let items = list.as_array().map(Vec::as_slice).unwrap_or_default();
    items.iter().filter_map(|item| item[key].as_str()).map(str::to_string).collect()
}
