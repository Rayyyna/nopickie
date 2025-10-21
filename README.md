# NoPickie - 智能防手贱检测器

智能识别手部动作，帮助检测并控制以下症状：

- 爱抠头
- 爱拔头发
- 爱摸脸
- 爱搓眼睛
- 爱吃手指
- 爱扇自己

## ✨ 功能特点

- 🎥 **实时检测**：摄像头+opencv+mediapipe 实时检测手部和头部位置关系
- 🔔 **智能提醒**：检测到风险动作时立即弹窗提醒
- 📊 **调试窗口**：实时显示检测画面和姿态关键点
- 📸 **自动截图**：自动保存手贱瞬间的截图

## 📦 下载


**系统要求：**
- macOS 10.15+
- Python 3.8 或更高版本
- 需要安装依赖包（见下方安装说明）

**下载：** [NoPickie_v1.0_Lite.dmg](https://github.com/YOUR_USERNAME/nopickie/releases/latest)

## 🚀 安装与使用

### 第一步：安装 Python 依赖

在终端执行以下命令：

```bash
pip3 install opencv-python mediapipe numpy
```

> 💡 **提示：** 如果下载慢，可以使用国内镜像：
> ```bash
> pip3 install -i https://pypi.tuna.tsinghua.edu.cn/simple opencv-python mediapipe numpy
> ```

### 第二步：安装应用

1. 下载 `NoPickie_v1.0_Lite.dmg`
2. 双击 DMG 文件挂载
3. 将 NoPickie.app 拖到"应用程序"文件夹

### 第三步：首次运行

由于应用未经过 Apple 公证，首次打开需要在“系统设置” - “隐私与安全性”中授权

### 第四步：授予摄像头权限

首次运行时会弹出权限请求：

- 点击 **"好"** 授予摄像头权限
- 如果不小心点了"拒绝"，可以在 **系统设置 → 隐私与安全性 → 相机** 中手动开启

## 📖 使用指南

### 主界面

启动应用后，可以看到：
- **启动/停止检测** 按钮
- **实时统计信息**（帧数、检测次数等）

### 菜单栏功能

点击顶部菜单栏图标：
- **显示/隐藏主窗口** - 切换主界面显示
- **显示/隐藏调试器** - 打开实时检测画面窗口
- **启动/停止检测** - 快速控制检测
- **查看检测状态** - 查看当前运行状态
- **查看截图文件夹** - 打开截图保存位置
- **退出程序** - 完全退出应用

### 调试窗口

显示实时检测画面，包括：
- 摄像头实时画面
- 姿态关键点和连接线
- 当前检测状态（Normal/Warning/Detected）
- 统计信息

### 截图保存位置

检测到挠头时，自动保存截图到：
```
~/Pictures/NoPickie/screenshots/
```

可以通过菜单栏的"查看截图文件夹"快速打开。

## 🛠️ 开发者指南

### 环境要求

- Rust 1.70+
- Node.js 18+
- Python 3.8+
- Tauri CLI

### 克隆仓库

```bash
git clone https://github.com/YOUR_USERNAME/nopickie.git
cd nopickie
```

### 安装依赖

```bash
# Python 依赖
cd python
pip3 install -r requirements.txt

# 前端依赖
cd ..
npm install
```

### 开发模式

```bash
npm run tauri dev
```

### 构建发布版

```bash
npm run tauri build
```

生成的应用位于：
```
src-tauri/target/release/bundle/macos/NoPickie.app
src-tauri/target/release/bundle/dmg/NoPickie_1.0.0_aarch64.dmg
```

## 📁 项目结构

```
nopickie/
├── python/              # Python 检测脚本
│   ├── main_simple.py   # 主检测脚本
│   ├── detector.py      # 检测器核心模块
│   └── config.json      # 检测参数配置
├── src/                 # 前端代码
│   ├── index.html       # 主界面
│   ├── main.js          # 主界面逻辑
│   ├── styles.css       # 主界面样式
│   ├── alert.html       # 提醒弹窗
│   ├── debug.html       # 调试窗口
│   └── ...
├── src-tauri/           # Rust 后端
│   ├── src/
│   │   └── lib.rs       # 主逻辑
│   ├── tauri.conf.json  # Tauri 配置
│   └── Cargo.toml       # Rust 依赖
└── README.md
```

## ❓ 常见问题

### Q: 提示"已损坏，无法打开"？

**A:** macOS 安全限制。解决方法：
- 右键点击应用 → 选择"打开"
- 或在终端执行：`xattr -cr /path/to/NoPickie.app`

### Q: 没有弹出摄像头权限请求？

**A:** 可能是 Python 依赖未安装。请确认：
```bash
python3 -c "import cv2, mediapipe, numpy; print('依赖完整')"
```

### Q: 检测不准确？

**A:** 可以调整检测参数，编辑 `python/config.json`：
- `distance_threshold`: 距离阈值（默认 0.22）
- `time_threshold`: 时间阈值（默认 2.0 秒）

### Q: 截图保存在哪里？

**A:** `~/Pictures/NoPickie/screenshots/`，可通过菜单栏直接打开。

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 开源协议

MIT License

## 🙏 致谢

- [Tauri](https://tauri.app/) - 跨平台应用框架
- [MediaPipe](https://mediapipe.dev/) - 姿态检测
- [OpenCV](https://opencv.org/) - 计算机视觉库

---

**如果这个应用拯救了你的发际线/皮肤/眼睛，请给个 ⭐️ Star！**
**如果没用，说明你手贱的频率已经超出了 AI 的处理能力。** 🫡
