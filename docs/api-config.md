# API 配置说明

## 阿里云百炼两种 API

| API 类型 | Key 格式 | Base URL | 说明 |
|----------|----------|----------|------|
| **通用 API** | `sk-xxxxx` | `https://dashscope.aliyuncs.com/compatible-mode/v1` | ✅ 可用于本项目 |
| **Coding Plan** | `sk-sp-xxxxx` | `https://coding.dashscope.aliyuncs.com/v1` | ❌ 专属服务，仅限 Coding Agent 产品 |

---

## 配置示例

### 百炼通用 API（推荐）

```json
{
  "provider": {
    "kind": "dashscope",
    "api_key": "sk-你的通用key",
    "model": "qwen-plus"
  },
  "allowed_paths": ["~/Documents"]
}
```

### 或使用 OpenAI-compatible 格式

```json
{
  "provider": {
    "kind": "custom",
    "api_key": "sk-你的通用key",
    "base_url": "https://dashscope.aliyuncs.com/compatible-mode/v1",
    "model": "qwen-plus"
  }
}
```

---

## 如何获取百炼通用 API Key

1. 登录阿里云百炼控制台：https://bailian.console.aliyun.com
2. 选择地域（华北2-北京）
3. 进入 API Key 管理
4. 创建 API Key（格式 `sk-xxxxx`）

---

## 支持的模型

- `qwen-plus`（推荐）
- `qwen-turbo`
- `qwen-max`
- `qwen-long`

---

## 其他 Provider

### OpenAI
```json
{
  "provider": {
    "kind": "openai",
    "api_key": "sk-xxx",
    "model": "gpt-4"
  }
}
```

### Claude
```json
{
  "provider": {
    "kind": "claude",
    "api_key": "sk-ant-xxx",
    "model": "claude-3-opus"
  }
}
```

### Ollama（本地）
```json
{
  "provider": {
    "kind": "ollama",
    "model": "llama3",
    "base_url": "http://localhost:11434"
  }
}
```