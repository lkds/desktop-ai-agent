/// 视频生成工具
use async_trait::async_trait;
use super::tool_trait::{Tool, ToolError, StepResult, RiskLevel};

pub struct VideoGenerateTool;

#[async_trait]
impl Tool for VideoGenerateTool {
    fn name(&self) -> &str { "video_generate" }
    fn description(&self) -> &str { "生成视频（使用 ffmpeg）" }
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "images": { "type": "array", "description": "图片路径列表" },
                "audio": { "type": "string", "description": "音频文件路径（可选）" },
                "output": { "type": "string", "description": "输出视频路径" },
                "fps": { "type": "integer", "description": "帧率", "default": 24 },
                "duration": { "type": "integer", "description": "每张图片显示时长（秒）", "default": 3 }
            },
            "required": ["images", "output"]
        })
    }
    fn risk_level(&self) -> RiskLevel { RiskLevel::Medium }
    
    async fn execute(&self, params: serde_json::Value, _allowed: &[String]) -> Result<StepResult, ToolError> {
        let images: Vec<String> = params["images"]
            .as_array()
            .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .ok_or_else(|| ToolError::InvalidParameters("images array required".into()))?;
        
        let output = params["output"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("output path required".into()))?;
        
        let fps = params["fps"].as_i64().unwrap_or(24);
        let duration = params["duration"].as_i64().unwrap_or(3);
        let audio = params["audio"].as_str();
        
        // 创建临时文件列表
        let concat_file = "/tmp/video_concat.txt";
        let mut concat_content = String::new();
        for img in &images {
            concat_content.push_str(&format!("file '{}'\nduration {}\n", img, duration));
        }
        concat_content.push_str(&format!("file '{}'\n", images.last().unwrap()));
        
        std::fs::write(concat_file, &concat_content)
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        
        // 构建 ffmpeg 命令
        let mut cmd = tokio::process::Command::new("ffmpeg");
        cmd.arg("-y")
           .arg("-f").arg("concat")
           .arg("-safe").arg("0")
           .arg("-i").arg(concat_file);
        
        if let Some(audio_file) = audio {
            cmd.arg("-i").arg(audio_file)
               .arg("-c:v").arg("libx264")
               .arg("-c:a").arg("aac")
               .arg("-shortest");
        } else {
            cmd.arg("-c:v").arg("libx264")
               .arg("-r").arg(format!("{}", fps));
        }
        
        cmd.arg("-pix_fmt").arg("yuv420p")
           .arg(output);
        
        let result = cmd.output().await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        
        if !result.status.success() {
            let stderr = String::from_utf8_lossy(&result.stderr);
            return Err(ToolError::ExecutionFailed(format!("ffmpeg failed: {}", stderr)));
        }
        
        Ok(StepResult {
            output: format!("视频生成成功: {} ({} 张图片)", output, images.len()),
            files: Some(vec![output.to_string()]),
            data: None,
        })
    }
}

pub struct VideoFromTextTool;

#[async_trait]
impl Tool for VideoFromTextTool {
    fn name(&self) -> &str { "video_from_text" }
    fn description(&self) -> &str { "从文本生成视频（调用 API）" }
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "text": { "type": "string", "description": "文本内容或脚本" },
                "style": { "type": "string", "description": "视频风格", "default": "default" },
                "output": { "type": "string", "description": "输出路径" }
            },
            "required": ["text", "output"]
        })
    }
    fn risk_level(&self) -> RiskLevel { RiskLevel::Low }
    
    async fn execute(&self, params: serde_json::Value, _allowed: &[String]) -> Result<StepResult, ToolError> {
        let text = params["text"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("text required".into()))?;
        let output = params["output"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("output required".into()))?;
        
        // 这里应该调用视频生成 API（如 RunwayML、Pika 等）
        // 简化版：生成占位符视频
        let placeholder = format!(
            "视频生成任务已提交:\n文本: {}\n输出: {}\n\n提示: 请配置视频生成 API (RunwayML/Pika/Sora) 完成实际生成",
            if text.len() > 100 { &text[..100] } else { text },
            output
        );
        
        Ok(StepResult {
            output: placeholder,
            files: None,
            data: None,
        })
    }
}