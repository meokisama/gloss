use reqwest::{Client, RequestBuilder};
use serde_json::{Value, json};

use crate::settings::Settings;

pub fn request(client: &Client, s: &Settings, system: &str, text: &str) -> RequestBuilder {
    let mut body = json!({
        "model": s.model,
        "max_tokens": 16000,
        "system": system,
        "messages": [{ "role": "user", "content": text }],
    });
    // Current models think by default; low effort keeps a short translation fast.
    if !s.effort.is_empty() {
        body["output_config"] = json!({ "effort": s.effort });
    }
    client
        .post(format!("{}/v1/messages", super::base_url(s)))
        .header("x-api-key", &s.api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&body)
}

pub fn parse(body: &Value) -> Result<String, String> {
    if body["stop_reason"] == "refusal" {
        return Err("The model declined this request.".into());
    }
    let blocks = body["content"].as_array().map(Vec::as_slice).unwrap_or_default();
    Ok(blocks.iter().filter(|b| b["type"] == "text").filter_map(|b| b["text"].as_str()).collect())
}
