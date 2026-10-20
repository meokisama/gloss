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
    auth(client.post(format!("{}/v1/messages", super::base_url(s))), s).json(&body)
}

pub fn parse(body: &Value) -> Result<String, String> {
    if body["stop_reason"] == "refusal" {
        return Err("The model declined this request.".into());
    }
    let blocks = body["content"].as_array().map(Vec::as_slice).unwrap_or_default();
    Ok(blocks.iter().filter(|b| b["type"] == "text").filter_map(|b| b["text"].as_str()).collect())
}

/// Newest first; 1000 is the page maximum, far above the catalogue size, so one page is all of it.
pub fn models_request(client: &Client, s: &Settings) -> RequestBuilder {
    auth(client.get(format!("{}/v1/models?limit=1000", super::base_url(s))), s)
}

pub fn parse_models(body: &Value) -> Vec<String> {
    super::strings(&body["data"], "id")
}

fn auth(req: RequestBuilder, s: &Settings) -> RequestBuilder {
    req.header("x-api-key", s.api_key()).header("anthropic-version", "2023-06-01")
}
