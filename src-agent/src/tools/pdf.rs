/// PDF 处理工具
use async_trait::async_trait;
use super::tool_trait::{Tool, ToolError, StepResult, RiskLevel};

pub struct PDFExtractTool;

#[async_trait]
impl Tool for PDFExtractTool {
    fn name(&self) -> &str { "pdf_extract_text" }
    fn description(&self) -> &str { "提取 PDF 文件文本内容" }
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "PDF 文件路径" },
                "pages": { "type": "string", "description": "页码范围，如 '1-10' 或 'all'" }
            },
            "required": ["path"]
        })
    }
    fn risk_level(&self) -> RiskLevel { RiskLevel::Low }
    
    async fn execute(&self, params: serde_json::Value, _allowed: &[String]) -> Result<StepResult, ToolError> {
        let path = params["path"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("path required".into()))?;
        
        // 使用 pdftotext（需要安装 poppler-utils）
        let output = tokio::process::Command::new("pdftotext")
            .arg(path)
            .arg("-")
            .output()
            .await;
        
        match output {
            Ok(result) if result.status.success() => {
                let text = String::from_utf8_lossy(&result.stdout);
                Ok(StepResult {
                    output: text.to_string(),
                    files: Some(vec![path.to_string()]),
                    data: None,
                })
            }
            Ok(_) => Err(ToolError::ExecutionFailed("pdftotext failed. Install poppler-utils.".into())),
            Err(_) => {
                // 如果没有 pdftotext，返回提示
                Ok(StepResult {
                    output: format!("PDF 文件: {}\n\n提示: 请安装 poppler-utils (pdftotext) 以提取 PDF 内容\nUbuntu: apt install poppler-utils\nMacOS: brew install poppler", path),
                    files: None,
                    data: None,
                })
            }
        }
    }
}

pub struct PDFMergeTool;

#[async_trait]
impl Tool for PDFMergeTool {
    fn name(&self) -> &str { "pdf_merge" }
    fn description(&self) -> &str { "合并多个 PDF 文件" }
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "files": { "type": "array", "description": "PDF 文件路径列表" },
                "output": { "type": "string", "description": "输出文件路径" }
            },
            "required": ["files", "output"]
        })
    }
    fn risk_level(&self) -> RiskLevel { RiskLevel::Medium }
    
    async fn execute(&self, params: serde_json::Value, _allowed: &[String]) -> Result<StepResult, ToolError> {
        let files: Vec<String> = params["files"]
            .as_array()
            .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .ok_or_else(|| ToolError::InvalidParameters("files array required".into()))?;
        
        let output = params["output"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("output required".into()))?;
        
        // 使用 pdfmerge 或 pdftk
        let mut cmd = tokio::process::Command::new("pdftk");
        for file in &files {
            cmd.arg(file);
        }
        cmd.arg("cat").arg("output").arg(output);
        
        let result = cmd.output().await;
        
        match result {
            Ok(r) if r.status.success() => {
                Ok(StepResult {
                    output: format!("PDF 合并成功: {} ({} 个文件)", output, files.len()),
                    files: Some(vec![output.to_string()]),
                    data: None,
                })
            }
            Ok(_) => Err(ToolError::ExecutionFailed("pdftk failed. Install pdftk.".into())),
            Err(_) => {
                Ok(StepResult {
                    output: format!("PDF 合合任务: {} -> {}\n\n提示: 请安装 pdftk 或 pdfmerge", files.join(", "), output),
                    files: None,
                    data: None,
                })
            }
        }
    }
}