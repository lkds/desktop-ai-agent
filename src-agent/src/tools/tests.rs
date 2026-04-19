#[cfg(test)]
mod tests {
    use super::super::fileops::*;
    use super::super::registry::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_file_read_tool() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().to_str().unwrap();
        let test_file = dir.path().join("test.txt");
        std::fs::write(&test_file, "Hello World").unwrap();
        
        let tool = FileReadTool;
        let allowed = vec![path.to_string()];
        
        let result = tool.execute(
            serde_json::json!({"path": test_file.to_str().unwrap()}),
            &allowed
        ).await;
        
        assert!(result.is_ok());
        let step = result.unwrap();
        assert_eq!(step.output, "Hello World");
    }

    #[tokio::test]
    async fn test_file_read_permission_denied() {
        let tool = FileReadTool;
        let allowed = vec!["/tmp/safe".to_string()];
        
        let result = tool.execute(
            serde_json::json!({"path": "/etc/passwd"}),
            &allowed
        ).await;
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ToolError::PermissionDenied(_)));
    }

    #[tokio::test]
    async fn test_file_write_tool() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().to_str().unwrap();
        let test_file = dir.path().join("output.txt");
        
        let tool = FileWriteTool;
        let allowed = vec![path.to_string()];
        
        let result = tool.execute(
            serde_json::json!({"path": test_file.to_str().unwrap(), "content": "Test content"}),
            &allowed
        ).await;
        
        assert!(result.is_ok());
        let content = std::fs::read_to_string(&test_file).unwrap();
        assert_eq!(content, "Test content");
    }

    #[tokio::test]
    async fn test_dir_list_tool() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().to_str().unwrap();
        std::fs::write(dir.path().join("file1.txt"), "").unwrap();
        std::fs::create_dir(dir.path().join("subdir")).unwrap();
        
        let tool = DirListTool;
        let allowed = vec![path.to_string()];
        
        let result = tool.execute(
            serde_json::json!({"path": path}),
            &allowed
        ).await;
        
        assert!(result.is_ok());
        let output = result.unwrap().output;
        assert!(output.contains("file1.txt"));
        assert!(output.contains("[DIR]"));
    }

    #[tokio::test]
    async fn test_tool_registry() {
        let registry = init_default_tools();
        
        assert!(registry.get_tool("file_read").is_some());
        assert!(registry.get_tool("file_write").is_some());
        assert!(registry.get_tool("dir_list").is_some());
        assert!(registry.get_tool("nonexistent").is_none());
        
        let tools = registry.list_tools();
        assert!(tools.len() >= 5);
    }

    #[tokio::test]
    async fn test_file_delete_tool() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().to_str().unwrap();
        let test_file = dir.path().join("delete_me.txt");
        std::fs::write(&test_file, "content").unwrap();
        
        let tool = FileDeleteTool;
        let allowed = vec![path.to_string()];
        
        let result = tool.execute(
            serde_json::json!({"path": test_file.to_str().unwrap()}),
            &allowed
        ).await;
        
        assert!(result.is_ok());
        assert!(!test_file.exists());
    }

    #[tokio::test]
    async fn test_file_move_tool() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().to_str().unwrap();
        let src = dir.path().join("source.txt");
        let dst = dir.path().join("dest.txt");
        std::fs::write(&src, "move me").unwrap();
        
        let tool = FileMoveTool;
        let allowed = vec![path.to_string()];
        
        let result = tool.execute(
            serde_json::json!({"source": src.to_str().unwrap(), "destination": dst.to_str().unwrap()}),
            &allowed
        ).await;
        
        assert!(result.is_ok());
        assert!(!src.exists());
        assert!(dst.exists());
        assert_eq!(std::fs::read_to_string(&dst).unwrap(), "move me");
    }
}