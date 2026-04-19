---
name: rust-project-generator
description: "生成 Rust 项目。触发条件：(1) 用户说'生成Rust项目'、'创建Rust程序'、'新建Rust' (2) CLI 工具 (3) Web 服务"
---

# Rust Project Generator

根据需求生成 Rust 项目脚手架。

## Workflow

1. 确定项目类型（CLI / Web / Library）
2. 使用 `cargo new` 创建项目
3. 添加必要的依赖和模板代码

## Project Types

### CLI Tool
```bash
cargo new <project_name> --name <project_name>
```

依赖:
```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
anyhow = "1"
tokio = { version = "1", features = ["full"] }
```

### Web Service (Axum)
```bash
cargo new <project_name> --name <project_name>
```

依赖:
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
tower = "0.4"
```

### Library
```bash
cargo new <project_name> --lib --name <project_name>
```

## Best Practices

生成的 Rust 项目应包含:

1. **README.md**: 项目说明和使用方法
2. **.gitignore**: `target/`, `Cargo.lock` (for libraries)
3. **src/main.rs** / **src/lib.rs**: 入口文件
4. **Proper error handling**: 使用 `anyhow` 或 `thiserror`

## Example

用户输入:
```
创建一个 Rust CLI 工具，用于计算文件的 SHA256 哈希值
```

生成:
- `Cargo.toml` - 依赖配置
- `src/main.rs` - 主程序
- `README.md` - 使用说明