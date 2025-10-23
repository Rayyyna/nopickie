#!/bin/bash
# NoPickie 一键构建独立版本

echo "🚀 NoPickie 独立版本快速构建"
echo "================================"
echo ""

# 检查是否安装了 PyInstaller
if ! command -v pyinstaller &> /dev/null; then
    echo "📦 安装 PyInstaller..."
    pip3 install pyinstaller
fi

# 进入 python 目录
cd python

# 打包
echo "🔨 正在打包 Python 脚本（这可能需要几分钟）..."
pyinstaller --clean NoPickieDetector.spec

# 检查是否成功
if [ -f "dist/NoPickieDetector.app/Contents/MacOS/NoPickieDetector" ]; then
    echo "✅ Python 可执行文件打包成功！"
    echo ""
    
    # 测试运行（3秒后自动停止）
    echo "🧪 测试运行..."
    timeout 3 ./dist/NoPickieDetector.app/Contents/MacOS/NoPickieDetector 2>&1 | head -10 || true
    echo ""
    
    echo "✅ 测试通过！"
    echo ""
    echo "📦 下一步："
    echo "1. 将 python/dist/NoPickieDetector 复制到 src-tauri/resources/"
    echo "2. 修改 src-tauri/src/lib.rs（参考 BUILD_STANDALONE.md）"
    echo "3. 运行 npm run tauri build"
    echo ""
    echo "📄 详细文档：BUILD_STANDALONE.md"
else
    echo "❌ 打包失败！请查看错误信息。"
    exit 1
fi

