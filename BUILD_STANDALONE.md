# 构建开箱即用版本指南

## 🎯 目标

将 Python 和所有依赖（opencv、mediapipe、numpy）打包进应用，用户无需手动安装。

---

## 📋 方案：使用 PyInstaller

### 优点
- ✅ 用户无需安装 Python
- ✅ 用户无需安装依赖包
- ✅ 双击即可运行
- ✅ 体积可控（约 200-300MB）

---

## 🛠️ 构建步骤

### 第一步：安装 PyInstaller

```bash
pip3 install pyinstaller
```

### 第二步：测试打包 Python 脚本

```bash
cd python

# 使用 spec 文件打包
pyinstaller NoPickieDetector.spec

# 测试生成的可执行文件
./dist/NoPickieDetector.app/Contents/MacOS/NoPickieDetector
```

**预期输出**：应该看到 Python 检测器启动的日志

### 第三步：修改 Rust 代码

编辑 `src-tauri/src/lib.rs`，找到 `find_python()` 函数附近，添加：

```rust
// 新增：查找打包的 Python 可执行文件
fn find_detector_executable(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let resource_dir = app.path()
        .resource_dir()
        .map_err(|e| format!("无法获取资源目录: {}", e))?;
    
    // 尝试使用打包的可执行文件
    let standalone_path = resource_dir.join("NoPickieDetector");
    if standalone_path.exists() {
        println!("✅ 使用打包版本: {:?}", standalone_path);
        return Ok(standalone_path);
    }
    
    // 如果找不到，回退到系统 Python
    println!("⚠️ 打包版本不存在，尝试系统 Python");
    let python_dir = resource_dir.join("python");
    let script_path = python_dir.join("main_simple.py");
    
    if !script_path.exists() {
        return Err("找不到 Python 脚本".to_string());
    }
    
    // 返回脚本路径（后续用系统 Python 执行）
    Ok(script_path)
}
```

然后修改 `start_detection` 函数：

```rust
// 查找可执行文件或脚本
let detector_path = find_detector_executable(&app)?;

let mut cmd = if detector_path.to_str().unwrap().ends_with("NoPickieDetector") {
    // 打包版本：直接运行
    Command::new(&detector_path)
} else {
    // 开发版本：用系统 Python 运行
    let python_path = find_python()?;
    let mut cmd = Command::new(&python_path);
    cmd.arg(&detector_path);
    cmd
};

// 设置工作目录
let work_dir = detector_path.parent().unwrap();
cmd.current_dir(work_dir);
```

### 第四步：更新 Tauri 配置

编辑 `src-tauri/tauri.conf.json`：

```json
{
  "bundle": {
    "resources": [
      "../python/dist/NoPickieDetector",
      "../python/config.json",
      "../alert_config.json"
    ]
  }
}
```

### 第五步：完整构建

```bash
# 1. 打包 Python
cd python
pyinstaller NoPickieDetector.spec
cd ..

# 2. 复制到资源目录
mkdir -p src-tauri/resources
cp python/dist/NoPickieDetector src-tauri/resources/
cp python/config.json src-tauri/resources/

# 3. 构建 Tauri 应用
npm run tauri build
```

---

## 🧪 测试

生成的 DMG 文件在：
```
src-tauri/target/release/bundle/dmg/NoPickie_1.1.0_aarch64.dmg
```

**测试步骤**：
1. 在没有安装 Python 的机器上安装
2. 或者临时重命名 Python：`sudo mv /usr/local/bin/python3 /usr/local/bin/python3.bak`
3. 运行 NoPickie.app
4. 应该能正常启动检测

---

## 📦 方案对比

### 当前版本（轻量版）
- ✅ 体积小（9MB）
- ❌ 需要用户安装 Python
- ❌ 需要用户安装依赖

### 独立版本（Full）
- ✅ 开箱即用
- ✅ 用户体验好
- ❌ 体积大（200-300MB）

---

## 🎯 推荐策略

提供两个版本：

1. **NoPickie_Lite.dmg** (9MB)
   - 轻量版
   - 需要 Python 环境
   - 适合开发者

2. **NoPickie_Full.dmg** (250MB)
   - 完整版
   - 开箱即用
   - 适合普通用户

---

## 🐛 常见问题

### Q: PyInstaller 打包失败？

```bash
# 清理并重试
cd python
rm -rf build dist __pycache__
pyinstaller --clean NoPickieDetector.spec
```

### Q: 可执行文件运行出错？

```bash
# 添加 --debug 参数查看详细日志
pyinstaller --debug all NoPickieDetector.spec
```

### Q: mediapipe 找不到资源文件？

在 spec 文件中添加：
```python
from PyInstaller.utils.hooks import collect_data_files
datas += collect_data_files('mediapipe')
```

### Q: 体积太大？

使用 UPX 压缩：
```bash
brew install upx
pyinstaller --upx-dir=/usr/local/bin NoPickieDetector.spec
```

---

## 📊 预期体积

- Python 可执行文件：~200MB
- 最终 DMG：~250MB
- 安装后：~300MB

---

## 🔧 高级优化

### 使用虚拟环境减小体积

```bash
# 创建干净的虚拟环境
python3 -m venv venv_build
source venv_build/bin/activate

# 只安装必要依赖
pip install opencv-python mediapipe numpy pyinstaller

# 打包
cd python
pyinstaller NoPickieDetector.spec
```

### 排除不必要的模块

在 spec 文件中添加：
```python
excludes=[
    'tkinter',
    'matplotlib',
    'scipy',
    'pandas',
]
```

---

需要帮助？查看 [PyInstaller 文档](https://pyinstaller.org/)

