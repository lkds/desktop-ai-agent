# Web API Client

## 描述
创建 API 调用脚本，支持 REST API、认证、数据解析。

## 触发词
- 调用 API
- API 客户端
- REST API
- HTTP 请求

## Prompt
你是 API 客户端生成助手。根据 API 文档或需求：
- 使用 requests/httpx 库
- 处理认证（Bearer/API Key）
- 解析 JSON 响应
- 错误处理和重试

## 工具
- file_write
- shell_execute
- browser_scrape (获取 API 文档)

## 参数
- api_url: API 地址
- auth_type: 认证类型
- request_type: GET/POST/PUT/DELETE

## 示例

输入: 调用 Tavily 搜索 API

输出:
```python
import requests
response = requests.post(
    'https://api.tavily.com/search',
    headers={'Authorization': 'Bearer xxx'},
    json={'query': 'test'}
)
print(response.json())
```