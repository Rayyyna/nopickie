#!/bin/bash
# NoPickie 独立版本构建脚本

set -e

echo "🚀 开始构建 NoPickie 独立版本..."

# 1. 安装 PyInstaller
echo "📦 安装 PyInstaller..."
pip3 install pyinstaller

# 2. 进入 python 目录
cd python

# 3. 使用 PyInstaller 打包
echo "🔨 打包 Python 脚本..."
pyinstaller --name=NoPickieDetector \
    --onefile \
    --windowed \
    --add-data "config.json:." \
    --hidden-import=cv2 \
    --hidden-import=mediapipe \
    --hidden-import=numpy \
    --collect-all mediapipe \
    --collect-all cv2 \
    main_simple.py

echo "✅ Python 可执行文件已生成: python/dist/NoPickieDetector"

# 4. 回到根目录
cd ..

# 5. 复制可执行文件到 Tauri 资源目录
echo "📋 复制到 Tauri 资源目录..."
mkdir -p src-tauri/resources
cp python/dist/NoPickieDetector src-tauri/resources/

echo "🎉 构建完成！"
echo ""
echo "下一步："
echo "1. 修改 src-tauri/src/lib.rs 中的 Python 路径"
echo "2. 运行 'npm run tauri build' 构建最终应用"

