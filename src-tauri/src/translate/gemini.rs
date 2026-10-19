use reqwest::{Client, RequestBuilder};
use serde_json::{Value, json};

use crate::settings::Settings;

pub fn request(client: &Client, s: &Settings, system: &str, text: &str) -> RequestBuilder {
    client
        .post(format!("{}/v1beta/models/{}:generateContent", super::base_url(s), s.model))
        .header("x-goog-api-key", &s.api_key)
        .json(&json!({
            "systemInstruction": { "parts": [{ "text": system }] },
            "contents": [{ "role": "user", "parts": [{ "text": text }] }],
        }))
}

pub fn parse(body: &Value) -> Result<String, String> {
    let parts = body["candidates"][0]["content"]["parts"].as_array().map(Vec::as_slice).unwrap_or_default();
    Ok(parts.iter().filter_map(|p| p["text"].as_str()).collect())
}
