# Desktop AI Agent - 技术架构文档

## 产品定位

类似 QoderWork 的桌面执行 Agent，核心差异化：
- **自定义模型**：支持任意 API key 和模型厂商
- **开放架构**：Skills/MCP 完全可扩展
- **本地优先**：数据不出本地，隐私安全

---

## 系统架构

```
┌─────────────────────────────────────────────────────────────┐
│                    Desktop Application                       │
│                     (Tauri 2.0)                              │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────────┐    ┌─────────────────────────────┐    │
│  │   React UI      │    │    Rust Backend             │    │
│  │   (TypeScript)  │◄──►│    (Agent Engine)           │    │
│  │                 │    │                             │    │
│  │  ┌───────────┐  │    │  ┌─────────────────────┐   │    │
│  │  │Chat Page  │  │    │  │ Agent Executor      │   │    │
│  │  │Settings   │  │    │  │ - Task Planning     │   │    │
│  │  │Skills     │  │    │  │ - Step Execution    │   │    │
│  │  │History    │  │    │  │ - State Tracking    │   │    │
│  │  └───────────┘  │    │  └─────────────────────┘   │    │
│  │                 │    │                             │    │
│  │  Zustand Store  │    │  ┌─────────────────────┐   │    │
│  │  Tauri IPC      │    │  │ Provider Manager    │   │    │
│  │                 │    │  │ - OpenAI            │   │    │
│  │                 │    │  │ - Claude            │   │    │
│  │                 │    │  │ - Ollama            │   │    │
│  │                 │    │  │ - Custom            │   │    │
│  │                 │    │  └─────────────────────┘   │    │
│  │                 │    │                             │    │
│  │                 │    │  ┌─────────────────────┐   │    │
│  │                 │    │  │ Tool Registry       │   │    │
│  │                 │    │  │ - FileOps           │   │    │
│  │                 │    │  │ - Browser           │   │    │
│  │                 │    │  │ - Shell             │   │    │
│  │                 │    │  │ - API Caller        │   │    │
│  │                 │    │  └─────────────────────┘   │    │
│  │                 │    │                             │    │
│  │                 │    │  ┌─────────────────────┐   │    │
│  │                 │    │  │ Skills Manager      │   │    │
│  │                 │    │  │ - Load Skills       │   │    │
│  │                 │    │  │ - Execute Skills    │   │    │
│  │                 │    │  │ - MCP Integration   │   │    │
│  │                 │    │  └─────────────────────┘   │    │
│  │                 │    │                             │    │
│  └─────────────────┘    └─────────────────────────────┘    │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│                     Local Storage                            │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ Config (JSON) | Task History | Skills Dir | Cache    │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

---

## 核心模块划分

### 1. Agent Executor（任务执行引擎）
- **Task Planner**：LLM 将任务拆解为步骤
- **Step Executor**：按步骤调用工具执行
- **State Manager**：追踪执行状态，支持暂停/恢复

### 2. Provider Manager（模型抽象层）
- 统一接口调用不同 LLM
- 配置存储：API key, base URL, model name
- 流式响应处理

### 3. Tool Registry（工具注册中心）
- FileOps：文件读写、整理、搜索
- Browser：自动化浏览器操作
- Shell：执行命令行
- API：调用外部 API

### 4. Skills Manager（技能系统）
- Skills 定义：SKILL.md + 工具配置
- Skills 执行：封装复杂工作流
- MCP 集成：连接外部 MCP server

---

## 技术选型理由

| 技术 | 选择 | 理由 |
|------|------|------|
| 桌面框架 | Tauri 2.0 | 轻量（~3MB）、安全、Rust 后端性能好 |
| 后端语言 | Rust | 高性能、内存安全、并发友好 |
| 前端 | React + TypeScript | 成熟、类型安全、组件化 |
| 状态管理 | Zustand | 轻量、简单、支持持久化 |
| IPC | Tauri invoke | 前后端通信，支持 async |
| 配置存储 | JSON + SQLite | JSON 简单配置，SQLite 存历史 |

---

## MVP 第一阶段（最小可行方案）

### 目标
- 基础对话 + 任务执行
- 自定义模型配置
- 文件读写操作
- 单个 Skill 示例

### 功能清单
1. 模型配置页面（API key + base URL + model）
2. 主对话页面（输入任务 → 执行 → 展示结果）
3. 文件操作工具（读写指定文件夹）
4. 一个示例 Skill（如：整理文件夹）

### 技术实现
- Tauri 基础框架搭建
- Rust Agent Executor 核心逻辑
- React 基础页面
- Provider 抽象层（先支持 OpenAI-compatible）

---

## 文件结构

```
qoderwork-clone/
├── src-tauri/               # Tauri Rust 后端
│   ├── src/
│   │   ├── main.rs          # Tauri 入口
│   │   ├── agent/           # Agent 执行引擎
│   │   │   ├── mod.rs
│   │   │   ├── task.rs      # Task/Step 结构
│   │   │   ├── executor.rs  # 执行器
│   │   │   └── planner.rs   # 任务规划
│   │   ├── providers/       # LLM Provider
│   │   │   ├── mod.rs
│   │   │   ├── trait.rs     # Provider trait
│   │   │   ├── openai.rs
│   │   │   ├── claude.rs
│   │   │   ├── ollama.rs
│   │   │   └── custom.rs
│   │   ├── tools/           # 工具实现
│   │   │   ├── mod.rs
│   │   │   ├── fileops.rs
│   │   │   ├── browser.rs
│   │   │   └── shell.rs
│   │   ├── skills/          # Skills 系统
│   │   │   ├── mod.rs
│   │   │   ├── loader.rs
│   │   │   └── executor.rs
│   │   ├── config/          # 配置管理
│   │   │   └── mod.rs
│   │   └── ipc/             # Tauri IPC handlers
│   │       └── mod.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── src-ui/                  # React 前端
│   ├── src/
│   │   ├── components/      # 通用组件
│   │   │   ├── ChatInput.tsx
│   │   │   ├── StepProgress.tsx
│   │   │   ├── ModelSelector.tsx
│   │   │   └── FilePicker.tsx
│   │   ├── pages/           # 页面
│   │   │   ├── ChatPage.tsx
│   │   │   ├── SettingsPage.tsx
│   │   │   ├── SkillsPage.tsx
│   │   │   └── HistoryPage.tsx
│   │   ├── stores/          # Zustand 状态
│   │   │   ├── chatStore.ts
│   │   │   ├── configStore.ts
│   │   │   └── skillsStore.ts
│   │   ├── hooks/           # React hooks
│   │   │   ├── useAgent.ts
│   │   │   ├── useIpc.ts
│   │   │   └── useSkills.ts
│   │   ├── ipc/             # Tauri IPC 调用
│   │   │   └── agent.ts
│   │   ├── App.tsx
│   │   └── main.tsx
│   ├── package.json
│   └── vite.config.ts
│   └── index.html
│
├── skills/                  # Skills 定义目录
│   ├── folder-organizer/
│   │   └── SKILL.md
│   ├── pdf-processor/
│   │   └── SKILL.md
│   └── ...
│
├── docs/
│   ├── architecture.md      # 本文档
│   ├── skills-guide.md      # Skills 开发指南
│   └── mcp-integration.md   # MCP 集成指南
│
└── README.md
```

---

## 下一步

1. 初始化 Tauri 项目
2. 实现 Provider 抽象层
3. 实现 Agent Executor 核心
4. 实现基础 UI
5. 实现第一个 Skill

预计 MVP 完成时间：**1-2 周**