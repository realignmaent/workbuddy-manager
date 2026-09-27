# WorkBuddy Manager 桌面客户端 (Tauri 2.0)

本项目已扩展支持原生桌面客户端模式（基于 Tauri 2.0 + Next.js + Python FastAPI 网关）。

## 特性

* **系统原生体验**：独立窗口渲染，去掉浏览器地址栏与杂乱标签页。
* **托盘常驻与静默运行**：点击窗口右上角关闭按钮自动最小化至 Windows 系统托盘，不会中断下游（Cursor / VS Code / Claude Code 等）的 API 请求。
* **托盘快捷操作**：
  * 打开控制台窗口
  * 一键复制本地 API 接口地址（`http://127.0.0.1:7864/v1`）
  * 在默认外部浏览器中打开
  * 完全退出
* **双模部署兼备**：客户端代码集中在 `src-tauri/` 目录中，与上游核心业务代码物理隔离，随时可通过 `git pull upstream main` 无缝同步原作者更新。

---

## 目录结构

```text
workbuddy-manager/
├── src-tauri/                       # 桌面客户端工程目录
│   ├── Cargo.toml                   # Rust 依赖配置
│   ├── tauri.conf.json              # Tauri 2.0 配置文件
│   ├── splashscreen.html            # 优雅的启动等待界面
│   ├── icons/                       # 客户端各尺寸高清图标
│   └── src/
│       ├── main.rs                  # 桌面端程序入口
│       └── lib.rs                   # 托盘、窗口事件与进程守护管理
└── .github/workflows/
    └── build-desktop.yml            # GitHub Actions 自动化云端打包工作流
```

---

## 构建与打包

### 方式一：GitHub Actions 云端自动打包（推荐，无需配置本地环境）

1. 将代码推送到你的 GitHub 仓库（`feature/desktop-client` 分支或 `main` 分支）：
   ```powershell
   git push origin feature/desktop-client
   ```
2. 打开 GitHub 仓库页面，点击顶部的 **Actions** 标签栏。
3. 找到 **Build Desktop Client** 工作流。
4. 构建完成后，在工作流详情页底部的 **Artifacts** 中即可直接下载 `WorkBuddy-Manager-Windows-Setup` 安装包。

### 方式二：本地手动构建（适合本地已有 Rust 环境）

前提条件：本地已安装 Node.js 18+、Python 3.11+ 以及 Rust 工具链。

```powershell
# 1. 安装前端依赖并导出静态资源
cd web
npm install
npm run build:export
cd ..

# 2. 安装 Tauri CLI
cargo install tauri-cli --version "^2.0.0" --locked

# 3. 编译并打包桌面安装程序
cargo tauri build
```
编译产物将生成在 `src-tauri/target/release/bundle/nsis/` 目录下。
