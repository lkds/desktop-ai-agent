#!/bin/bash
# 自动检查 CI 状态并发布
# 需要设置环境变量: export GITHUB_TOKEN=your_token

REPO="lkds/desktop-ai-agent"
WORK_DIR="/root/.openclaw/workspace/qoderwork-clone"

if [ -z "$GITHUB_TOKEN" ]; then
  echo "Error: Set GITHUB_TOKEN environment variable"
  exit 1
fi

CURRENT_VERSION=$(cat $WORK_DIR/src-tauri/tauri.conf.json | grep '"version"' | head -1 | cut -d'"' -f4)

# 获取最近的 workflow run 状态
STATUS=$(curl -s "https://api.github.com/repos/$REPO/actions/workflows/test.yml/runs?branch=master&per_page=1" \
  -H "Authorization: token $GITHUB_TOKEN" \
  | jq -r '.workflow_runs[0].status // "unknown"')

CONCLUSION=$(curl -s "https://api.github.com/repos/$REPO/actions/workflows/test.yml/runs?branch=master&per_page=1" \
  -H "Authorization: token $GITHUB_TOKEN" \
  | jq -r '.workflow_runs[0].conclusion // "running"')

echo "CI Status: $STATUS, Conclusion: $CONCLUSION"

# 检查是否已发布当前版本
RELEASED=$(curl -s "https://api.github.com/repos/$REPO/releases/tags/v$CURRENT_VERSION" \
  -H "Authorization: token $GITHUB_TOKEN" \
  | jq -r '.tag_name // "not_found"')

echo "Release v$CURRENT_VERSION: $RELEASED"

if [ "$STATUS" = "completed" ] && [ "$CONCLUSION" = "success" ] && [ "$RELEASED" = "null" ] || [ "$RELEASED" = "not_found" ]; then
  echo "✅ CI passed, creating release..."
  
  # 更新版本号
  NEW_VERSION=$(echo $CURRENT_VERSION | awk -F. '{$3+=1; print $1"."$2"."$3}')
  sed -i "s/\"version\": \"$CURRENT_VERSION\"/\"version\": \"$NEW_VERSION\"/" $WORK_DIR/src-tauri/tauri.conf.json
  
  cd $WORK_DIR
  git add .
  git commit -m "v$NEW_VERSION - 自动发布"
  git push origin master
  git tag v$NEW_VERSION
  git push origin v$NEW_VERSION
  
  echo "✅ Released v$NEW_VERSION"
else
  echo "⏳ Waiting for CI or already released"
fi