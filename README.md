# NoPickie - 智能防手贱检测器

> 😊 AI 帮你管住手 😊

智能识别手部动作，帮助检测并控制以下手贱行为：

- 🤚 爱抠头
- 💇 爱拔头发  
- 👐 爱摸脸
- 👁️ 爱搓眼睛
- 🫵 爱吃手指
- 👋 自扇耳光

## ✨ 功能特点

- 🎥 **实时检测**：基于 OpenCV + MediaPipe 的姿态识别，实时监控手部和头部位置关系
- 😊 **弹窗提醒**：检测到手贱行为时弹窗提醒 "😊 你的手在干嘛？😊"
- 🧪 **调试窗口**：实时显示检测画面、姿态关键点和检测区域
- 📸 **自动截图**：自动保存手贱瞬间的截图，方便复盘

## 📦 下载安装

### 系统要求

- macOS 10.15+
- Python 3.8 或更高版本
- 摄像头权限

### 下载

**最新版本：** [NoPickie v1.1.0](https://github.com/Rayyyna/nopickie/releases/latest)


## 🚀 快速开始

### 第一步：安装 Python 依赖

在终端执行以下命令：

```bash
pip3 install opencv-python mediapipe numpy
```

> 💡 **提示**：如果下载慢，可以使用国内镜像：
> ```bash
> pip3 install -i https://pypi.tuna.tsinghua.edu.cn/simple opencv-python mediapipe numpy
> ```

### 第二步：安装应用

1. 下载 `NoPickie_1.0.0_aarch64.dmg`
2. 双击 DMG 文件挂载
3. 将 NoPickie.app 拖到"应用程序"文件夹

### 第三步：首次运行

由于应用未经过 Apple 公证，首次打开需要：

1. 右键点击应用 → 选择"打开"
2. 或在"系统设置" → "隐私与安全性"中点击"仍要打开"

### 第四步：授予摄像头权限

首次运行时会弹出权限请求：

- 点击 **"好"** 授予摄像头权限
- 如果不小心点了"拒绝"，可以在 **系统设置 → 隐私与安全性 → 相机** 中手动开启

---

## 📖 使用指南

### 主界面功能

启动应用后的主窗口包含：

#### 🎛️ 控制区
- **▶️ 启动检测** - 开始监控手贱行为
- **⏸️ 停止检测** - 停止监控
- **🧪 调试器** - 一键打开/关闭调试窗口
- **📂 截图文件夹** - 快速打开截图保存位置

#### 📊 状态显示
- **当前状态**：Normal（正常）/ Warning（警告）/ Detected（检测到）
- **触发次数**：累计提醒次数
- **已处理帧**：实时处理的视频帧数

#### 📝 事件日志
实时显示检测事件，包括：
- 启动/停止信息
- 状态变化
- 手贱行为检测 "😏 又手贱了！第 X 次"

### 菜单栏功能

点击顶部菜单栏图标可以：

- **显示/隐藏主窗口** - 切换主界面显示
- **显示/隐藏调试器** - 打开实时检测画面
- **启动/停止检测** - 快速控制检测
- **查看检测状态** - 查看当前运行状态
- **查看截图文件夹** - 打开截图保存位置
- **退出程序** - 完全退出应用

### 调试窗口

显示实时检测画面，方便调试和查看效果：

- 📹 摄像头实时画面（镜像翻转）
- 🔴 姿态关键点和连接线
- 🟢 头部检测区域（绿色圆圈，自适应大小）
- 🔴 脸部排除区域（红色圆圈）
- 📊 实时状态和统计信息
- 📐 自适应半径显示

### 提醒弹窗

检测到手贱行为时会出现带都动效果的弹窗，3s后自动消失

### 截图功能

每次检测到手贱行为会自动保存截图：

- 📍 **保存位置**：`~/Pictures/NoPickie/screenshots/`
- 📅 **文件命名**：`screenshot_20251022_143018.jpg`（含日期时间）
- 📸 **截图内容**：包含骨骼点和检测区域的完整画面
- 🖱️ **快速打开**：主窗口点击"📂 截图文件夹"按钮

---

## ⚙️ 配置调整

如果检测不准确或太敏感，可以修改配置文件 `python/config.json`：

```json
{
  "detection": {
    "distance_threshold": 0.35,      // 距离阈值（越小越敏感）
    "time_threshold": 3.5,           // 持续时间阈值（秒）
    "smoothing_frames": 5,           // 平滑帧数（减少抖动）
    "head_zone_radius": 0.25,        // 头部检测区域半径（自适应）
    "face_exclude_radius": 0.10      // 脸部排除区域半径
  }
}
```

**调整建议**：
- 太容易误报 → 增大 `distance_threshold` 或 `time_threshold`
- 检测不到 → 减小 `distance_threshold`
- 想区分摸脸和抠头 → 调整 `face_exclude_radius`

---

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

热重载支持，修改代码后自动刷新。

### 构建发布版

```bash
npm run tauri build
```

生成的应用位于：
```
src-tauri/target/release/bundle/macos/NoPickie.app
src-tauri/target/release/bundle/dmg/NoPickie_1.0.0_aarch64.dmg
```

---

## 📁 项目结构

```
NoPickieV1.0/
├── python/                    # Python 检测脚本
│   ├── main_simple.py         # 主检测脚本（输出JSON事件）
│   ├── detector.py            # 检测器核心模块（MediaPipe）
│   ├── config.json            # 检测参数配置
│   └── requirements.txt       # Python依赖
├── src/                       # 前端代码
│   ├── index.html             # 主界面（黄色系UI）
│   ├── main.js                # 主界面逻辑
│   ├── styles.css             # 主界面样式
│   ├── alert.html             # 提醒弹窗
│   ├── alert.js               # 弹窗逻辑（抖动动画）
│   ├── alert.css              # 弹窗样式
│   ├── debug.html             # 调试窗口
│   └── debug.js               # 调试窗口逻辑
├── src-tauri/                 # Rust 后端
│   ├── src/
│   │   └── lib.rs             # 主逻辑（进程管理、事件转发）
│   ├── tauri.conf.json        # Tauri 配置
│   ├── Cargo.toml             # Rust 依赖
│   └── icons/                 # 应用图标
├── alert_config.json          # 弹窗配置
├── README.md                  # 本文档
├── RELEASE_NOTES.md           # 版本发布说明
└── CHANGELOG.md               # 变更日志
```

---

## ❓ 常见问题

### Q: 提示"已损坏，无法打开"？

**A:** macOS 安全限制。解决方法：
- 右键点击应用 → 选择"打开"
- 或在终端执行：`xattr -cr /Applications/NoPickie.app`

### Q: 没有弹出摄像头权限请求？

**A:** 可能是 Python 依赖未安装。请确认：
```bash
python3 -c "import cv2, mediapipe, numpy; print('✅ 依赖完整')"
```

如果报错，重新安装依赖：
```bash
pip3 install opencv-python mediapipe numpy
```

### Q: 检测不准确，总是误报？

**A:** 可以调整检测参数：
1. 打开 `python/config.json`
2. 增大 `distance_threshold`（如改为 0.40）
3. 增大 `time_threshold`（如改为 5.0）
4. 重启应用测试

### Q: 弹窗位置不合适？

**A:** 修改 `alert_config.json`：
```json
{
  "position": {
    "x": 50,    // 距离屏幕左边距离
    "y": 50     // 距离屏幕顶部距离
  }
}
```

### Q: 截图保存在哪里？

**A:** `~/Pictures/NoPickie/screenshots/`

快捷打开方式：
- 主窗口点击"📂 截图文件夹"按钮
- 菜单栏选择"查看截图文件夹"

### Q: 如何关闭持续提醒？

**A:** 增大 `time_threshold`，或者...停止手贱 😊

### Q: 调试窗口的绿圈和红圈是什么？

**A:** 
- 🟢 **绿圈**：头部检测区域，手在这个范围内会被检测
- 🔴 **红圈**：脸部排除区域，在这个范围内不算（区分摸脸）
- 绿圈会根据你离摄像头的距离自动调整大小


---

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

如果你有好的想法：
- 💡 新的提醒文案
- 🎨 UI 改进建议
- 🐛 Bug 反馈
- ✨ 新功能建议

都欢迎在 GitHub Issues 中讨论！

---

## 📄 开源协议

MIT License - 详见 [LICENSE](LICENSE)

---

## 🙏 致谢

本项目基于以下开源项目：

- [Tauri](https://tauri.app/) - 轻量级跨平台应用框架
- [MediaPipe](https://mediapipe.dev/) - Google 的姿态检测库
- [OpenCV](https://opencv.org/) - 计算机视觉库
- [Rust](https://www.rust-lang.org/) - 系统编程语言

---

## 📊 项目统计

![GitHub stars](https://img.shields.io/github/stars/YOUR_USERNAME/nopickie?style=social)
![GitHub forks](https://img.shields.io/github/forks/YOUR_USERNAME/nopickie?style=social)
![GitHub issues](https://img.shields.io/github/issues/YOUR_USERNAME/nopickie)
![GitHub license](https://img.shields.io/github/license/YOUR_USERNAME/nopickie)

---

**如果这个应用拯救了你的发际线/皮肤/眼睛，请给个 ⭐️ Star！**  
**如果没用，说明你手贱的频率已经超出了 AI 的处理能力。** 🫡

---

## 📮 联系方式

- GitHub Issues: [提交问题](https://github.com/Rayyyna/nopickie/issues)
- Email:raynanrx@gmail.com

---

**版本**：v1.1.0  
**最后更新**：2025-10-22
