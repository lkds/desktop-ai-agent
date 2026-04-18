# Desktop AI Agent

**开源替代 QoderWork - 完全自定义模型，开放架构**

[![GitHub](https://img.shields.io/badge/GitHub-lkds/desktop--ai--agent-blue)](https://github.com/lkds/desktop-ai-agent)

## 核心特性

- ✅ **自定义模型**：任意 API Key + 模型厂商（OpenAI/Claude/Ollama/自定义 endpoint）
- ✅ **本地执行**：数据不出本地，隐私安全
- ✅ **Skills 系统**：可扩展技能模块
- ✅ **文件操作**：完整的本地文件读写能力
- ✅ **任务追踪**：实时进度，暂停/恢复/取消
- ✅ **权限控制**：精细化文件访问权限

## 快速开始

### 环境要求

- Rust 1.70+
- Node.js 18+
- pnpm 或 npm

### 1. 克隆项目

```bash
git clone https://github.com/lkds/desktop-ai-agent.git
cd desktop-ai-agent
```

### 2. 安装前端依赖

```bash
cd src-ui
npm install
```

### 3. 开发模式运行

```bash
# 方式一：Tauri 开发（需要 Rust 环境）
cargo tauri dev

# 方式二：CLI 测试（纯 Rust）
cd src-agent
cargo run --bin agent-cli
```

### 4. 打包发布

```bash
cargo tauri build
```

产物在 `src-tauri/target/release/bundle/` 目录。

## 配置模型

### 配置文件

编辑 `~/.desktop-agent/config.json`：

```json
{
  "provider": {
    "kind": "openai",
    "api_key": "sk-xxx",
    "base_url": "https://api.openai.com/v1",
    "model": "gpt-4"
  },
  "allowed_paths": ["~/Documents", "~/Desktop"],
  "skills_dir": "~/.desktop-agent/skills"
}
```

### 支持的 Provider

| Provider | kind | API Key | Base URL |
|----------|------|---------|----------|
| OpenAI | `openai` | 必需 | 可选（默认官方） |
| Claude | `claude` | 必需 | 可选 |
| Ollama | `ollama` | 不需要 | 可选（默认 localhost:11434） |
| 自定义 | `custom` | 可选 | 必需 |

### 自定义 endpoint 示例

```json
{
  "provider": {
    "kind": "custom",
    "api_key": "your-key",
    "base_url": "https://your-api.com/v1",
    "model": "your-model"
  }
}
```

支持任何 OpenAI-compatible API。

## Skills 使用

### 已内置 Skills

- **folder-organizer**: 自动整理文件夹，按类型/时间分类
- **pdf-processor**: PDF 提取文本、合并、拆分

### 自定义 Skill

在 `skills/<name>/SKILL.md` 创建：

```markdown
# Skill Name

## Description
技能描述

## Triggers
- 触发关键词1
- 触发关键词2

## Prompt
执行 Prompt 模板

## Tools
- dir_list
- file_move
- file_write

## Parameters
- path: 要操作的路径

## Examples
### Example 1
Input: ...
Output: ...
```

### 安装 Skill

将 Skill 目录放到 `~/.desktop-agent/skills/`，或在前端 Skills 页面安装。

## 项目结构

```
desktop-ai-agent/
├── src-agent/         # Rust 后端
│   ├── src/
│   │   ├── providers/ # LLM Provider 抽象层
│   │   ├── agent/     # Agent 执行引擎
│   │   ├── tools/     # 工具实现
│   │   ├── skills/    # Skills 系统
│   │   ├── config/    # 配置管理
│   │   └── ipc/       # Tauri IPC
│   └── Cargo.toml
│
├── src-tauri/         # Tauri 入口
│   ├── src/main.rs
│   └── tauri.conf.json
│
├── src-ui/            # React 前端
│   ├── src/
│   │   ├── pages/     # 页面组件
│   │   ├── stores/    # Zustand 状态
│   │   └── App.tsx
│   └── package.json
│
├── skills/            # Skills 定义
│   ├── folder-organizer/
│   └── pdf-processor/
│
└── README.md
```

## 核心模块说明

### Agent Executor

任务执行引擎核心：

```
用户输入 → LLM 规划 → 步骤拆解 → 工具调用 → 结果交付
```

支持：
- 任务暂停/恢复/取消
- 实时步骤进度追踪
- 错误处理和重试

### Provider Manager

统一模型调用接口：

```rust
pub trait Provider {
    async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse>;
    async fn generate_stream(&self, request) -> Result<Stream>;
    async fn health_check(&self) -> Result<bool>;
}
```

### Tools Registry

已实现工具：

| Tool | 功能 | 风险等级 |
|------|------|----------|
| file_read | 读文件 | Low |
| file_write | 写文件 | Medium |
| dir_list | 列目录 | Low |
| file_move | 移动文件 | Medium |
| file_delete | 删除文件 | High |

风险等级高的操作需要用户确认。

## API 调用示例

### 执行任务

```typescript
import { invoke } from '@tauri-apps/api'

const task = await invoke('execute_task', {
  description: '整理 ~/Downloads 目录，按文件类型分类'
})

console.log(task.status) // 'completed'
console.log(task.steps)  // 步骤列表
console.log(task.result) // 结果
```

### 配置模型

```typescript
await invoke('save_config', {
  config: {
    provider: {
      kind: 'openai',
      api_key: 'sk-xxx',
      model: 'gpt-4'
    },
    allowed_paths: ['~/Documents']
  }
})
```

## 常见问题

### Q: 编译失败？

检查 Rust 版本：
```bash
rustc --version  # 需要 1.70+
```

更新 Rust：
```bash
rustup update
```

### Q: 前端启动失败？

```bash
cd src-ui
rm -rf node_modules package-lock.json
npm install
```

### Q: Ollama 连接失败？

确保 Ollama 运行：
```bash
ollama serve
```

检查端口：`http://localhost:11434`

### Q: 如何使用本地模型？

配置 Ollama：
```json
{
  "provider": {
    "kind": "ollama",
    "model": "llama3"
  }
}
```

## 与 QoderWork 对比

| 特性 | QoderWork | Desktop AI Agent |
|------|-----------|------------------|
| 自定义模型 | ❌ 固定厂商 | ✅ 完全自定义 |
| 开源 | ❌ | ✅ MIT |
| 本地部署 | ❌ | ✅ |
| Skills 扩展 | ✅ | ✅ |
| MCP 集成 | ✅ | 🚧 开发中 |
| 价格 | $19/月 | 免费 |

## 开发路线

### 已完成 ✅

- Provider 抽象层
- Agent 执行引擎
- 文件操作工具
- Skills 系统
- 前端 UI
- Tauri 集成

### 开发中 🚧

- 浏览器自动化工具
- MCP 集成
- Shell 命令工具
- 更多 Skills

### 计划中 📋

- 多智能体并行
- 视频生成 Skill
- 定时任务
- 跨会话记忆

## License

MIT

## 贡献

欢迎提交 PR！

```bash
git clone https://github.com/lkds/desktop-ai-agent.git
cd desktop-ai-agent
# 修改代码
git commit -am "Your changes"
git push
```

---

**GitHub: https://github.com/lkds/desktop-ai-agent**