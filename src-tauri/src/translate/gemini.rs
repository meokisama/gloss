use reqwest::{Client, RequestBuilder};
use serde_json::{Value, json};

use crate::settings::Settings;

pub fn request(client: &Client, s: &Settings, system: &str, text: &str) -> RequestBuilder {
    client
        .post(format!("{}/v1beta/models/{}:streamGenerateContent?alt=sse", super::base_url(s), s.model))
        .header("x-goog-api-key", s.api_key())
        .json(&json!({
            "systemInstruction": { "parts": [{ "text": system }] },
            "contents": [{ "role": "user", "parts": [{ "text": text }] }],
        }))
}

/// Each event is a partial `generateContent` response.
pub fn delta(event: &Value) -> Result<String, String> {
    let parts = event["candidates"][0]["content"]["parts"].as_array().map(Vec::as_slice).unwrap_or_default();
    Ok(parts.iter().filter_map(|p| p["text"].as_str()).collect())
}

pub fn models_request(client: &Client, s: &Settings) -> RequestBuilder {
    client
        .get(format!("{}/v1beta/models?pageSize=1000", super::base_url(s)))
        .header("x-goog-api-key", s.api_key())
}

/// Names come as `models/<id>`; embedding and other models without `generateContent` cannot translate.
pub fn parse_models(body: &Value) -> Vec<String> {
    let models = body["models"].as_array().map(Vec::as_slice).unwrap_or_default();
    models
        .iter()
        .filter(|m| m["supportedGenerationMethods"].as_array().is_some_and(|methods| methods.iter().any(|x| x == "generateContent")))
        .filter_map(|m| m["name"].as_str())
        .map(|name| name.trim_start_matches("models/").to_string())
        .collect()
}
