/// 音频处理工具
use async_trait::async_trait;
use super::tool_trait::{Tool, ToolError, StepResult, RiskLevel};

pub struct AudioConvertTool;

#[async_trait]
impl Tool for AudioConvertTool {
    fn name(&self) -> &str { "audio_convert" }
    fn description(&self) -> &str { "转换音频格式" }
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "input": { "type": "string", "description": "输入音频文件" },
                "output": { "type": "string", "description": "输出音频文件" },
                "bitrate": { "type": "string", "description": "比特率如 128k", "default": "128k" }
            },
            "required": ["input", "output"]
        })
    }
    fn risk_level(&self) -> RiskLevel { RiskLevel::Low }
    
    async fn execute(&self, params: serde_json::Value, _allowed: &[String]) -> Result<StepResult, ToolError> {
        let input = params["input"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("input required".into()))?;
        let output = params["output"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("output required".into()))?;
        let bitrate = params["bitrate"].as_str().unwrap_or("128k");
        
        let result = tokio::process::Command::new("ffmpeg")
            .arg("-i").arg(input)
            .arg("-b:a").arg(bitrate)
            .arg("-y").arg(output)
            .output()
            .await;
        
        match result {
            Ok(r) if r.status.success() => Ok(StepResult {
                output: format!("音频转换成功: {} -> {}", input, output),
                files: Some(vec![output.to_string()]),
                data: None,
            }),
            _ => Ok(StepResult {
                output: format!("提示: 请安装 ffmpeg 以处理音频"),
                files: None,
                data: None,
            }),
        }
    }
}

pub struct AudioExtractTool;

#[async_trait]
impl Tool for AudioExtractTool {
    fn name(&self) -> &str { "audio_extract" }
    fn description(&self) -> &str { "从视频中提取音频" }
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "video": { "type": "string", "description": "视频文件" },
                "output": { "type": "string", "description": "输出音频文件" }
            },
            "required": ["video", "output"]
        })
    }
    fn risk_level(&self) -> RiskLevel { RiskLevel::Low }
    
    async fn execute(&self, params: serde_json::Value, _allowed: &[String]) -> Result<StepResult, ToolError> {
        let video = params["video"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("video required".into()))?;
        let output = params["output"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("output required".into()))?;
        
        let result = tokio::process::Command::new("ffmpeg")
            .arg("-i").arg(video)
            .arg("-vn")  // 无视频
            .arg("-acodec").arg("copy")
            .arg("-y").arg(output)
            .output()
            .await;
        
        match result {
            Ok(r) if r.status.success() => Ok(StepResult {
                output: format!("音频提取成功: {} -> {}", video, output),
                files: Some(vec![output.to_string()]),
                data: None,
            }),
            _ => Ok(StepResult {
                output: format!("提示: 请安装 ffmpeg"),
                files: None,
                data: None,
            }),
        }
    }
}