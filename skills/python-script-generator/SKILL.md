---
name: python-script-generator
description: "生成 Python 脚本。触发条件：(1) 用户说'生成Python脚本'、'写Python'、'创建Python程序' (2) 自动化脚本 (3) 数据处理脚本"
---

# Python Script Generator

根据需求生成 Python 脚本。

## Workflow

1. 理解用户需求（数据处理、自动化、API 调用等）
2. 生成完整可运行的 Python 脚本
3. 包含错误处理和依赖说明

## Best Practices

生成的脚本应包含:

1. **Shebang 和编码声明**: `#!/usr/bin/env python3` 和 `# -*- coding: utf-8 -*-`
2. **Type hints**: 使用类型注解
3. **Docstrings**: 函数和模块文档
4. **Argument parsing**: 使用 `argparse`
5. **Error handling**: 使用 `try-except`
6. **Main guard**: `if __name__ == '__main__':`

## Template

```python
#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Script description."""

import argparse
import sys
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(description='Script description')
    parser.add_argument('input', help='Input argument')
    parser.add_argument('--output', help='Output argument')
    args = parser.parse_args()

    # Implementation here
    pass


if __name__ == '__main__':
    main()
```

## Example

用户输入:
```
生成 Python 脚本：批量压缩当前目录下所有 PNG 图片，输出到 compressed/ 目录
```

生成:
- `compress_images.py` - 主脚本
- 依赖: `pip install pillow`
- 用法: `python compress_images.py . --output compressed/`