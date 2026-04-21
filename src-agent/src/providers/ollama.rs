/// Ollama Provider 实现 (本地模型)
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::pin::Pin;
use futures::Stream;
use super::provider_trait::*;

pub struct OllamaProvider {
    config: ProviderConfig,
    client: Client,
}

impl OllamaProvider {
    pub fn new(config: ProviderConfig) -> Result<Self, ProviderError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .map_err(|e| ProviderError::InternalError(e.to_string()))?;
        Ok(Self { config, client })
    }
    
    fn get_base_url(&self) -> &str {
        self.config.base_url.as_deref().unwrap_or("http://localhost:11434")
    }
}

#[async_trait]
impl Provider for OllamaProvider {
    fn name(&self) -> &str { "ollama" }
    
    async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse, ProviderError> {
        let url = format!("{}api/generate", self.get_base_url());
        
        let prompt = request.messages.iter()
            .map(|m| format!("{}: {}", 
                match m.role {
                    Role::System => "System",
                    Role::User => "User",
                    Role::Assistant => "Assistant",
                    Role::Tool => "Tool",
                },
                m.content
            ))
            .collect::<Vec<_>>()
            .join("\n\n");
        
        let body = OllamaRequest {
            model: self.config.model.clone(),
            prompt,
            stream: false,
        };
        
        let response = self.client.post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send().await
            .map_err(|e| ProviderError::ConnectionFailed(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(ProviderError::InvalidResponse(response.status().to_string()));
        }
        
        let ollama_response: OllamaResponse = response.json().await
            .map_err(|e| ProviderError::InvalidResponse(e.to_string()))?;
        
        Ok(GenerateResponse {
            message: Message {
                role: Role::Assistant,
                content: ollama_response.response,
                tool_calls: None,
            },
            finish_reason: FinishReason::Stop,
            usage: Some(Usage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0,
            }),
        })
    }
    
    async fn generate_stream(&self, _request: GenerateRequest) 
        -> Result<Pin<Box<dyn Stream<Item = StreamChunk> + Send>>, ProviderError> {
        Err(ProviderError::InternalError("Streaming not implemented".to_string()))
    }
    
    async fn health_check(&self) -> Result<bool, ProviderError> {
        let response = self.client.get(&format!("{}api/tags", self.get_base_url()))
            .send().await
            .map_err(|e| ProviderError::ConnectionFailed(e.to_string()))?;
        Ok(response.status().is_success())
    }
    
    async fn list_models(&self) -> Result<Vec<String>, ProviderError> {
        let response = self.client.get(&format!("{}api/tags", self.get_base_url()))
            .send().await
            .map_err(|e| ProviderError::ConnectionFailed(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(ProviderError::InvalidResponse(response.status().to_string()));
        }
        
        let models: OllamaModelsResponse = response.json().await
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
#[allow(dead_code)]
struct OllamaResponse {
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