---
name: web-scrape
description: "抓取网页内容。触发条件：(1) 用户说'抓取网页'、'爬取网站'、'获取页面内容' (2) 提取网页数据 (3) 网页内容解析"
---

# Web Scrape

抓取网页内容，提取数据。

## Workflow

1. 发送 HTTP 请求获取网页
2. 解析 HTML
3. 提取目标数据

## Methods

### curl + jq (简单场景)
```bash
curl -s "https://api.example.com/data" | jq '.items[]'
```

### Python + BeautifulSoup
```python
from bs4 import BeautifulSoup
import requests

url = "https://example.com"
response = requests.get(url)
soup = BeautifulSoup(response.text, 'html.parser')

# 提取标题
title = soup.find('h1').text

# 提取所有链接
links = [a['href'] for a in soup.find_all('a', href=True)]

# CSS 选择器
items = soup.select('.item-list .item')
```

### Python + Playwright (动态页面)
```python
from playwright.sync_api import sync_playwright

with sync_playwright() as p:
    browser = p.chromium.launch()
    page = browser.new_page()
    page.goto("https://example.com")
    
    # 等待元素加载
    page.wait_for_selector(".content")
    
    # 提取文本
    text = page.locator(".content").text_content()
    
    browser.close()
```

## Parameters

| 参数 | 必填 | 说明 |
|------|------|------|
| url | ✓ | 目标 URL |
| selector | | CSS 选择器 |
| wait_for | | 等待元素加载 |
| format | | 输出格式 (text/json/markdown) |

## Best Practices

1. **尊重 robots.txt**
2. **添加 User-Agent**
3. **控制请求频率**
4. **处理错误和重试**