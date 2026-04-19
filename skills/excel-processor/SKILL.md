---
name: excel-processor
description: "处理 Excel 文件。触发条件：(1) 用户说'处理Excel'、'读取xlsx'、'编辑Excel' (2) 数据清洗、格式转换 (3) Excel 批量操作"
---

# Excel Processor

处理 Excel 文件的读取、写入、转换。

## Workflow

1. 读取 Excel/CSV 文件
2. 执行数据操作（清洗、转换、合并）
3. 输出结果

## Scripts

读取 Excel:
```bash
python scripts/read_excel.py <file.xlsx> [--sheet <name>]
```

写入 Excel:
```bash
python scripts/write_excel.py --data '<json>' --output <file.xlsx>
```

CSV 转 Excel:
```bash
python scripts/csv_to_excel.py <input.csv> <output.xlsx>
```

合并多个 Excel:
```bash
python scripts/merge_excel.py <file1.xlsx> <file2.xlsx> ... --output <merged.xlsx>
```

## Parameters

| 参数 | 必填 | 说明 |
|------|------|------|
| input | ✓ | 输入文件路径 |
| output | ✓ | 输出文件路径 |
| sheet | | 工作表名称 |
| format | | 输出格式 (xlsx/csv/json) |

## Dependencies

```bash
pip install openpyxl pandas
```