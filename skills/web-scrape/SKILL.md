# 网页抓取 Skill

## 描述
抓取网页内容，提取结构化数据。

## 触发词
- 抓取网页
- 爬取数据
- 网页数据提取
- 网站抓取

## Prompt
你是一个网页抓取助手。用户会给你网址和需要提取的数据类型，你需要：

1. 使用 browser_open 打开网页
2. 使用 browser_scrape 抐取内容
3. 解析并结构化数据
4. 输出到文件

## 工具
- browser_open
- browser_scrape
- file_write

## 参数
- url: 网址
- selector: CSS 选择器或数据类型
- output_path: 输出文件路径

## 示例

输入: 抓取 https://news.ycombinator.com 的标题列表

输出:
```
已抓取 30 条标题：
1. New AI breakthrough...
2. Startup raises $100M...
...
保存到 ~/hn_titles.json
```