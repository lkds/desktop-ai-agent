/// Claude Provider 实现
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use super::provider_trait::*;

pub struct ClaudeProvider {
    config: ProviderConfig,
    client: Client,
}

impl ClaudeProvider {
    pub fn new(config: ProviderConfig) -> Result<Self, ProviderError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .map_err(|e| ProviderError::InternalError(e.to_string()))?;
        Ok(Self { config, client })
    }
    
    fn get_api_key(&self) -> Result<&str, ProviderError> {
        self.config.api_key.as_deref()
            .ok_or(ProviderError::ApiKeyMissing)
    }
}

#[async_trait]
impl Provider for ClaudeProvider {
    fn name(&self) -> &str { "claude" }
    
    async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse, ProviderError> {
        let api_key = self.get_api_key()?;
        let url = self.config.base_url.as_deref()
            .unwrap_or("https://api.anthropic.com/v1/messages");
        
        let system_msg = request.messages.iter()
            .find(|m| m.role == Role::System)
            .map(|m| m.content.clone());
        
        let body = ClaudeRequest {
            model: self.config.model.clone(),
            max_tokens: request.max_tokens.unwrap_or(4096),
            messages: request.messages.iter()
                .filter(|m| m.role != Role::System)
                .map(|m| ClaudeMessage {
                    role: if m.role == Role::User { "user".to_string() } else { "assistant".to_string() },
                    content: m.content.clone(),
                }).collect(),
            system: system_msg,
        };
        
        let response = self.client.post(url)
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send().await
            .map_err(|e| ProviderError::ConnectionFailed(e.to_string()))?;
        
        if !response.status().is_success() {
            if response.status() == 429 { return Err(ProviderError::RateLimited); }
            return Err(ProviderError::InvalidResponse(response.status().to_string()));
        }
        
        let claude_response: ClaudeResponse = response.json().await
            .map_err(|e| ProviderError::InvalidResponse(e.to_string()))?;
        
        let content_text = claude_response.content.iter()
            .filter_map(|c| if c.kind == "text" { Some(c.text.clone()) } else { None })
            .collect::<Vec<_>>()
            .join("");
        
        Ok(GenerateResponse {
            message: Message {
                role: Role::Assistant,
                content: content_text,
                tool_calls: None,
            },
            finish_reason: FinishReason::Stop,
            usage: Some(Usage {
                prompt_tokens: claude_response.usage.input_tokens,
                completion_tokens: claude_response.usage.output_tokens,
                total_tokens: claude_response.usage.input_tokens + claude_response.usage.output_tokens,
            }),
        })
    }
    
    async fn generate_stream(&self, _request: GenerateRequest) 
        -> Result<Box<dyn futures::Stream<Item = StreamChunk> + Unpin + Send>, ProviderError> {
        Err(ProviderError::InternalError("Streaming not implemented".to_string()))
    }
    
    async fn health_check(&self) -> Result<bool, ProviderError> { Ok(true) }
    
    async fn list_models(&self) -> Result<Vec<String>, ProviderError> {
        Ok(vec!["claude-3-opus".to_string(), "claude-3-sonnet".to_string(), "claude-3-haiku".to_string()])
    }
}

#[derive(Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<ClaudeMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
}

#[derive(Serialize)]
struct ClaudeMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ClaudeResponse {
    content: Vec<ClaudeContent>,
    usage: ClaudeUsage,
}

#[derive(Deserialize)]
struct ClaudeContent {
    #[serde(rename = "type")]
    kind: String,
    text: String,
}

#[derive(Deserialize)]
struct ClaudeUsage {
    input_tokens: u32,
    output_tokens: u32,
}