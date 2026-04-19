---
name: pdf-processor
description: "处理 PDF 文件。触发条件：(1) 用户说'处理PDF'、'提取PDF文字'、'合并PDF' (2) PDF 文本提取 (3) PDF 合并/分割"
---

# PDF Processor

处理 PDF 文件的提取、合并、分割。

## Workflow

1. 读取 PDF 文件
2. 执行操作（提取文本、合并、分割）
3. 输出结果

## Scripts

提取文本:
```bash
python scripts/extract_text.py <input.pdf> [--output <output.txt>]
```

合并 PDF:
```bash
python scripts/merge_pdf.py <file1.pdf> <file2.pdf> ... --output <merged.pdf>
```

分割 PDF:
```bash
python scripts/split_pdf.py <input.pdf> --pages 1-5,10-15 --output <output_dir>
```

旋转 PDF:
```bash
python scripts/rotate_pdf.py <input.pdf> <output.pdf> --degrees 90
```

## Parameters

| 参数 | 必填 | 说明 |
|------|------|------|
| input | ✓ | 输入文件路径 |
| output | ✓ | 输出文件路径 |
| pages | | 页码范围 (如 1-5,10-15) |
| degrees | | 旋转角度 (90/180/270) |

## Dependencies

```bash
pip install pymupdf pdfplumber
```