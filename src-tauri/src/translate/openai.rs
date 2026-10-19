//! OpenAI Chat Completions, also spoken by OpenRouter, Groq, DeepSeek, Ollama and LM Studio.

use reqwest::{Client, RequestBuilder};
use serde_json::{Value, json};

use crate::settings::Settings;

pub fn request(client: &Client, s: &Settings, system: &str, text: &str) -> RequestBuilder {
    let req = client.post(format!("{}/chat/completions", super::base_url(s))).json(&json!({
        "model": s.model,
        "messages": [{ "role": "system", "content": system }, { "role": "user", "content": text }],
    }));
    if s.api_key.is_empty() { req } else { req.bearer_auth(&s.api_key) }
}

pub fn parse(body: &Value) -> Result<String, String> {
    Ok(body["choices"][0]["message"]["content"].as_str().unwrap_or_default().to_string())
}
