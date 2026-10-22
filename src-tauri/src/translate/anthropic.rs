use reqwest::{Client, RequestBuilder};
use serde_json::{Value, json};

use crate::settings::Settings;

pub fn request(client: &Client, s: &Settings, system: &str, text: &str) -> RequestBuilder {
    let mut body = json!({
        "model": s.model,
        "max_tokens": 16000,
        "stream": true,
        "system": system,
        "messages": [{ "role": "user", "content": text }],
    });
    // Current models think by default; low effort keeps a short translation fast.
    if !s.effort.is_empty() {
        body["output_config"] = json!({ "effort": s.effort });
    }
    auth(client.post(format!("{}/v1/messages", super::base_url(s))), s).json(&body)
}

/// Thinking arrives as `thinking_delta` and is left out.
pub fn delta(event: &Value) -> Result<String, String> {
    let delta = &event["delta"];
    if delta["stop_reason"] == "refusal" {
        return Err("The model declined this request.".into());
    }
    Ok(if delta["type"] == "text_delta" { delta["text"].as_str().unwrap_or_default().to_string() } else { String::new() })
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
