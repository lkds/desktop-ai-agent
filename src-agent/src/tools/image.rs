/// 图像处理工具
use async_trait::async_trait;
use super::tool_trait::{Tool, ToolError, StepResult, RiskLevel};

pub struct ImageResizeTool;

#[async_trait]
impl Tool for ImageResizeTool {
    fn name(&self) -> &str { "image_resize" }
    fn description(&self) -> &str { "调整图片尺寸" }
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "input": { "type": "string", "description": "输入图片路径" },
                "output": { "type": "string", "description": "输出图片路径" },
                "width": { "type": "integer", "description": "目标宽度" },
                "height": { "type": "integer", "description": "目标高度（可选）" }
            },
            "required": ["input", "output", "width"]
        })
    }
    fn risk_level(&self) -> RiskLevel { RiskLevel::Low }
    
    async fn execute(&self, params: serde_json::Value, _allowed: &[String]) -> Result<StepResult, ToolError> {
        let input = params["input"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("input required".into()))?;
        let output = params["output"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("output required".into()))?;
        let width = params["width"].as_i64()
            .ok_or_else(|| ToolError::InvalidParameters("width required".into()))?;
        
        // 使用 ffmpeg 或 imagemagick
        let result = tokio::process::Command::new("ffmpeg")
            .arg("-i").arg(input)
            .arg("-vf").arg(format!("scale={}:-1", width))
            .arg("-y").arg(output)
            .output()
            .await;
        
        match result {
            Ok(r) if r.status.success() => Ok(StepResult {
                output: format!("图片调整成功: {} -> {}", input, output),
                files: Some(vec![output.to_string()]),
                data: None,
            }),
            Ok(_) => Err(ToolError::ExecutionFailed("ffmpeg failed".into())),
            Err(_) => {
                // 尝试 imagemagick
                let result2 = tokio::process::Command::new("convert")
                    .arg(input)
                    .arg("-resize").arg(format!("{}x", width))
                    .arg(output)
                    .output()
                    .await;
                
                match result2 {
                    Ok(r) if r.status.success() => Ok(StepResult {
                        output: format!("图片调整成功 (ImageMagick): {} -> {}", input, output),
                        files: Some(vec![output.to_string()]),
                        data: None,
                    }),
                    _ => Ok(StepResult {
                        output: format!("提示: 请安装 ffmpeg 或 imagemagick 以处理图片"),
                        files: None,
                        data: None,
                    }),
                }
            }
        }
    }
}

pub struct ImageConvertTool;

#[async_trait]
impl Tool for ImageConvertTool {
    fn name(&self) -> &str { "image_convert" }
    fn description(&self) -> &str { "转换图片格式" }
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "input": { "type": "string", "description": "输入图片" },
                "output": { "type": "string", "description": "输出图片（带扩展名）" }
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
        
        let result = tokio::process::Command::new("ffmpeg")
            .arg("-i").arg(input)
            .arg("-y").arg(output)
            .output()
            .await;
        
        match result {
            Ok(r) if r.status.success() => Ok(StepResult {
                output: format!("格式转换成功: {} -> {}", input, output),
                files: Some(vec![output.to_string()]),
                data: None,
            }),
            _ => Ok(StepResult {
                output: format!("提示: 请安装 ffmpeg 以转换图片格式"),
                files: None,
                data: None,
            }),
        }
    }
}