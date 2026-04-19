---
name: folder-organizer
description: "整理文件夹。触发条件：(1) 用户说'整理文件夹'、'整理目录'、'清理文件' (2) 按类型/日期分类文件 (3) 批量重命名"
---

# Folder Organizer

整理文件夹，按类型或日期分类文件。

## Workflow

1. 扫描目标目录
2. 按规则分类（类型/日期/自定义）
3. 移动或重命名文件

## Scripts

按类型分类:
```bash
python scripts/organize_by_type.py <directory> [--dry-run]
```

按日期分类:
```bash
python scripts/organize_by_date.py <directory> [--dry-run]
```

批量重命名:
```bash
python scripts/batch_rename.py <directory> --pattern "<pattern>" --replacement "<replacement>"
```

查找重复文件:
```bash
python scripts/find_duplicates.py <directory>
```

## Parameters

| 参数 | 必填 | 说明 |
|------|------|------|
| directory | ✓ | 目标目录 |
| dry-run | | 预览模式，不实际执行 |
| pattern | | 重命名正则模式 |
| replacement | | 替换字符串 |

## Example

按类型分类:
```
整理 ~/Downloads 文件夹

结果:
~/Downloads/
├── Images/      # jpg, png, gif
├── Documents/  # pdf, doc, txt
├── Archives/    # zip, tar, gz
├── Videos/      # mp4, mkv
└── Music/       # mp3, wav
```