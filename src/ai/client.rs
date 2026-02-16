use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub enum Model {
    Sonnet,
    Haiku,
    Opus,
}

impl Model {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sonnet => "claude-sonnet-4-5-20250929",
            Self::Haiku => "claude-haiku-4-5-20251001",
            Self::Opus => "claude-opus-4-6",
        }
    }
}

// --- Request types ---

#[derive(Debug, Serialize)]
pub struct ClaudeRequest {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tool>,
    pub max_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: Vec<ContentBlock>,
}

impl Message {
    pub fn user(text: &str) -> Self {
        Self {
            role: "user".to_string(),
            content: vec![ContentBlock::Text { text: text.to_string() }],
        }
    }

    pub fn assistant_tool_use(id: &str, name: &str, input: Value) -> Self {
        Self {
            role: "assistant".to_string(),
            content: vec![ContentBlock::ToolUse {
                id: id.to_string(),
                name: name.to_string(),
                input,
            }],
        }
    }

    pub fn user_tool_result(tool_use_id: &str, content: &str) -> Self {
        Self {
            role: "user".to_string(),
            content: vec![ContentBlock::ToolResult {
                tool_use_id: tool_use_id.to_string(),
                content: content.to_string(),
            }],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text { text: String },
    ToolUse { id: String, name: String, input: Value },
    ToolResult { tool_use_id: String, content: String },
}

#[derive(Debug, Serialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

// --- Response types ---

#[derive(Debug, Deserialize)]
pub struct ClaudeResponse {
    pub id: String,
    pub content: Vec<ContentBlock>,
    pub model: String,
    pub stop_reason: String,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

impl ClaudeResponse {
    /// Extract the first tool_use block, if any.
    pub fn tool_use(&self) -> Option<(&str, &str, &Value)> {
        self.content.iter().find_map(|block| match block {
            ContentBlock::ToolUse { id, name, input } => Some((id.as_str(), name.as_str(), input)),
            _ => None,
        })
    }

    /// Extract the first text block, if any.
    pub fn text(&self) -> Option<&str> {
        self.content.iter().find_map(|block| match block {
            ContentBlock::Text { text } => Some(text.as_str()),
            _ => None,
        })
    }
}

// --- Error types ---

#[derive(Debug, thiserror::Error)]
pub enum ClaudeError {
    #[error("Rate limited by Anthropic API")]
    RateLimit,
    #[error("Request timed out")]
    Timeout,
    #[error("Invalid response from API: {0}")]
    InvalidResponse(String),
    #[error("API error ({status}): {message}")]
    ApiError { status: u16, message: String },
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
}

// --- Client ---

pub struct ClaudeClient {
    http: reqwest::Client,
    api_key: String,
    base_url: String,
}

impl ClaudeClient {
    pub fn new(api_key: String) -> Self {
        Self::new_with_base_url(api_key, "https://api.anthropic.com".to_string())
    }

    pub fn new_with_base_url(api_key: String, base_url: String) -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .expect("Failed to build HTTP client");
        Self { http, api_key, base_url }
    }

    pub async fn send(
        &self,
        model: Model,
        system: Option<&str>,
        messages: Vec<Message>,
        tools: Vec<Tool>,
        max_tokens: u32,
    ) -> Result<ClaudeResponse, ClaudeError> {
        let request = ClaudeRequest {
            model: model.as_str().to_string(),
            system: system.map(String::from),
            messages,
            tools,
            max_tokens,
        };

        let mut retries = 0;
        let max_retries = 3;

        loop {
            let result = self
                .http
                .post(format!("{}/v1/messages", self.base_url))
                .header("x-api-key", &self.api_key)
                .header("anthropic-version", "2023-06-01")
                .header("content-type", "application/json")
                .json(&request)
                .send()
                .await;

            match result {
                Ok(resp) => {
                    let status = resp.status().as_u16();
                    if status == 429 && retries < max_retries {
                        retries += 1;
                        let delay = Duration::from_secs(1 << retries);
                        tokio::time::sleep(delay).await;
                        continue;
                    }
                    if status == 429 {
                        return Err(ClaudeError::RateLimit);
                    }
                    if status >= 400 {
                        let body = resp.text().await.unwrap_or_default();
                        return Err(ClaudeError::ApiError {
                            status,
                            message: body,
                        });
                    }

                    let response: ClaudeResponse = resp
                        .json()
                        .await
                        .map_err(|e| ClaudeError::InvalidResponse(e.to_string()))?;
                    return Ok(response);
                }
                Err(e) if e.is_timeout() => return Err(ClaudeError::Timeout),
                Err(e) => return Err(ClaudeError::Http(e)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_to_string() {
        assert_eq!(Model::Sonnet.as_str(), "claude-sonnet-4-5-20250929");
        assert_eq!(Model::Haiku.as_str(), "claude-haiku-4-5-20251001");
    }

    #[test]
    fn parse_tool_use_from_response() {
        let json = serde_json::json!({
            "id": "msg_123",
            "type": "message",
            "role": "assistant",
            "content": [{
                "type": "tool_use",
                "id": "toolu_123",
                "name": "generate_macrocycle_skeleton",
                "input": {
                    "target_ctl": 65.0,
                    "mesocycles": []
                }
            }],
            "model": "claude-sonnet-4-5-20250929",
            "stop_reason": "tool_use",
            "usage": { "input_tokens": 100, "output_tokens": 50 }
        });

        let response: ClaudeResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.stop_reason, "tool_use");
        assert_eq!(response.content.len(), 1);
        match &response.content[0] {
            ContentBlock::ToolUse { name, input, .. } => {
                assert_eq!(name, "generate_macrocycle_skeleton");
                assert_eq!(input["target_ctl"], 65.0);
            }
            _ => panic!("Expected ToolUse"),
        }
    }

    #[test]
    fn parse_text_response() {
        let json = serde_json::json!({
            "id": "msg_123",
            "type": "message",
            "role": "assistant",
            "content": [{ "type": "text", "text": "Hello!" }],
            "model": "claude-sonnet-4-5-20250929",
            "stop_reason": "end_turn",
            "usage": { "input_tokens": 10, "output_tokens": 5 }
        });

        let response: ClaudeResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.stop_reason, "end_turn");
        match &response.content[0] {
            ContentBlock::Text { text } => assert_eq!(text, "Hello!"),
            _ => panic!("Expected Text"),
        }
    }

    #[test]
    fn claude_error_display() {
        let err = ClaudeError::RateLimit;
        assert_eq!(format!("{}", err), "Rate limited by Anthropic API");
    }
}
