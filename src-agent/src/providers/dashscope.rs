/// DashScope Provider（阿里云百炼通用 API）
use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;
use super::provider_trait::*;

pub struct DashScopeProvider {
    config: ProviderConfig,
    client: Client,
}

impl DashScopeProvider {
    pub fn new(config: ProviderConfig) -> Result<Self, ProviderError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .map_err(|e| ProviderError::InternalError(e.to_string()))?;
        Ok(Self { config, client })
    }
}

#[async_trait]
impl Provider for DashScopeProvider {
    fn name(&self) -> &str { "dashscope" }
    
    async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse, ProviderError> {
        let api_key = self.config.api_key.as_ref()
            .ok_or(ProviderError::ApiKeyMissing)?;
        
        // 百炼通用 API base URL
        let base_url = self.config.base_url.as_deref()
            .unwrap_or("https://dashscope.aliyuncs.com/compatible-mode/v1");
        
        let url = format!("{}/chat/completions", base_url);
        
        let body = serde_json::json!({
            "model": self.config.model,
            "messages": request.messages.iter().map(|m| {
                serde_json::json!({
                    "role": m.role.to_string(),
                    "content": m.content
                })
            }).collect::<Vec<_>>(),
            "max_tokens": request.max_tokens,
            "temperature": request.temperature
        });
        
        let response = self.client.post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send().await
            .map_err(|e| ProviderError::ConnectionFailed(e.to_string()))?;
        
        let status = response.status();
        if !status.is_success() {
            if status == 429 { return Err(ProviderError::RateLimited); }
            let text = response.text().await.unwrap_or_default();
            return Err(ProviderError::InvalidResponse(format!("{}: {}", status, text)));
        }
        
        let resp: serde_json::Value = response.json().await
            .map_err(|e| ProviderError::InvalidResponse(e.to_string()))?;
        
        let content = resp["choices"][0]["message"]["content"]
            .as_str().unwrap_or_default();
        
        Ok(GenerateResponse {
            message: Message {
                role: Role::Assistant,
                content: content.to_string(),
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
        -> Result<Box<dyn futures::Stream<Item = StreamChunk> + Unpin + Send>, ProviderError> {
        Err(ProviderError::InternalError("Streaming not implemented".into()))
    }
    
    async fn health_check(&self) -> Result<bool, ProviderError> { Ok(true) }
    
    async fn list_models(&self) -> Result<Vec<String>, ProviderError> {
        Ok(vec!["qwen-plus".into(), "qwen-turbo".into(), "qwen-max".into()])
    }
}