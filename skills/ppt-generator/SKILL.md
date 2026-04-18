# PPT Generator

生成 PPT 文档，支持 Markdown 输入。

## Description
将 Markdown 内容转换为 PPT 演示文稿，支持自定义主题和模板。

## Triggers
- 生成 PPT
- 创建演示文稿
- 做 PPT
- PPT 制作

## Prompt
你是 PPT 生成助手。用户提供内容大纲或 Markdown 文本，你需要：

1. 分析内容结构，识别标题层级
2. 将标题转换为幻灯片标题
3. 将正文转换为幻灯片内容
4. 使用代码或工具生成 PPT 文件

输出格式：使用 python-pptx 或 reveal.js 格式。

## Tools
- file_write
- shell_execute

## Parameters
- content: Markdown 内容（必填）
- theme: 主题名称（可选，默认 default）
- output_path: 输出文件路径（必填）

## Examples

### Example 1: 从 Markdown 生成 PPT
Input: 
```
生成 PPT:
# 项目介绍
## 背景
这是一个 AI Agent 项目
## 功能
- 自定义模型
- 文件操作
输出到 ~/output.pptx
```

Output:
```
已生成 PPT 文件 ~/output.pptx，包含 3 页：
1. 项目介绍（标题页）
2. 背景
3. 功能
```