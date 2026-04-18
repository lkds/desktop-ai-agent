/// OpenAI Provider 实现
/// 支持标准 OpenAI API 和 OpenAI-compatible endpoint

use async_trait::async_trait;
use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::trait::*;

/// OpenAI Provider
pub struct OpenAIProvider {
    config: ProviderConfig,
    client: Client,
}

impl OpenAIProvider {
    pub fn new(config: ProviderConfig) -> Result<Self, ProviderError> {
        let base_url = config.base_url.clone()
            .unwrap_or_else(|| "https://api.openai.com/v1".to_string());
        
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
    fn name(&self) -> &str {
        "openai"
    }
    
    async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse, ProviderError> {
        let api_key = self.get_api_key()?;
        let base_url = self.config.base_url.as_deref()
            .unwrap_or("https://api.openai.com/v1");
        
        let url = format!("{}/chat/completions", base_url);
        
        // 构建 OpenAI 格式的请求体
        let body = OpenAIRequest {
            model: self.config.model.clone(),
            messages: request.messages.iter().map(|m| OpenAIMessage {
                role: m.role.to_string(),
                content: m.content.clone(),
                tool_calls: m.tool_calls.clone(),
            }).collect(),
            tools: request.tools.map(|t| t.iter().map(|tool| OpenAITool {
                type: "function",
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
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| ProviderError::ConnectionFailed(e.to_string()))?;
        
        if !response.status().is_success() {
            let status = response.status();
            if status == 429 {
                return Err(ProviderError::RateLimited);
            }
            let text = response.text().await.unwrap_or_default();
            return Err(ProviderError::InvalidResponse(format!("{}: {}", status, text)));
        }
        
        let openai_response: OpenAIResponse = response
            .json()
            .await
            .map_err(|e| ProviderError::InvalidResponse(e.to_string()))?;
        
        // 转换为通用格式
        let choice = openai_response.choices.first()
            .ok_or_else(|| ProviderError::InvalidResponse("No choices returned".to_string()))?;
        
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
    
    async fn generate_stream(
        &self,
        request: GenerateRequest,
    ) -> Result<Box<dyn futures::Stream<Item = StreamChunk> + Unpin + Send>, ProviderError> {
        let api_key = self.get_api_key()?;
        let base_url = self.config.base_url.as_deref()
            .unwrap_or("https://api.openai.com/v1");
        
        let url = format!("{}/chat/completions", base_url);
        
        let body = OpenAIRequest {
            model: self.config.model.clone(),
            messages: request.messages.iter().map(|m| OpenAIMessage {
                role: m.role.to_string(),
                content: m.content.clone(),
                tool_calls: m.tool_calls.clone(),
            }).collect(),
            tools: request.tools.map(|t| t.iter().map(|tool| OpenAITool {
                type: "function",
                function: OpenAIFunction {
                    name: tool.name.clone(),
                    description: tool.description.clone(),
                    parameters: tool.parameters.clone(),
                },
            }).collect()),
            temperature: request.temperature,
            max_tokens: request.max_tokens,
            stream: true,
        };
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| ProviderError::ConnectionFailed(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(ProviderError::InvalidResponse(response.status().to_string()));
        }
        
        let stream = response.bytes_stream()
            .filter_map(|chunk| async move {
                let bytes = chunk.ok()?;
                let text = String::from_utf8_lossy(&bytes);
                
                // 解析 SSE 格式
                for line in text.lines() {
                    if line.starts_with("data: ") {
                        let data = &line[6..];
                        if data == "[DONE]" {
                            return Some(StreamChunk {
                                delta: String::new(),
                                tool_calls: None,
                                finish_reason: Some(FinishReason::Stop),
                            });
                        }
                        
                        if let Ok(parsed) = serde_json::from_str::<OpenAIStreamResponse>(data) {
                            let choice = parsed.choices.first()?;
                            let delta = choice.delta.content.clone().unwrap_or_default();
                            let tool_calls = choice.delta.tool_calls.clone();
                            let finish_reason = choice.finish_reason.clone().map(|r| match r.as_str() {
                                "stop" => FinishReason::Stop,
                                "tool_calls" => FinishReason::ToolCalls,
                                _ => FinishReason::Error,
                            });
                            
                            return Some(StreamChunk {
                                delta,
                                tool_calls,
                                finish_reason,
                            });
                        }
                    }
                }
                None
            });
        
        Ok(Box::new(stream))
    }
    
    async fn health_check(&self) -> Result<bool, ProviderError> {
        let api_key = self.get_api_key()?;
        let base_url = self.config.base_url.as_deref()
            .unwrap_or("https://api.openai.com/v1");
        
        let url = format!("{}/models", base_url);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await
            .map_err(|e| ProviderError::ConnectionFailed(e.to_string()))?;
        
        Ok(response.status().is_success())
    }
    
    async fn list_models(&self) -> Result<Vec<String>, ProviderError> {
        let api_key = self.get_api_key()?;
        let base_url = self.config.base_url.as_deref()
            .unwrap_or("https://api.openai.com/v1");
        
        let url = format!("{}/models", base_url);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await
            .map_err(|e| ProviderError::ConnectionFailed(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(ProviderError::InvalidResponse(response.status().to_string()));
        }
        
        let models: OpenAIModelsResponse = response
            .json()
            .await
            .map_err(|e| ProviderError::InvalidResponse(e.to_string()))?;
        
        Ok(models.data.iter().map(|m| m.id.clone()).collect())
    }
}

// --- OpenAI API 格式定义 ---

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    tools: Option<Vec<OpenAITool>>,
    temperature: Option<f32>,
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
    type: &'static str,
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

#[derive(Deserialize)]
struct OpenAIStreamResponse {
    choices: Vec<OpenAIStreamChoice>,
}

#[derive(Deserialize)]
struct OpenAIStreamChoice {
    delta: OpenAIDelta,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct OpenAIDelta {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Deserialize)]
struct OpenAIModelsResponse {
    data: Vec<OpenAIModel>,
}

#[derive(Deserialize)]
struct OpenAIModel {
    id: String,
}