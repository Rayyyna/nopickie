# NoPickie v1.0 Release Notes

## 🎉 首次发布

NoPickie 是一个基于 AI 的实时挠头行为检测应用，帮助你改善工作习惯。

### ✨ 主要功能

- **实时检测**：使用摄像头实时监测挠头动作
- **智能提醒**：检测到挠头时立即弹窗提醒
- **调试窗口**：实时显示检测画面和姿态关键点
- **自动截图**：自动保存挠头瞬间到本地
- **菜单栏控制**：方便的菜单栏快捷操作

### 📦 下载说明

#### 轻量版 (推荐)

**文件名：** `NoPickie_v1.0_Lite.dmg`  
**大小：** ~25 MB  
**系统要求：**
- macOS 10.15 或更高版本
- Python 3.8+ 

**需要预先安装 Python 依赖：**
```bash
pip3 install opencv-python mediapipe numpy
```

> 💡 如果下载慢，使用国内镜像：
> ```bash
> pip3 install -i https://pypi.tuna.tsinghua.edu.cn/simple opencv-python mediapipe numpy
> ```

### 🚀 安装步骤

1. **安装 Python 依赖**（轻量版必需）
   ```bash
   pip3 install opencv-python mediapipe numpy
   ```

2. **下载并安装应用**
   - 下载 `NoPickie_v1.0_Lite.dmg`
   - 双击 DMG 文件
   - 将 NoPickie.app 拖到"应用程序"文件夹

3. **首次运行**
   - **右键点击** NoPickie.app → 选择"打开"
   - 在弹窗中再次点击"打开"

4. **授予摄像头权限**
   - 首次启动检测时会弹出权限请求
   - 点击"好"允许访问摄像头

### 📖 使用说明

#### 启动检测
1. 打开应用
2. 点击"启动检测"按钮
3. 摄像头开始工作，实时监测挠头动作

#### 查看调试画面
- 点击菜单栏图标 → "显示/隐藏调试器"
- 可以看到实时检测画面和姿态关键点

#### 查看截图
- 点击菜单栏图标 → "查看截图文件夹"
- 或手动打开：`~/Pictures/NoPickie/screenshots/`

### ⚠️ 已知问题

1. **首次打开提示"已损坏"**
   - 解决：右键点击应用 → 选择"打开"
   - 或终端执行：`xattr -cr /Applications/NoPickie.app`

2. **没有摄像头权限弹窗**
   - 原因：Python 依赖未正确安装
   - 解决：确认依赖安装：`python3 -c "import cv2, mediapipe, numpy; print('OK')"`

3. **菜单栏无图标**
   - 已知问题，功能正常，图标将在后续版本修复

### 🔜 后续计划

- [ ] 完整版（内置 Python，无需安装依赖）
- [ ] 自定义检测参数界面
- [ ] 更多提醒方式（声音、桌面通知等）
- [ ] 统计报告功能
- [ ] Windows 支持

### 🐛 反馈问题

如遇到问题，请在 [Issues](https://github.com/YOUR_USERNAME/nopickie/issues) 中反馈，包含：
- macOS 版本
- Python 版本
- 详细错误信息或截图

---

**感谢使用 NoPickie！如果有帮助，请给个 ⭐️ Star！**


