---
name: web-api-client
description: "调用 Web API。触发条件：(1) 用户说'调用API'、'请求接口'、'HTTP请求' (2) REST API 调用 (3) API 测试"
---

# Web API Client

发起 HTTP 请求调用 Web API。

## Workflow

1. 构造请求（URL、Method、Headers、Body）
2. 发送请求
3. 解析响应

## Common Patterns

### GET 请求
```bash
curl -s "https://api.example.com/data" | jq .
```

### POST 请求 (JSON)
```bash
curl -s -X POST "https://api.example.com/users" \
  -H "Content-Type: application/json" \
  -d '{"name": "test", "email": "test@example.com"}' | jq .
```

### 带认证
```bash
curl -s -H "Authorization: Bearer <token>" \
  "https://api.example.com/protected" | jq .
```

## Python Script

```python
#!/usr/bin/env python3
import requests
import json

def api_call(method: str, url: str, headers: dict = None, 
             params: dict = None, data: dict = None) -> dict:
    response = requests.request(
        method=method,
        url=url,
        headers=headers or {},
        params=params,
        json=data
    )
    response.raise_for_status()
    return response.json()
```

## Parameters

| 参数 | 必填 | 说明 |
|------|------|------|
| method | | HTTP 方法 (GET/POST/PUT/DELETE) |
| url | ✓ | 请求 URL |
| headers | | 请求头 |
| params | | URL 参数 |
| data | | 请求体 |

## Error Handling

- 4xx: 客户端错误，检查参数
- 5xx: 服务端错误，稍后重试
- 超时: 设置 `timeout` 参数