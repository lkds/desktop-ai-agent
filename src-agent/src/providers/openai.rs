/// OpenAI Provider 实现
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::pin::Pin;
use futures::Stream;
use super::provider_trait::*;

pub struct OpenAIProvider {
    config: ProviderConfig,
    client: Client,
}

impl OpenAIProvider {
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
impl Provider for OpenAIProvider {
    fn name(&self) -> &str { "openai" }
    
    async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse, ProviderError> {
        let api_key = self.get_api_key()?;
        let base_url = self.config.base_url.as_deref().unwrap_or("https://api.openai.com/v1");
        let url = format!("{}/chat/completions", base_url);
        
        let body = OpenAIRequest {
            model: self.config.model.clone(),
            messages: request.messages.iter().map(|m| OpenAIMessage {
                role: m.role.to_string(),
                content: m.content.clone(),
                tool_calls: m.tool_calls.clone(),
            }).collect(),
            tools: request.tools.map(|t| t.iter().map(|tool| OpenAITool {
                kind: "function",
                function: OpenAIFunction {
                    name: tool.name.clone(),
                    description: tool.description.clone(),
                    parameters: tool.parameters.clone(),
                },
            }).collect()),
            temperature: request.temperature,
            max_tokens: request.max_tokens,
            stream: false,
        };
        
        let response = self.client.post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send().await
            .map_err(|e| ProviderError::ConnectionFailed(e.to_string()))?;
        
        if !response.status().is_success() {
            if response.status() == 429 { return Err(ProviderError::RateLimited); }
            return Err(ProviderError::InvalidResponse(response.status().to_string()));
        }
        
        let openai_response: OpenAIResponse = response.json().await
            .map_err(|e| ProviderError::InvalidResponse(e.to_string()))?;
        
        let choice = openai_response.choices.first()
            .ok_or_else(|| ProviderError::InvalidResponse("No choices".to_string()))?;
        
        Ok(GenerateResponse {
            message: Message {
                role: Role::Assistant,
                content: choice.message.content.clone().unwrap_or_default(),
                tool_calls: choice.message.tool_calls.clone(),
            },
            finish_reason: match choice.finish_reason.as_str() {
                "stop" => FinishReason::Stop,
                "tool_calls" => FinishReason::ToolCalls,
                "length" => FinishReason::Length,
                _ => FinishReason::Error,
            },
            usage: Some(Usage {
                prompt_tokens: openai_response.usage.prompt_tokens,
                completion_tokens: openai_response.usage.completion_tokens,
                total_tokens: openai_response.usage.total_tokens,
            }),
        })
    }
    
    async fn generate_stream(&self, _request: GenerateRequest) 
        -> Result<Pin<Box<dyn Stream<Item = StreamChunk> + Send>>, ProviderError> {
        Err(ProviderError::InternalError("Streaming not implemented".to_string()))
    }
    
    async fn health_check(&self) -> Result<bool, ProviderError> {
        let api_key = self.get_api_key()?;
        let base_url = self.config.base_url.as_deref().unwrap_or("https://api.openai.com/v1");
        let response = self.client.get(&format!("{}/models", base_url))
            .header("Authorization", format!("Bearer {}", api_key))
            .send().await
            .map_err(|e| ProviderError::ConnectionFailed(e.to_string()))?;
        Ok(response.status().is_success())
    }
    
    async fn list_models(&self) -> Result<Vec<String>, ProviderError> {
        Ok(vec!["gpt-4".to_string(), "gpt-3.5-turbo".to_string()])
    }
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OpenAITool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    stream: bool,
}

#[derive(Serialize)]
struct OpenAIMessage {
    role: String,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Serialize)]
struct OpenAITool {
    #[serde(rename = "type")]
    kind: &'static str,
    function: OpenAIFunction,
}

#[derive(Serialize)]
struct OpenAIFunction {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
    usage: OpenAIUsage,
}

#[derive(Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessageResponse,
    finish_reason: String,
}

#[derive(Deserialize)]
struct OpenAIMessageResponse {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Deserialize)]
struct OpenAIUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}