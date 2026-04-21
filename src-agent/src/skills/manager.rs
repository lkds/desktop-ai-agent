/// Skills Manager - 增强版解析器
/// 使用 pulldown-cmark 进行真正的 Markdown 解析

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;
use tokio::fs;
use pulldown_cmark::{Parser, Event, Tag, HeadingLevel};

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
    pub version: String,
    pub author: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillParameter {
    pub description: String,
    #[serde(rename = "type")]
    pub param_type: String,
    pub required: bool,
    pub default: Option<String>,
    pub enum_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillExample {
    pub input: String,
    pub output: String,
    pub explanation: Option<String>,
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
                    match self.load_skill(&skill_file).await {
                        Ok(skill) => {
                            let id = skill.id.clone();
                            self.skills.insert(id.clone(), skill);
                            tracing::info!("Loaded skill: {}", id);
                        }
                        Err(e) => {
                            tracing::warn!("Failed to load skill from {}: {}", skill_file.display(), e);
                        }
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
        let input_lower = input.to_lowercase();
        let mut matches: Vec<(usize, &Skill)> = Vec::new();
        
        for skill in self.skills.values() {
            for trigger in &skill.triggers {
                let trigger_lower = trigger.to_lowercase();
                if input_lower.contains(&trigger_lower) {
                    matches.push((trigger.len(), skill));
                    break;
                }
            }
        }
        
        matches.sort_by(|a, b| b.0.cmp(&a.0));
        matches.first().map(|(_, skill)| *skill)
    }
    
    pub fn match_skills_fuzzy(&self, input: &str, limit: usize) -> Vec<&Skill> {
        let input_lower = input.to_lowercase();
        let words: Vec<&str> = input_lower.split_whitespace().collect();
        let mut matches: Vec<(usize, &Skill)> = Vec::new();
        
        for skill in self.skills.values() {
            let mut score = 0;
            for trigger in &skill.triggers {
                let trigger_lower = trigger.to_lowercase();
                for word in &words {
                    if trigger_lower.contains(word) || word.contains(&trigger_lower) {
                        score += 1;
                    }
                }
            }
            if score > 0 {
                matches.push((score, skill));
            }
        }
        
        matches.sort_by(|a, b| b.0.cmp(&a.0));
        matches.into_iter().take(limit).map(|(_, skill)| skill).collect()
    }
    
    pub fn get_skill(&self, id: &str) -> Option<&Skill> {
        self.skills.get(id)
    }
    
    pub fn list_skills(&self) -> Vec<&Skill> {
        self.skills.values().collect()
    }
    
    pub fn skills_count(&self) -> usize {
        self.skills.len()
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
        tracing::info!("Installed skill: {}", skill_id);
        Ok(skill_id)
    }
    
    pub async fn uninstall_skill(&mut self, id: &str) -> Result<(), SkillError> {
        let skill_dir = self.skills_dir.join(id);
        if skill_dir.exists() {
            fs::remove_dir_all(&skill_dir).await
                .map_err(|e| SkillError::UninstallError(e.to_string()))?;
        }
        self.skills.remove(id);
        tracing::info!("Uninstalled skill: {}", id);
        Ok(())
    }
    
    pub fn generate_prompt(&self, skill_id: &str, params: HashMap<String, String>) -> Option<String> {
        let skill = self.skills.get(skill_id)?;
        let mut prompt = skill.prompt_template.clone();
        
        for (key, value) in &params {
            prompt = prompt.replace(&format!("{{{{{}}}}}", key), value);
        }
        
        for (key, param) in &skill.parameters {
            if !params.contains_key(key) && param.default.is_some() {
                prompt = prompt.replace(&format!("{{{{{}}}}}", key), param.default.as_ref().unwrap());
            }
        }
        
        Some(prompt)
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
        version: "1.0".to_string(),
        author: None,
    };
    
    let parser = Parser::new(content);
    let mut current_section: String = String::new();
    let mut current_text = String::new();
    let mut list_items: Vec<String> = Vec::new();
    
    for event in parser {
        match event {
            Event::Start(Tag::Heading(_level, ..)) => {
                save_section_content(&mut skill, &current_section, &current_text, &list_items);
                current_text.clear();
                list_items.clear();
                current_section.clear();
            }
            Event::End(Tag::Heading(level, ..)) => {
                if level == HeadingLevel::H1 {
                    skill.name = current_text.trim().to_string();
                } else if level == HeadingLevel::H2 {
                    current_section = current_text.trim().to_lowercase();
                }
            }
            Event::Text(text) => { current_text.push_str(&text); }
            Event::Code(text) => { current_text.push_str(&text); }
            Event::Start(Tag::CodeBlock(_)) => {}
            Event::End(Tag::CodeBlock(_)) => {
                if current_section == "prompt" {
                    skill.prompt_template = current_text.trim().to_string();
                }
            }
            Event::Start(Tag::List(_)) => { list_items.clear(); }
            Event::End(Tag::List(_)) => {
                if current_section == "triggers" { skill.triggers = list_items.clone(); }
                else if current_section == "tools" { skill.tools = list_items.clone(); }
                list_items.clear();
            }
            Event::Start(Tag::Item) => { current_text.clear(); }
            Event::End(Tag::Item) => {
                if !current_text.trim().is_empty() { list_items.push(current_text.trim().to_string()); }
            }
            Event::Start(Tag::Paragraph) => { current_text.clear(); }
            Event::End(Tag::Paragraph) => {
                if current_section == "description" && skill.description.is_empty() {
                    skill.description = current_text.trim().to_string();
                }
            }
            _ => {}
        }
    }
    
    save_section_content(&mut skill, &current_section, &current_text, &list_items);
    
    if content.starts_with("---") {
        if let Some(yaml_end) = content.find("\n---") {
            parse_yaml_frontmatter(&content[4..yaml_end], &mut skill);
        }
    }
    
    if skill.name.is_empty() { skill.name = skill.id.clone(); }
    Ok(skill)
}

fn save_section_content(skill: &mut Skill, section: &str, text: &str, list: &[String]) {
    match section {
        "description" if skill.description.is_empty() => { skill.description = text.trim().to_string(); }
        "triggers" => { skill.triggers = list.to_vec(); }
        "tools" => { skill.tools = list.to_vec(); }
        "prompt" => { skill.prompt_template = text.trim().to_string(); }
        _ => {}
    }
}

fn parse_yaml_frontmatter(yaml: &str, skill: &mut Skill) {
    for line in yaml.lines() {
        if line.contains(':') {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let value = parts[1].trim();
                match key {
                    "version" => skill.version = value.to_string(),
                    "author" => skill.author = Some(value.to_string()),
                    "triggers" => {
                        if value.starts_with('[') && value.ends_with(']') {
                            skill.triggers = value[1..value.len()-1].split(',')
                                .map(|s| s.trim().trim_matches('"').to_string())
                                .collect();
                        }
                    }
                    _ => {}
                }
            }
        }
    }
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
    ParseError(String),
}

impl std::fmt::Display for SkillError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoadError(msg) => write!(f, "Load error: {}", msg),
            Self::InvalidSkill(msg) => write!(f, "Invalid skill: {}", msg),
            Self::InstallError(msg) => write!(f, "Install error: {}", msg),
            Self::UninstallError(msg) => write!(f, "Uninstall error: {}", msg),
            Self::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
            Self::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for SkillError {}