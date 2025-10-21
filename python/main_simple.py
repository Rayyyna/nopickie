"""
NoPickie - Python 检测脚本（简化版for Tauri）
输出 JSON 到 stdout 供 Tauri 读取
"""

import cv2
import json
import sys
import time
import base64
from datetime import datetime
from pathlib import Path
from detector import HeadScratchDetector

def load_config():
    """加载配置"""
    try:
        with open('config.json', 'r') as f:
            return json.load(f)
    except Exception as e:
        print(f"加载配置失败: {e}", file=sys.stderr)
        return {
            "detection": {
                "distance_threshold": 0.22,
                "time_threshold": 2.0,
                "smoothing_frames": 5,
                "head_zone_radius": 0.35,
                "face_exclude_radius": 0.10
            },
            "display": {
                "show_skeleton": True,
                "show_distance": True,
                "window_width": 640,
                "window_height": 480,
                "fps": 30
            },
            "camera": {
                "device_id": 0
            }
        }

def send_event(event_type, data):
    """发送事件到 Tauri（通过 JSON）"""
    event = {
        "event": event_type,
        "timestamp": datetime.now().isoformat(),
        **data
    }
    print(json.dumps(event), flush=True)

def main():
    print("========================================", file=sys.stderr)
    print("🧠 NoPickie Python 检测器启动...", file=sys.stderr)
    print("========================================", file=sys.stderr)
    
    # 加载配置
    config = load_config()
    
    # 创建截图目录（用户图片目录）
    screenshots_dir = Path.home() / "Pictures" / "NoPickie" / "screenshots"
    screenshots_dir.mkdir(parents=True, exist_ok=True)
    print(f"📂 截图目录: {screenshots_dir}", file=sys.stderr)
    
    # 初始化检测器
    try:
        detector = HeadScratchDetector(config)
        print("✅ 检测器已初始化", file=sys.stderr)
    except Exception as e:
        print(f"❌ 检测器初始化失败: {e}", file=sys.stderr)
        sys.exit(1)
    
    # 打开摄像头
    cap = cv2.VideoCapture(config['camera']['device_id'])
    
    if not cap.isOpened():
        print("❌ 无法打开摄像头", file=sys.stderr)
        print("💡 可能需要授予摄像头权限", file=sys.stderr)
        
        # 发送权限需要事件到前端
        send_event("camera_permission_needed", {
            "message": "无法访问摄像头",
            "help": "请在系统弹出的对话框中点击「好」允许摄像头访问，然后重新点击「启动检测」"
        })
        
        # 等待一下，确保事件发送成功
        time.sleep(0.5)
        sys.exit(1)
    
    print("✅ 摄像头已打开", file=sys.stderr)
    print("🎥 开始检测...", file=sys.stderr)
    
    # 状态追踪
    last_state = "Normal"
    frame_count = 0
    last_frame_send_time = 0
    FRAME_SEND_INTERVAL = 0.2  # 每0.2秒发送一帧（5 FPS）
    
    try:
        while True:
            ret, frame = cap.read()
            
            if not ret:
                print("❌ 无法读取摄像头", file=sys.stderr)
                break
            
            # 使用检测器处理
            processed_frame, stats = detector.process_frame(frame)
            
            frame_count += 1
            current_state = stats['state']
            
            # 定期发送视频帧到调试窗口（控制帧率避免过载）
            current_time = time.time()
            if current_time - last_frame_send_time >= FRAME_SEND_INTERVAL:
                # 适度缩小图像（宽度调整到720px，保持清晰度）
                h, w = processed_frame.shape[:2]
                target_width = 720
                target_height = int(h * target_width / w)
                display_frame = cv2.resize(processed_frame, (target_width, target_height))
                
                # 编码为JPEG（高质量）
                _, buffer = cv2.imencode('.jpg', display_frame, [cv2.IMWRITE_JPEG_QUALITY, 90])
                # 转base64
                img_b64 = base64.b64encode(buffer).decode('utf-8')
                
                # 发送debug_frame事件
                send_event("debug_frame", {
                    "image_b64": img_b64,
                    "status": current_state.lower(),
                    "count": stats['trigger_count'],
                    "duration": stats['duration']
                })
                
                last_frame_send_time = current_time
            
            # 状态变化时发送事件
            if current_state != last_state:
                send_event("state_changed", {
                    "state": current_state,
                    "stats": stats
                })
                print(f"📊 状态变化: {last_state} → {current_state}", file=sys.stderr)
            
            # 检测到挠头时发送事件
            if current_state == 'Detected' and last_state != 'Detected':
                # 保存截图（使用完整路径）
                timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
                filename = f"screenshot_{timestamp}.jpg"
                filepath = screenshots_dir / filename
                cv2.imwrite(str(filepath), processed_frame)
                
                print(f"📸 截图已保存: {filepath}", file=sys.stderr)
                
                send_event("scratch_detected", {
                    "trigger_count": stats['trigger_count'],
                    "duration": stats['duration'],
                    "distance": stats['distance'],
                    "screenshot": str(filepath),
                    "screenshot_dir": str(screenshots_dir)
                })
                print(f"🔔 检测到挠头！触发次数: {stats['trigger_count']}", file=sys.stderr)
            
            # 定期发送状态更新（每5秒）
            if frame_count % 150 == 0:
                send_event("status_update", {
                    "state": current_state,
                    "stats": stats,
                    "frame_count": frame_count
                })
            
            last_state = current_state
            
            # 控制帧率
            time.sleep(1/30)
    
    except KeyboardInterrupt:
        print("\n⏹ 收到中断信号", file=sys.stderr)
    
    except Exception as e:
        print(f"❌ 错误: {e}", file=sys.stderr)
    
    finally:
        # 清理
        cap.release()
        detector.cleanup()
        print("✅ 资源已清理", file=sys.stderr)

if __name__ == '__main__':
    main()
