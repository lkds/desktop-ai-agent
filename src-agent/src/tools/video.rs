/// Video Generation Tool
/// 使用外部 API 或本地工具生成视频

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::tool_trait::{Tool, StepResult, ToolError, RiskLevel};

/// Video Generator Tool
pub struct VideoGeneratorTool {
    api_key: Option<String>,
    provider: VideoProvider,
}

/// 支持的视频生成 Provider
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoProvider {
    Runway,
    Pika,
    Synthesia,
    Local,  // 使用 ffmpeg 本地合成
}

/// 视频生成参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoParams {
    pub input_type: InputType,
    pub content: String,
    pub output_path: String,
    #[serde(default = "default_duration")]
    pub duration: u32,
    #[serde(default = "default_resolution")]
    pub resolution: String,
    #[serde(default)]
    pub style: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InputType {
    Text,
    Image,
    Audio,
}

fn default_duration() -> u32 { 5 }
fn default_resolution() -> String { "1080p".to_string() }

impl VideoGeneratorTool {
    pub fn new(provider: VideoProvider, api_key: Option<String>) -> Self {
        Self { api_key, provider }
    }
    
    pub fn local() -> Self {
        Self::new(VideoProvider::Local, None)
    }
}

#[async_trait]
impl Tool for VideoGeneratorTool {
    fn name(&self) -> &str {
        "video_generate"
    }
    
    fn description(&self) -> &str {
        "Generate video from text description, image, or audio. Supports Runway, Pika, Synthesia APIs or local ffmpeg synthesis."
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "required": ["input_type", "content", "output_path"],
            "properties": {
                "input_type": {
                    "type": "string",
                    "enum": ["text", "image", "audio"],
                    "description": "Input type for video generation"
                },
                "content": {
                    "type": "string",
                    "description": "Text description or image/audio file path"
                },
                "output_path": {
                    "type": "string",
                    "description": "Output video file path"
                },
                "duration": {
                    "type": "integer",
                    "default": 5,
                    "description": "Video duration in seconds"
                },
                "resolution": {
                    "type": "string",
                    "default": "1080p",
                    "description": "Video resolution (720p, 1080p, 4k)"
                },
                "style": {
                    "type": "string",
                    "description": "Video style (cinematic, realistic, anime)"
                }
            }
        })
    }
    
    async fn execute(
        &self,
        parameters: serde_json::Value,
        allowed_paths: &[String],
    ) -> Result<StepResult, ToolError> {
        let params: VideoParams = serde_json::from_value(parameters)
            .map_err(|e| ToolError::InvalidParameters(e.to_string()))?;
        
        // 验证输出路径
        let output_path = PathBuf::from(&params.output_path);
        if !allowed_paths.iter().any(|p| output_path.starts_with(p)) {
            return Err(ToolError::PermissionDenied(params.output_path));
        }
        
        match self.provider {
            VideoProvider::Local => {
                self.generate_local(&params).await
            }
            VideoProvider::Runway | VideoProvider::Pika | VideoProvider::Synthesia => {
                if self.api_key.is_none() {
                    return Err(ToolError::ExecutionFailed(
                        format!("API key required for {} provider", self.provider_name())
                    ));
                }
                self.generate_via_api(&params).await
            }
        }
    }
    
    fn risk_level(&self) -> RiskLevel {
        RiskLevel::Medium  // 会创建新文件
    }
}

impl VideoGeneratorTool {
    fn provider_name(&self) -> &str {
        match self.provider {
            VideoProvider::Runway => "Runway",
            VideoProvider::Pika => "Pika",
            VideoProvider::Synthesia => "Synthesia",
            VideoProvider::Local => "Local (ffmpeg)",
        }
    }
    
    /// 本地视频生成（使用 ffmpeg）
    async fn generate_local(&self, params: &VideoParams) -> Result<StepResult, ToolError> {
        match params.input_type {
            InputType::Image => {
                // 图片转视频：使用 ffmpeg 创建静态视频
                let image_path = PathBuf::from(&params.content);
                if !image_path.exists() {
                    return Err(ToolError::FileNotFoundError(params.content.clone()));
                }
                
                // ffmpeg 命令
                let resolution = match params.resolution.as_str() {
                    "4k" => "3840:2160",
                    "720p" => "1280:720",
                    _ => "1920:1080",
                };
                
                let cmd = format!(
                    "ffmpeg -loop 1 -i {} -c:v libx264 -t {} -pix_fmt yuv420p -vf scale={} {} -y",
                    params.content, params.duration, resolution, params.output_path
                );
                
                // 执行命令
                let output = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(&cmd)
                    .output()
                    .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
                
                if !output.status.success() {
                    return Err(ToolError::ExecutionFailed(
                        String::from_utf8_lossy(&output.stderr).to_string()
                    ));
                }
                
                Ok(StepResult {
                    output: format!(
                        "Generated video: {} ({}s, {})",
                        params.output_path, params.duration, params.resolution
                    ),
                    files: Some(vec![params.output_path.clone()]),
                    data: None,
                })
            }
            InputType::Text => {
                // 文本转视频：本地只能创建简单字幕视频
                Err(ToolError::ExecutionFailed(
                    "Local text-to-video requires external API (Runway/Pika). Use image input for local generation.".to_string()
                ))
            }
            InputType::Audio => {
                // 音频+图片转视频
                Err(ToolError::ExecutionFailed(
                    "Local audio-to-video not implemented yet. Use Synthesia API for talking head videos.".to_string()
                ))
            }
        }
    }
    
    /// API 视频生成
    async fn generate_via_api(&self, params: &VideoParams) -> Result<StepResult, ToolError> {
        // TODO: 实现 API 调用
        // Runway API: https://api.runwayml.com/v1/generate
        // Pika API: https://api.pika.art/v1/generate
        // Synthesia API: https://api.synthesia.io/v2/videos
        
        Err(ToolError::ExecutionFailed(
            "API video generation not yet implemented. Configure provider and API key in config.".to_string()
        ))
    }
}