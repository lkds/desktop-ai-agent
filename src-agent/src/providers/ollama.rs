/// Ollama Provider 实现
/// 本地模型，无需 API Key

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use futures::StreamExt;

use super::trait::*;

pub struct OllamaProvider {
    config: ProviderConfig,
    client: Client,
}

impl OllamaProvider {
    pub fn new(config: ProviderConfig) -> Result<Self, ProviderError> {
        let base_url = config.base_url.clone()
            .unwrap_or_else(|| "http://localhost:11434".to_string());
        
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(120)) // 本地模型可能较慢
            .build()
            .map_err(|e| ProviderError::InternalError(e.to_string()))?;
        
        Ok(Self { config, client })
    }
    
    fn get_base_url(&self) -> &str {
        self.config.base_url.as_deref()
            .unwrap_or("http://localhost:11434")
    }
}

#[async_trait]
impl Provider for OllamaProvider {
    fn name(&self) -> &str {
        "ollama"
    }
    
    async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse, ProviderError> {
        let url = format!("{}api/generate", self.get_base_url());
        
        let body = OllamaRequest {
            model: self.config.model.clone(),
            prompt: request.messages.iter()
                .map(|m| format!("{}: {}", 
                    match m.role {
                        Role::System => "System",
                        Role::User => "User",
                        Role::Assistant => "Assistant",
                        Role::Tool => "Tool",
                    },
                    m.content
                ))
                .join("\n\n"),
            stream: false,
        };
        
        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| ProviderError::ConnectionFailed(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(ProviderError::InvalidResponse(response.status().to_string()));
        }
        
        let ollama_response: OllamaResponse = response
            .json()
            .await
            .map_err(|e| ProviderError::InvalidResponse(e.to_string()))?;
        
        Ok(GenerateResponse {
            message: Message {
                role: Role::Assistant,
                content: ollama_response.response,
                tool_calls: None,
            },
            finish_reason: FinishReason::Stop,
            usage: Some(Usage {
                prompt_tokens: 0, // Ollama 不返回 token 数
                completion_tokens: 0,
                total_tokens: 0,
            }),
        })
    }
    
    async fn generate_stream(
        &self,
        request: GenerateRequest,
    ) -> Result<Box<dyn futures::Stream<Item = StreamChunk> + Unpin + Send>, ProviderError> {
        let url = format!("{}api/generate", self.get_base_url());
        
        let body = OllamaRequest {
            model: self.config.model.clone(),
            prompt: request.messages.iter()
                .map(|m| format!("{}: {}", 
                    match m.role {
                        Role::System => "System",
                        Role::User => "User",
                        Role::Assistant => "Assistant",
                        Role::Tool => "Tool",
                    },
                    m.content
                ))
                .join("\n\n"),
            stream: true,
        };
        
        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| ProviderError::ConnectionFailed(e.to_string()))?;
        
        let stream = response.bytes_stream()
            .filter_map(|chunk| async move {
                let bytes = chunk.ok()?;
                let text = String::from_utf8_lossy(&bytes);
                
                if let Ok(parsed) = serde_json::from_str::<OllamaStreamResponse>(&text) {
                    return Some(StreamChunk {
                        delta: parsed.response,
                        tool_calls: None,
                        finish_reason: if parsed.done { Some(FinishReason::Stop) } else { None },
                    });
                }
                None
            });
        
        Ok(Box::new(stream))
    }
    
    async fn health_check(&self) -> Result<bool, ProviderError> {
        let url = format!("{}api/tags", self.get_base_url());
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| ProviderError::ConnectionFailed(e.to_string()))?;
        
        Ok(response.status().is_success())
    }
    
    async fn list_models(&self) -> Result<Vec<String>, ProviderError> {
        let url = format!("{}api/tags", self.get_base_url());
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| ProviderError::ConnectionFailed(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(ProviderError::InvalidResponse(response.status().to_string()));
        }
        
        let models: OllamaModelsResponse = response
            .json()
            .await
            .map_err(|e| ProviderError::InvalidResponse(e.to_string()))?;
        
        Ok(models.models.iter().map(|m| m.name.clone()).collect())
    }
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
    done: bool,
}

#[derive(Deserialize)]
struct OllamaStreamResponse {
    response: String,
    done: bool,
}

#[derive(Deserialize)]
struct OllamaModelsResponse {
    models: Vec<OllamaModel>,
}

#[derive(Deserialize)]
struct OllamaModel {
    name: String,
}