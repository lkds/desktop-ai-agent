/// DashScope Provider（阿里云百炼通用 API）- 完整版含 Streaming
use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;
use std::pin::Pin;
use futures::{Stream, StreamExt};
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
        
        let base_url = self.config.base_url.as_deref()
            .unwrap_or("https://dashscope.aliyuncs.com/compatible-mode/v1");
        
        let url = format!("{}/chat/completions", base_url);
        
        let mut body = serde_json::json!({
            "model": self.config.model,
            "messages": request.messages.iter().map(|m| {
                serde_json::json!({
                    "role": m.role.to_string(),
                    "content": m.content
                })
            }).collect::<Vec<_>>(),
            "max_tokens": request.max_tokens,
            "temperature": request.temperature,
        });
        
        if let Some(tools) = request.tools {
            body["tools"] = serde_json::to_value(&tools).unwrap();
        }
        
        let response = self.client.post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send().await
            .map_err(|e| ProviderError::ConnectionFailed(e.to_string()))?;
        
        let status = response.status();
        if !status.is_success() {
            if status == 429 { return Err(ProviderError::RateLimited); }
            if status == 401 { return Err(ProviderError::ApiKeyMissing); }
            let text = response.text().await.unwrap_or_default();
            return Err(ProviderError::InvalidResponse(format!("{}: {}", status, text)));
        }
        
        let resp: serde_json::Value = response.json().await
            .map_err(|e| ProviderError::InvalidResponse(e.to_string()))?;
        
        let content = resp["choices"][0]["message"]["content"]
            .as_str().unwrap_or_default();
        
        let tool_calls = resp["choices"][0]["message"]["tool_calls"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|tc| {
                Some(ToolCall {
                    id: tc["id"].as_str()?.to_string(),
                    name: tc["function"]["name"].as_str()?.to_string(),
                    arguments: tc["function"]["arguments"].as_str()?.to_string(),
                })
            }).collect());
        
        let finish_reason = match resp["choices"][0]["finish_reason"].as_str() {
            Some("stop") => FinishReason::Stop,
            Some("tool_calls") => FinishReason::ToolCalls,
            Some("length") => FinishReason::Length,
            _ => FinishReason::Stop,
        };
        
        let usage = resp["usage"].as_object().map(|u| Usage {
            prompt_tokens: u["prompt_tokens"].as_u64().unwrap_or(0) as u32,
            completion_tokens: u["completion_tokens"].as_u64().unwrap_or(0) as u32,
            total_tokens: u["total_tokens"].as_u64().unwrap_or(0) as u32,
        });
        
        Ok(GenerateResponse {
            message: Message { role: Role::Assistant, content: content.to_string(), tool_calls },
            finish_reason,
            usage,
        })
    }
    
    async fn generate_stream(
        &self, 
        request: GenerateRequest
    ) -> Result<Pin<Box<dyn Stream<Item = StreamChunk> + Send>>, ProviderError> {
        let api_key = self.config.api_key.as_ref()
            .ok_or(ProviderError::ApiKeyMissing)?
            .clone();
        
        let base_url = self.config.base_url.as_deref()
            .unwrap_or("https://dashscope.aliyuncs.com/compatible-mode/v1");
        
        let url = format!("{}/chat/completions", base_url);
        
        let body = serde_json::json!({
            "model": self.config.model,
            "messages": request.messages.iter().map(|m| {
                serde_json::json!({ "role": m.role.to_string(), "content": m.content })
            }).collect::<Vec<_>>(),
            "max_tokens": request.max_tokens,
            "temperature": request.temperature,
            "stream": true,
        });
        
        let response = self.client.post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send().await
            .map_err(|e| ProviderError::ConnectionFailed(e.to_string()))?;
        
        let status = response.status();
        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(ProviderError::InvalidResponse(format!("{}: {}", status, text)));
        }
        
        let stream = response.bytes_stream()
            .then(|bytes_result| async move {
                match bytes_result {
                    Ok(bytes) => {
                        let line = String::from_utf8_lossy(&bytes).to_string();
                        if line.starts_with("data: ") {
                            let json_str = &line[6..];
                            if json_str.trim() == "[DONE]" {
                                return StreamChunk { delta: String::new(), tool_calls: None, finish_reason: Some(FinishReason::Stop) };
                            }
                            if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_str.trim()) {
                                let delta = json["choices"][0]["delta"]["content"]
                                    .as_str().unwrap_or_default().to_string();
                                let finish_reason = json["choices"][0]["finish_reason"]
                                    .as_str().map(|r| match r {
                                        "stop" => FinishReason::Stop,
                                        "tool_calls" => FinishReason::ToolCalls,
                                        "length" => FinishReason::Length,
                                        _ => FinishReason::Stop,
                                    });
                                return StreamChunk { delta, tool_calls: None, finish_reason };
                            }
                        }
                        StreamChunk { delta: String::new(), tool_calls: None, finish_reason: None }
                    }
                    Err(_) => StreamChunk { delta: String::new(), tool_calls: None, finish_reason: None }
                }
            })
            .filter(|chunk| futures::future::ready(!chunk.delta.is_empty() || chunk.finish_reason.is_some()));
        
        Ok(Box::pin(stream))
    }
    
    async fn health_check(&self) -> Result<bool, ProviderError> {
        let request = GenerateRequest {
            messages: vec![Message { role: Role::User, content: "ping".to_string(), tool_calls: None }],
            tools: None,
            temperature: Some(0.1),
            max_tokens: Some(10),
            stream: false,
        };
        match self.generate(request).await {
            Ok(_) => Ok(true),
            Err(ProviderError::RateLimited) => Ok(true),
            Err(e) => Err(e),
        }
    }
    
    async fn list_models(&self) -> Result<Vec<String>, ProviderError> {
        Ok(vec!["qwen-plus".into(), "qwen-turbo".into(), "qwen-max".into(), "qwen-long".into(), "qwen-vl-plus".into(), "qwen-vl-max".into()])
    }
}