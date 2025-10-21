# NoPickie 安装指南

## 🚀 快速安装（5 分钟）

### 1. 安装 Python 依赖

```bash
pip3 install opencv-python mediapipe numpy
```

> 💡 **网络慢？** 使用清华镜像：
> ```bash
> pip3 install -i https://pypi.tuna.tsinghua.edu.cn/simple opencv-python mediapipe numpy
> ```

**验证安装：**
```bash
python3 -c "import cv2, mediapipe, numpy; print('✅ 依赖OK')"
```

---

### 2. 安装应用

1. 下载 `NoPickie_v1.0_Final.dmg`
2. 双击挂载 → 拖拽到"应用程序"
3. **右键点击** NoPickie.app → 选择"打开"（首次）

---

### 3. 授予摄像头权限

启动检测时会弹出权限请求，点击"好"即可。

---

## ❓ 故障排除

### "已损坏，无法打开"
**解决：** 右键点击 → 选择"打开"

### "没有摄像头权限弹窗"
**原因：** Python 依赖未安装  
**解决：** 执行步骤 1

### "启动检测没反应"
**检查：**
```bash
python3 --version  # 确认有 Python 3.8+
python3 -c "import cv2, mediapipe, numpy"  # 确认依赖完整
```

### 卸载
```bash
rm -rf /Applications/NoPickie.app ~/Pictures/NoPickie
```

---

更多问题见 [GitHub Issues](https://github.com/Rayyyna/nopickie/issues)
