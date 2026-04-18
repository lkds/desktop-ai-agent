# Desktop AI Agent - 开源替代 QoderWork

**核心差异化：完全自定义模型，开放架构**

## 特性

- ✅ **自定义模型**：支持任意 API Key 和模型厂商（OpenAI/Claude/Ollama/自定义）
- ✅ **本地执行**：数据不出本地，隐私安全
- ✅ **Skills 系统**：可扩展的技能模块，封装复杂工作流
- ✅ **MCP 集成**：支持 Model Context Protocol
- ✅ **浏览器自动化**：内置浏览器操作能力
- ✅ **文件操作**：完整的本地文件读写能力
- ✅ **多智能体并行**：同时执行多个任务

## 技术栈

- **桌面框架**: Tauri 2.0（Rust + Web）
- **后端**: Rust（高性能、安全）
- **前端**: React + TypeScript + Zustand
- **模型抽象层**: 统一的 Provider 接口

## 快速开始

### 1. 前端构建

```bash
cd src-ui
npm install
npm run dev
```

### 2. 后端编译

```bash
cd src-agent
cargo build
```

### 3. Tauri 打包

```bash
cargo tauri build
```

## 项目结构

```
qoderwork-clone/
├── docs/               # 文档
│   └── architecture.md
├── src-agent/          # Rust 后端
│   ├── src/
│   │   ├── agent/      # Agent 执行引擎
│   │   ├── providers/  # LLM Provider 抽象层
│   │   ├── tools/      # 工具实现
│   │   ├── skills/     # Skills 系统
│   │   └── config/     # 配置管理
│   └── Cargo.toml
├── src-ui/             # React 前端
│   ├── src/
│   │   ├── pages/      # 页面组件
│   │   ├── stores/     # Zustand 状态
│   │   └── components/ # UI 组件
│   └── package.json
├── skills/             # Skills 定义
│   ├── folder-organizer/
│   ├── pdf-processor/
│   └── ...
└── README.md
```

## 核心模块

### 1. Agent Executor

任务执行引擎：
- 自然语言 → 任务拆解 → 步骤执行 → 结果交付
- 支持暂停/恢复/取消
- 实时进度追踪

### 2. Provider Manager

模型抽象层：
- 统一接口调用不同 LLM
- 支持流式响应
- 配置持久化

### 3. Tools Registry

工具注册中心：
- FileOps: 文件读写、移动、删除
- Browser: 浏览器自动化
- Shell: 命令执行
- API: 外部接口调用

### 4. Skills Manager

技能系统：
- SKILL.md 定义技能
- 触发词自动匹配
- MCP 集成扩展

## 配置模型

在设置页面配置：
- Provider 类型（OpenAI/Claude/Ollama/自定义）
- API Key
- Base URL（自定义 endpoint）
- 模型名称

示例配置：
```json
{
  "provider": {
    "kind": "openai",
    "api_key": "sk-xxx",
    "base_url": "https://api.openai.com/v1",
    "model": "gpt-4"
  },
  "allowed_paths": ["~/Documents", "~/Desktop"]
}
```

## Skills 开发

创建 `skills/<skill-name>/SKILL.md`：

```markdown
# Skill Name

## Description
技能描述

## Triggers
- 触发关键词

## Prompt
执行 Prompt 模板

## Tools
- 使用的工具列表

## Parameters
参数定义

## Examples
示例输入输出
```

## 路线图

### Phase 1 (MVP)
- 基础对话 + 任务执行
- 自定义模型配置
- 文件操作工具
- 示例 Skills

### Phase 2
- 浏览器自动化
- MCP 集成
- 多智能体并行
- Skills 市场

### Phase 3
- 视频生成
- 定时任务
- 跨会话记忆
- Windows/Linux 支持

## License

MIT

## 贡献

欢迎提交 PR 和 Skills！