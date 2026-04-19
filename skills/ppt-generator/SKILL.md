---
name: ppt-generator
description: "生成 PPT 演示文稿。触发条件：(1) 用户说'生成PPT'、'创建演示文稿'、'做PPT' (2) 需要从 Markdown 转换为 PPT (3) 制作幻灯片"
---

# PPT Generator

将 Markdown 内容转换为 PPT 演示文稿。

## Workflow

1. 解析 Markdown 结构（# 为标题页，## 为幻灯片标题，正文为内容）
2. 使用 python-pptx 生成 .pptx 文件
3. 应用主题和样式

## Usage

```bash
python scripts/generate_ppt.py --input <markdown_file> --output <output.pptx> [--theme default|dark|light]
```

## Parameters

| 参数 | 必填 | 说明 |
|------|------|------|
| input | ✓ | Markdown 文件路径 |
| output | ✓ | 输出 PPT 路径 |
| theme | | 主题名称 (default/dark/light) |

## Example

输入 Markdown:
```markdown
# 项目介绍
## 背景
这是一个 AI Agent 项目

## 功能
- 自定义模型
- 文件操作
```

生成 3 页 PPT：
1. 标题页：项目介绍
2. 内容页：背景
3. 内容页：功能