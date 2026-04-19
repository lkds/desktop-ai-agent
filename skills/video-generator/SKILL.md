---
name: video-generator
description: "生成视频内容。触发条件：(1) 用户说'生成视频'、'创建视频'、'制作视频' (2) 文本转视频 (3) 图片转视频 (4) AI 视频生成"
---

# Video Generator

生成视频内容，支持文本转视频、图片转视频。

## Workflow

1. 分析输入类型（文本描述 / 图片路径）
2. 选择工具：
   - **本地模式**: ffmpeg + 静态图片/文字合成
   - **API 模式**: Runway/Pika/Synthesia（需配置 API Key）
3. 生成视频文件

## Local Mode (ffmpeg)

文本转视频（文字滚动）:
```bash
bash scripts/text_to_video.sh "<text>" <output.mp4> [duration]
```

图片转视频（静态图片转为视频）:
```bash
bash scripts/image_to_video.sh <input.jpg> <output.mp4> [duration]
```

## API Mode

在 `~/.config/desktop-ai-agent/config.toml` 配置:

```toml
[video]
provider = "runway"  # runway | pika | synthesia
api_key = "your-api-key"
```

## Parameters

| 参数 | 必填 | 说明 |
|------|------|------|
| input | ✓ | 输入内容（文本或图片路径）|
| output | ✓ | 输出视频路径 |
| duration | | 时长秒数 (默认 5) |
| resolution | | 分辨率 (默认 1080p) |
| style | | 风格 (cinematic/realistic/anime) |

## Examples

文本转视频:
```
生成视频：一只猫咪在阳光下打盹，时长 10 秒，输出到 ~/videos/cat.mp4
```

图片转视频:
```
将 ~/photos/sunset.jpg 转为 5 秒视频，输出到 ~/videos/sunset.mp4
```