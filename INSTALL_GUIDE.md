# NoPickie 安装指南

## 📋 系统要求

- macOS 10.15 (Catalina) 或更高版本
- Python 3.8 或更高版本
- 摄像头（内置或外接）

## 🚀 快速安装（5 分钟）

### 步骤 1：检查 Python

打开"终端"（应用程序 → 实用工具 → 终端），输入：

```bash
python3 --version
```

**预期输出：** `Python 3.x.x`（x 为任意数字）

如果提示"command not found"，请先安装 Python：
- 访问 [python.org](https://www.python.org/downloads/)
- 下载 macOS 版本并安装

### 步骤 2：安装 Python 依赖

在终端中复制并执行以下命令：

```bash
pip3 install opencv-python mediapipe numpy
```

**等待安装完成**（约 2-5 分钟，需下载 100-150MB）

> 💡 **下载慢？** 使用国内镜像加速：
> ```bash
> pip3 install -i https://pypi.tuna.tsinghua.edu.cn/simple opencv-python mediapipe numpy
> ```

### 步骤 3：验证依赖安装

```bash
python3 -c "import cv2, mediapipe, numpy; print('✅ 依赖安装成功')"
```

**如果看到** `✅ 依赖安装成功`，说明依赖已正确安装！

### 步骤 4：安装 NoPickie 应用

1. **下载** `NoPickie_v1.0_Lite.dmg`
2. **双击** DMG 文件挂载
3. **拖拽** NoPickie.app 到"应用程序"文件夹

### 步骤 5：首次运行

由于应用未经过 Apple 公证，需要：

1. **打开访达** → 应用程序
2. **找到** NoPickie.app
3. **右键点击** → 选择"打开"
4. **在弹窗中** 再次点击"打开"

✅ 之后就可以正常双击启动了！

### 步骤 6：授予摄像头权限

1. **启动** NoPickie
2. **点击** "启动检测"按钮
3. **弹出权限请求对话框**：
   > "NoPickie 需要访问摄像头来检测挠头动作，帮助您改善工作习惯"
4. **点击** "好"

✅ 完成！现在可以开始使用了！

## 🎮 使用方法

### 基础操作

- **启动检测**：点击主界面的"启动检测"按钮
- **停止检测**：点击"停止检测"按钮
- **查看调试画面**：菜单栏 → "显示/隐藏调试器"
- **查看截图**：菜单栏 → "查看截图文件夹"

### 菜单栏功能

点击顶部菜单栏的 NoPickie 图标：

| 菜单项 | 功能 |
|--------|------|
| 显示/隐藏主窗口 | 切换主界面显示 |
| 显示/隐藏调试器 | 打开/关闭实时检测画面 |
| 启动/停止检测 | 快速控制检测开关 |
| 查看检测状态 | 显示当前运行状态 |
| 查看截图文件夹 | 打开截图保存位置 |
| 退出程序 | 完全退出应用 |

## ❓ 常见问题

### Q1: 提示"已损坏，无法打开"

**原因：** macOS 安全机制阻止未公证的应用

**解决方案 1**（推荐）：
- 右键点击应用 → 选择"打开"

**解决方案 2**：
```bash
xattr -cr /Applications/NoPickie.app
```

### Q2: 没有弹出摄像头权限请求

**原因：** Python 依赖未正确安装

**检查依赖：**
```bash
python3 -c "import cv2, mediapipe, numpy; print('OK')"
```

如果报错，重新安装依赖：
```bash
pip3 install --force-reinstall opencv-python mediapipe numpy
```

### Q3: 点击"启动检测"没反应

**可能原因：**
1. Python 依赖缺失（见 Q2）
2. Python 版本过低（需要 3.8+）

**诊断方法：**

在终端运行应用查看详细日志：
```bash
/Applications/NoPickie.app/Contents/MacOS/tauri-app
```

观察输出中的错误信息。

### Q4: 检测不准确

**调整检测参数：**

编辑配置文件（需要开发者技能）：
```bash
open /Applications/NoPickie.app/Contents/Resources/_up_/python/config.json
```

关键参数：
- `distance_threshold`: 0.15-0.30（默认 0.22）
- `time_threshold`: 1.0-3.0（默认 2.0）

### Q5: 如何卸载

1. **删除应用：**
   ```bash
   rm -rf /Applications/NoPickie.app
   ```

2. **删除截图（可选）：**
   ```bash
   rm -rf ~/Pictures/NoPickie
   ```

3. **卸载 Python 依赖（可选）：**
   ```bash
   pip3 uninstall opencv-python mediapipe numpy
   ```

## 📞 获取帮助

- **GitHub Issues:** [提交问题](https://github.com/YOUR_USERNAME/nopickie/issues)
- **邮箱：** your-email@example.com

---

**祝你使用愉快！改掉挠头习惯从今天开始！** 💪


