/// Skills Manager
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub triggers: Vec<String>,
    pub prompt_template: String,
    pub tools: Vec<String>,
    pub parameters: HashMap<String, SkillParameter>,
    pub examples: Vec<SkillExample>,
    pub source_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillParameter {
    pub description: String,
    pub required: bool,
    pub default: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillExample {
    pub input: String,
    pub output: String,
}

pub struct SkillsManager {
    skills: HashMap<String, Skill>,
    skills_dir: PathBuf,
}

impl SkillsManager {
    pub fn new(skills_dir: PathBuf) -> Self {
        Self { skills: HashMap::new(), skills_dir }
    }
    
    pub async fn load_all(&mut self) -> Result<(), SkillError> {
        if !self.skills_dir.exists() {
            fs::create_dir_all(&self.skills_dir).await
                .map_err(|e| SkillError::LoadError(e.to_string()))?;
            return Ok(());
        }
        
        let mut entries = fs::read_dir(&self.skills_dir).await
            .map_err(|e| SkillError::LoadError(e.to_string()))?;
        
        while let Some(entry) = entries.next_entry().await.map_err(|e| SkillError::LoadError(e.to_string()))? {
            let path = entry.path();
            if path.is_dir() {
                let skill_file = path.join("SKILL.md");
                if skill_file.exists() {
                    if let Ok(skill) = self.load_skill(&skill_file).await {
                        self.skills.insert(skill.id.clone(), skill);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    async fn load_skill(&self, path: &PathBuf) -> Result<Skill, SkillError> {
        let content = fs::read_to_string(path).await
            .map_err(|e| SkillError::LoadError(e.to_string()))?;
        
        let skill = parse_skill_md(&content, path)?;
        Ok(skill)
    }
    
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
    
    pub fn get_skill(&self, id: &str) -> Option<&Skill> {
        self.skills.get(id)
    }
    
    pub fn list_skills(&self) -> Vec<&Skill> {
        self.skills.values().collect()
    }
    
    pub async fn install_skill(&mut self, source_dir: &PathBuf) -> Result<String, SkillError> {
        let skill_file = source_dir.join("SKILL.md");
        if !skill_file.exists() {
            return Err(SkillError::InvalidSkill("SKILL.md not found".to_string()));
        }
        
        let content = fs::read_to_string(&skill_file).await
            .map_err(|e| SkillError::LoadError(e.to_string()))?;
        let skill = parse_skill_md(&content, &skill_file)?;
        
        let dest_dir = self.skills_dir.join(&skill.id);
        copy_dir_all(source_dir, &dest_dir).await
            .map_err(|e| SkillError::InstallError(e.to_string()))?;
        
        let skill_id = skill.id.clone();
        self.skills.insert(skill_id.clone(), skill);
        Ok(skill_id)
    }
    
    pub async fn uninstall_skill(&mut self, id: &str) -> Result<(), SkillError> {
        let skill_dir = self.skills_dir.join(id);
        if skill_dir.exists() {
            fs::remove_dir_all(&skill_dir).await
                .map_err(|e| SkillError::UninstallError(e.to_string()))?;
        }
        self.skills.remove(id);
        Ok(())
    }
}

fn parse_skill_md(content: &str, path: &PathBuf) -> Result<Skill, SkillError> {
    let id = path.parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    let mut skill = Skill {
        id,
        name: String::new(),
        description: String::new(),
        triggers: Vec::new(),
        prompt_template: String::new(),
        tools: Vec::new(),
        parameters: HashMap::new(),
        examples: Vec::new(),
        source_path: path.to_string_lossy().to_string(),
    };
    
    let mut current_section = "";
    
    for line in content.lines() {
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
        } else if !line.is_empty() {
            match current_section {
                "description" => skill.description.push_str(line),
                "triggers" => if line.starts_with("- ") { skill.triggers.push(line[2..].to_string()); },
                "prompt" => skill.prompt_template.push_str(line),
                "tools" => if line.starts_with("- ") { skill.tools.push(line[2..].to_string()); },
                _ => {}
            }
        }
    }
    
    Ok(skill)
}

async fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> Result<(), std::io::Error> {
    fs::create_dir_all(dst).await?;
    let mut entries = fs::read_dir(src).await?;
    while let Some(entry) = entries.next_entry().await? {
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            Box::pin(copy_dir_all(&src_path, &dst_path)).await?;
        } else {
            fs::copy(&src_path, &dst_path).await?;
        }
    }
    Ok(())
}

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