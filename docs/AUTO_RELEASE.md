# 自动发布流程

## 检查逻辑

每 30 分钟心跳检查：

1. **CI 测试状态**：查询 GitHub Actions test.yml 最新运行结果
2. **发布状态**：检查当前版本是否已发布
3. **自动发布**：
   - CI 通过 + 未发布 → 自动打 tag 触发 release.yml
   - CI 失败 → 记录问题，等待修复
   - 已发布 → 跳过

## 使用

```bash
export GITHUB_TOKEN=your_token
scripts/auto_release.sh
```

## 状态文件

`state/release_state.json`:

```json
{
  "last_ci_check": "2026-04-19T10:45:00Z",
  "ci_status": "success",
  "current_version": "0.3.8",
  "released": true
}
```