//! OpenAI Chat Completions, also spoken by OpenRouter, Groq, DeepSeek, Ollama and LM Studio.

use reqwest::{Client, RequestBuilder};
use serde_json::{Value, json};

use crate::settings::Settings;

pub fn request(client: &Client, s: &Settings, system: &str, text: &str) -> RequestBuilder {
    auth(client.post(format!("{}/chat/completions", super::base_url(s))), s).json(&json!({
        "model": s.model,
        "messages": [{ "role": "system", "content": system }, { "role": "user", "content": text }],
        "stream": true,
    }))
}

pub fn delta(event: &Value) -> Result<String, String> {
    Ok(event["choices"][0]["delta"]["content"].as_str().unwrap_or_default().to_string())
}

pub fn models_request(client: &Client, s: &Settings) -> RequestBuilder {
    auth(client.get(format!("{}/models", super::base_url(s))), s)
}

/// The list comes back in no useful order and includes non-chat models (embeddings, TTS); there is no field to filter on.
pub fn parse_models(body: &Value) -> Vec<String> {
    let mut ids = super::strings(&body["data"], "id");
    ids.sort();
    ids
}

fn auth(req: RequestBuilder, s: &Settings) -> RequestBuilder {
    if s.api_key().is_empty() { req } else { req.bearer_auth(s.api_key()) }
}
