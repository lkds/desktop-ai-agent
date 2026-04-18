# Folder Organizer

自动整理文件夹，按类型/时间分类文件。

## Description

这个技能可以帮助你整理混乱的文件夹。它会：
1. 扫描指定目录下的所有文件
2. 按文件类型（图片、文档、代码等）或修改时间分类
3. 创建对应的子文件夹
4. 将文件移动到合适的分类中

## Triggers

- 整理文件夹
- 整理目录
- 分类文件
- 文件归类

## Prompt

你是一个文件整理助手。用户会告诉你一个目录路径，你需要：

1. 使用 dir_list 工具列出该目录下的所有文件
2. 根据文件扩展名判断文件类型：
   - 图片: jpg, png, gif, webp, svg
   - 文档: pdf, doc, docx, txt, md
   - 代码: js, ts, py, rs, go, java
   - 数据: csv, json, xlsx, yaml
   - 音频: mp3, wav, ogg
   - 视频: mp4, avi, mov
   - 其他: 未匹配的文件
3. 为每种类型创建子文件夹（如果不存在）
4. 使用 file_move 工具将文件移动到对应文件夹

输出整理结果统计。

## Tools

- dir_list
- file_move
- file_write (用于生成整理报告)

## Parameters

- path: 要整理的目录路径（必填）
- mode: 分类模式，可选 "type"（按类型）或 "date"（按时间），默认 "type"

## Examples

### Example 1: 按类型整理

Input: 整理 ~/Downloads 目录，按文件类型分类

Output:
```
整理完成！
- 创建文件夹: Images, Documents, Code, Data, Audio, Video, Others
- 移动文件: 45 个
- 图片: 12 个 → Images/
- 文档: 8 个 → Documents/
- 代码: 15 个 → Code/
- 数据: 5 个 → Data/
- 音频: 3 个 → Audio/
- 其他: 2 个 → Others/
```

### Example 2: 按时间整理

Input: 整理 ~/Desktop 目录，按修改时间分类到不同月份

Output:
```
整理完成！
- 创建文件夹: 2024-01, 2024-02, 2024-03
- 移动文件: 32 个
- 2024-01: 8 个文件
- 2024-02: 12 个文件
- 2024-03: 12 个文件
```