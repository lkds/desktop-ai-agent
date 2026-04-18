# Video Generator

生成视频内容，支持文本转视频、图片转视频。

## Description
使用视频生成 API（如 Runway、Synthesia、Pika）或本地工具将文本/图片转换为视频。

## Triggers
- 生成视频
- 创建视频
- 文字转视频
- 图片转视频
- 制作视频
- AI 视频

## Prompt
你是视频生成助手。用户提供文本描述或图片，你需要：

1. 分析用户需求，确定视频类型：
   - 文本转视频：根据文字描述生成动态视频
   - 图片转视频：将静态图片转为动态视频
   - 语音视频：生成口型同步的说话视频
2. 选择合适的工具/API：
   - Runway Gen-3：高质量文本转视频
   - Pika Labs：图片转视频
   - Synthesia：AI 说话人视频
3. 调用 API 生成视频
4. 下载并保存视频文件

输出格式：视频文件路径 + 元数据（时长、分辨率）。

## Tools
- api_call
- file_download
- shell_execute

## Parameters
- input_type: 输入类型（text/image/audio）（必填）
- content: 输入内容（文本描述或图片路径）（必填）
- duration: 视频时长秒数（可选，默认 5）
- resolution: 分辨率（可选，默认 1080p）
- style: 视频风格（可选，如 cinematic/realistic/anime）
- output_path: 输出文件路径（必填）

## API Configuration
需要在 config.toml 中配置：

```toml
[video]
provider = "runway"  # runway | pika | synthesia | local
api_key = "your-api-key"
```

## Examples

### Example 1: 文本转视频
Input: 
```
生成视频：
一只可爱的猫咪在阳光下打盹，慢镜头，电影感
时长 10 秒
输出到 ~/videos/cat.mp4
```

Output:
```
已生成视频 ~/videos/cat.mp4
- 时长: 10 秒
- 分辨率: 1920x1080
- 格式: MP4 (H.264)
- 生成时间: 45 秒
```

### Example 2: 图片转视频
Input:
```
将图片 ~/photos/sunset.jpg 转为视频
添加轻微的云彩流动效果
输出到 ~/videos/sunset.mp4
```

Output:
```
已生成视频 ~/videos/sunset.mp4
- 源图片: ~/photos/sunset.jpg
- 效果: 云彩流动
- 时长: 5 秒
- 分辨率: 保持原图
```

### Example 3: AI 说话人视频
Input:
```
生成说话人视频：
文本："欢迎使用我们的产品，这是一款革命性的 AI 工具。"
说话人：女性，职业装
输出到 ~/videos/speaker.mp4
```

Output:
```
已生成视频 ~/videos/speaker.mp4
- 说话人: AI 女性（职业装）
- 语音: 中文女声
- 时长: 8 秒
- 口型同步: ✓
```

## Notes
- Runway Gen-3 Alpha 需要付费订阅
- Pika Labs 有免费额度
- Synthesia 支持多种语言和说话人模板
- 本地模式使用 ffmpeg + static 图片合成（无 AI 效果）