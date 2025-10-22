# Changelog

所有重要更改都会记录在此文件中。

本项目遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

---

## [1.1.0] - 2025-10-22

### ✨ Added（新增）

- **阴阳怪气提醒系统**
  - 弹窗文案改为"😊 你的手在干嘛？😊"
  - 温和讽刺风格，提升趣味性
  - 文件：`src/alert.html`, `alert_config.json`

- **弹窗抖动反馈**
  - 添加 CSS 抖动动画（左右晃动 0.5 秒）
  - 每次触发都会抖动，即使弹窗已显示
  - 通过事件监听实现重复触发
  - 文件：`src/alert.css`, `src/alert.js`, `src-tauri/src/lib.rs`

- **持续触发机制**
  - 持续手贱行为每 3.5 秒重复提醒
  - 重置计时器逻辑，允许重复触发
  - 触发计数器准确反映提醒次数
  - 文件：`python/detector.py` (第 245-247 行)

- **黄色系UI设计**
  - 主界面采用温暖的黄色系配色
  - 主色调：`#FDB750`，背景：`#FFFBF0`
  - 渐变按钮效果
  - 黄色系阴影和徽章
  - 文件：`src/styles.css` (全局配色变量)

- **主窗口快捷按钮**
  - 🧪 调试器按钮 - 一键打开/关闭调试窗口
  - 📂 截图文件夹按钮 - 快速打开截图位置
  - 新增两个 Tauri 命令：`toggle_debug_window`, `open_screenshots_folder`
  - 文件：`src/index.html`, `src/main.js`, `src-tauri/src/lib.rs`

### 🐛 Fixed（修复）

- **截图文件夹打开问题**
  - 修复需要点击两次才能打开的 bug
  - 将 `.spawn()` 改为 `.output()` 确保同步执行
  - 添加错误处理和状态检查
  - 文件：`src-tauri/src/lib.rs` (第 290-300 行)

### 🔧 Changed（变更）

- **文案更新**
  - 主窗口标题：`挠头检测器` → `防手贱助手`
  - 副标题：`智能挠头检测器` → `😊 AI 帮你管住手 😊`
  - 事件日志：`检测到挠头` → `😏 又手贱了！`
  - 文件：`src/index.html`, `src/main.js`

- **UI优化**
  - 辅助功能按钮样式（浅黄色背景，悬停变深）
  - 徽章颜色使用主题色
  - 提示框采用黄色渐变背景
  - 按钮悬停效果优化（渐变方向反转）
  - 文件：`src/styles.css`

- **代码重构**
  - 移除 `if self.current_state != "Detected"` 条件判断
  - 添加计时器重置逻辑
  - 优化弹窗事件发送机制
  - 文件：`python/detector.py`, `src-tauri/src/lib.rs`

### 📝 Documentation（文档）

- 更新 README.md
  - 添加新功能说明
  - 更新使用指南
  - 添加配置调整说明
  - 修复错误的术语（挠头 → 手贱行为）

- 创建 CHANGELOG.md
  - 详细记录所有变更
  - 遵循 Keep a Changelog 规范

- 更新 RELEASE_NOTES.md
  - v1.1.0 版本说明
  - 详细的功能介绍和升级指南

---

## [1.0.0] - 2025-10-21

### 🎉 首次发布

NoPickie v1.0 正式发布！

### ✨ Added（新增）

- **核心检测功能**
  - 基于 MediaPipe 的姿态检测
  - 自适应检测区域（根据肩宽调整）
  - 双重判断（头部区域 + 脸部排除）
  - 平滑滤波（5帧）
  - 文件：`python/detector.py`, `python/main_simple.py`

- **提醒弹窗**
  - 简洁的通知条设计
  - 3秒自动消失
  - 固定位置（左上角 50, 50）
  - 文件：`src/alert.html`, `src/alert.js`, `src/alert.css`

- **主窗口界面**
  - 检测状态显示
  - 统计信息（触发次数、帧数）
  - 启动/停止按钮
  - 事件日志
  - 文件：`src/index.html`, `src/main.js`, `src/styles.css`

- **调试窗口**
  - 实时摄像头画面
  - 姿态关键点可视化
  - 检测区域显示（绿圈 + 红圈）
  - 实时状态和统计
  - 文件：`src/debug.html`, `src/debug.js`, `src/debug.css`

- **菜单栏控制**
  - 显示/隐藏主窗口
  - 显示/隐藏调试器
  - 启动/停止检测
  - 查看检测状态
  - 查看截图文件夹
  - 退出程序
  - 文件：`src-tauri/src/lib.rs` (菜单栏逻辑)

- **自动截图**
  - 检测到行为时自动截图
  - 保存到 `~/Pictures/NoPickie/screenshots/`
  - 文件名含时间戳
  - 包含骨骼点和检测区域
  - 文件：`python/main_simple.py` (第 146-160 行)

- **配置系统**
  - 可调整检测参数
  - JSON 配置文件
  - 支持热修改（重启生效）
  - 文件：`python/config.json`

- **Tauri 应用框架**
  - Rust 后端
  - Python 进程管理
  - 事件系统（JSON 通信）
  - 窗口管理
  - 文件：`src-tauri/src/lib.rs`, `src-tauri/tauri.conf.json`

### 🎨 Design（设计）

- 蓝色系UI设计
- 简洁现代的界面风格
- 响应式布局
- 平滑的动画效果

### 📚 Documentation（文档）

- README.md - 项目说明和使用指南
- RELEASE_NOTES.md - 版本发布说明
- INSTALL_GUIDE.md - 详细安装指南
- LICENSE - MIT 开源协议

### 🔧 Infrastructure（基础设施）

- Tauri 2.0 框架
- Rust + Node.js + Python 混合架构
- macOS 原生支持
- DMG 打包配置

---

## 版本类型说明

- **Added（新增）**：新功能
- **Changed（变更）**：现有功能的改变
- **Deprecated（弃用）**：即将移除的功能
- **Removed（移除）**：已移除的功能
- **Fixed（修复）**：Bug 修复
- **Security（安全）**：安全相关的修复

---

## 链接

- [v1.1.0] https://github.com/YOUR_USERNAME/nopickie/releases/tag/v1.1.0
- [v1.0.0] https://github.com/YOUR_USERNAME/nopickie/releases/tag/v1.0.0

---

**说明**：将 `YOUR_USERNAME` 替换为你的 GitHub 用户名

