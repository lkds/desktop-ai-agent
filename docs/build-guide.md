# 打包指南

## 系统要求

### macOS
```bash
# 无需额外依赖，直接打包
cargo tauri build
```

### Windows
```powershell
# 安装 Visual Studio Build Tools
cargo tauri build
```

### Linux (Ubuntu/Debian)
```bash
# 安装依赖
sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev libsoup-3.0-dev

# 打包
cargo tauri build
```

### Linux (Fedora)
```bash
sudo dnf install gtk3-devel webkit2gtk4.1-devel libappindicator-gtk3-devel librsvg2-devel openssl-devel libsoup3-devel
cargo tauri build
```

## 已验证状态

- ✅ Rust 后端编译通过 (Linux x86_64)
- ✅ 前端构建成功
- ❌ Tauri 打包 (需要目标系统)

## 推荐打包方式

**在 macOS/Windows 上打包**:
1. 克隆项目到目标系统
2. `cd src-ui && npm install && npm run build`
3. `cargo tauri build`

产物位置: `src-tauri/target/release/bundle/`

## CI/CD 打包

可以配置 GitHub Actions 自动打包多平台：

```yaml
# .github/workflows/release.yml
name: Release
on:
  push:
    tags: ['v*']

jobs:
  build:
    strategy:
      matrix:
        platform: [macos-latest, windows-latest, ubuntu-22.04]
    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
      - run: cd src-ui && npm install && npm run build
      - uses: dtolnay/rust-toolchain@stable
      - uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tagName: ${{ github.ref_name }}
          releaseName: 'Desktop AI Agent ${{ github.ref_name }}'
```