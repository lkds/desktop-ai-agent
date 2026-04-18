/// Skills 系统
/// Skill = 封装的复杂工作流，包含 Prompt + 工具配置 + 执行策略

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;

/// Skill 定义（从 SKILL.md 解析）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    /// Skill ID
    pub id: String,
    /// Skill 名称
    pub name: String,
    /// Skill 描述
    pub description: String,
    /// 触发关键词
    pub triggers: Vec<String>,
    /// Skill 的 Prompt 模板
    pub prompt_template: String,
    /// 可用工具列表
    pub tools: Vec<String>,
    /// 执行参数
    pub parameters: HashMap<String, SkillParameter>,
    /// 示例输入输出
    pub examples: Vec<SkillExample>,
    /// Skill 来源路径
    pub source_path: String,
}

/// Skill 参数定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillParameter {
    pub description: String,
    pub required: bool,
    pub default: Option<String>,
}

/// Skill 示例
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillExample {
    pub input: String,
    pub output: String,
}

/// Skills Manager
pub struct SkillsManager {
    skills: HashMap<String, Skill>,
    skills_dir: PathBuf,
}

impl SkillsManager {
    pub fn new(skills_dir: PathBuf) -> Self {
        Self {
            skills: HashMap::new(),
            skills_dir,
        }
    }
    
    /// 加载所有 Skills
    pub async fn load_all(&mut self) -> Result<(), SkillError> {
        if !self.skills_dir.exists() {
            fs::create_dir_all(&self.skills_dir)
                .await
                .map_err(|e| SkillError::LoadError(e.to_string()))?;
            return Ok(());
        }
        
        let mut read_dir = fs::read_dir(&self.skills_dir)
            .await
            .map_err(|e| SkillError::LoadError(e.to_string()))?;
        
        while let Some(entry) = read_dir.next_entry().await.map_err(|e| SkillError::LoadError(e.to_string()))? {
            let path = entry.path();
            if path.is_dir() {
                let skill_file = path.join("SKILL.md");
                if skill_file.exists() {
                    let skill = self.load_skill(&skill_file)?;
                    self.skills.insert(skill.id.clone(), skill);
                }
            }
        }
        
        Ok(())
    }
    
    /// 加载单个 Skill
    fn load_skill(&self, path: &PathBuf) -> Result<Skill, SkillError> {
        let content = fs::read_to_string(path)
            .await
            .map_err(|e| SkillError::LoadError(e.to_string()))?;
        
        // 解析 SKILL.md 格式
        let skill = parse_skill_md(&content, path)?;
        
        Ok(skill)
    }
    
    /// 匹配 Skill（根据用户输入）
    pub fn match_skill(&self, input: &str) -> Option<&Skill> {
        for skill in self.skills.values() {
            for trigger in &skill.triggers {
                if input.contains(trigger) {
                    return Some(skill);
                }
            }
        }
        None
    }
    
    /// 获取 Skill
    pub fn get_skill(&self, id: &str) -> Option<&Skill> {
        self.skills.get(id)
    }
    
    /// 列出所有 Skills
    pub fn list_skills(&self) -> Vec<&Skill> {
        self.skills.values().collect()
    }
    
    /// 安装 Skill（从目录复制）
    pub async fn install_skill(&mut self, source_dir: &PathBuf) -> Result<String, SkillError> {
        let skill_file = source_dir.join("SKILL.md");
        if !skill_file.exists() {
            return Err(SkillError::InvalidSkill("SKILL.md not found"));
        }
        
        // 读取 Skill
        let content = fs::read_to_string(&skill_file)
            .await
            .map_err(|e| SkillError::LoadError(e.to_string()))?;
        let skill = parse_skill_md(&content, &skill_file)?;
        
        // 复制到 Skills 目录
        let dest_dir = self.skills_dir.join(&skill.id);
        copy_dir_all(source_dir, &dest_dir)
            .await
            .map_err(|e| SkillError::InstallError(e.to_string()))?;
        
        // 加载到内存
        self.skills.insert(skill.id.clone(), skill);
        
        Ok(skill.id)
    }
    
    /// 删除 Skill
    pub async fn uninstall_skill(&mut self, id: &str) -> Result<(), SkillError> {
        let skill_dir = self.skills_dir.join(id);
        if skill_dir.exists() {
            fs::remove_dir_all(&skill_dir)
                .await
                .map_err(|e| SkillError::UninstallError(e.to_string()))?;
        }
        
        self.skills.remove(id);
        
        Ok(())
    }
}

/// 解析 SKILL.md 文件
fn parse_skill_md(content: &str, path: &PathBuf) -> Result<Skill, SkillError> {
    let mut skill = Skill {
        id: path.parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string(),
        name: String::new(),
        description: String::new(),
        triggers: Vec::new(),
        prompt_template: String::new(),
        tools: Vec::new(),
        parameters: HashMap::new(),
        examples: Vec::new(),
        source_path: path.to_string_lossy().to_string(),
    };
    
    // 简单解析（实际应该用 markdown parser）
    let lines = content.lines();
    let mut current_section = "";
    
    for line in lines {
        if line.starts_with("# ") {
            skill.name = line[2..].to_string();
        } else if line.starts_with("## Description") {
            current_section = "description";
        } else if line.starts_with("## Triggers") {
            current_section = "triggers";
        } else if line.starts_with("## Prompt") {
            current_section = "prompt";
        } else if line.starts_with("## Tools") {
            current_section = "tools";
        } else if line.starts_with("## Parameters") {
            current_section = "parameters";
        } else if line.starts_with("## Examples") {
            current_section = "examples";
        } else if !line.is_empty() {
            match current_section {
                "description" => skill.description.push_str(line),
                "triggers" => {
                    if line.starts_with("- ") {
                        skill.triggers.push(line[2..].to_string());
                    }
                }
                "prompt" => skill.prompt_template.push_str(line),
                "tools" => {
                    if line.starts_with("- ") {
                        skill.tools.push(line[2..].to_string());
                    }
                }
                "parameters" => {
                    // 解析参数定义
                }
                "examples" => {
                    // 解析示例
                }
                _ => {}
            }
        }
    }
    
    Ok(skill)
}

/// 复制目录
async fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> Result<(), std::io::Error> {
    fs::create_dir_all(dst)?;
    
    let mut read_dir = fs::read_dir(src)?;
    
    while let Some(entry) = read_dir.next_entry()? {
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if src_path.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    
    Ok(())
}

/// Skill 错误
#[derive(Debug, Clone)]
pub enum SkillError {
    LoadError(String),
    InvalidSkill(String),
    InstallError(String),
    UninstallError(String),
    ExecutionError(String),
}

impl std::fmt::Display for SkillError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoadError(msg) => write!(f, "Load error: {}", msg),
            Self::InvalidSkill(msg) => write!(f, "Invalid skill: {}", msg),
            Self::InstallError(msg) => write!(f, "Install error: {}", msg),
            Self::UninstallError(msg) => write!(f, "Uninstall error: {}", msg),
            Self::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
        }
    }
}

impl std::error::Error for SkillError {}

use tokio::fs;