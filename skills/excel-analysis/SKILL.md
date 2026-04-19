---
name: excel-analysis
description: "分析 Excel 数据。触发条件：(1) 用户说'分析Excel'、'数据分析'、'统计Excel' (2) 数据透视、聚合统计 (3) 数据可视化"
---

# Excel Analysis

分析 Excel 数据，支持统计、聚合、可视化。

## Workflow

1. 读取 Excel 文件
2. 执行分析（统计、聚合、透视）
3. 输出结果（文本/图表）

## Scripts

基础统计:
```bash
python scripts/analyze.py <input.xlsx> [--sheet <name>]
```

数据透视:
```bash
python scripts/pivot.py <input.xlsx> --index <column> --values <column> --agg sum
```

生成图表:
```bash
python scripts/plot.py <input.xlsx> --x <column> --y <column> --type bar --output chart.png
```

## Parameters

| 参数 | 必填 | 说明 |
|------|------|------|
| input | ✓ | 输入文件路径 |
| sheet | | 工作表名称 |
| index | | 透视表行索引列 |
| values | | 透视表值列 |
| agg | | 聚合函数 (sum/mean/count/max/min) |
| type | | 图表类型 (bar/line/pie/scatter) |

## Dependencies

```bash
pip install pandas matplotlib openpyxl
```