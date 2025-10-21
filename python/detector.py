"""
智能挠头检测器 - 核心模块
使用MediaPipe进行姿态检测，识别挠头行为
"""

import cv2
import mediapipe as mp
import numpy as np
import time
import math
from collections import deque


class HeadScratchDetector:
    """挠头行为检测器"""
    
    def __init__(self, config):
        """
        初始化检测器
        
        Args:
            config: 配置字典，包含检测参数
        """
        # 加载配置
        self.distance_threshold = config['detection']['distance_threshold']
        self.time_threshold = config['detection']['time_threshold']
        self.smoothing_frames = config['detection']['smoothing_frames']
        self.head_zone_radius = config['detection'].get('head_zone_radius', 0.25)
        self.face_exclude_radius = config['detection'].get('face_exclude_radius', 0.10)
        self.show_skeleton = config['display']['show_skeleton']
        self.show_distance = config['display']['show_distance']
        
        # 初始化MediaPipe Pose
        self.mp_pose = mp.solutions.pose
        self.mp_drawing = mp.solutions.drawing_utils
        self.mp_drawing_styles = mp.solutions.drawing_styles
        
        self.pose = self.mp_pose.Pose(
            min_detection_confidence=0.5,
            min_tracking_confidence=0.5,
            model_complexity=1  # 0=Lite, 1=Full, 2=Heavy
        )
        
        # 状态变量
        self.current_state = "Normal"  # Normal/Warning/Detected
        self.scratch_start_time = None
        self.scratch_duration = 0.0
        self.trigger_count = 0
        self.last_trigger_time = 0
        
        # 自适应检测区域
        self.adaptive_head_zone = self.head_zone_radius  # 当前自适应半径
        self.adaptive_multiplier = 1.2  # 基于肩宽的倍数（可调整）
        
        # 距离历史（用于平滑）
        self.distance_history = deque(maxlen=self.smoothing_frames)
        
        # 用于FPS计算
        self.fps = 0
        self.frame_count = 0
        self.fps_start_time = time.time()
        
    def process_frame(self, frame):
        """
        处理单帧图像
        
        Args:
            frame: 输入的BGR图像
            
        Returns:
            processed_frame: 处理后的图像（带可视化）
            stats: 统计信息字典
        """
        # 镜像翻转画面（左右翻转，体验更自然）
        frame = cv2.flip(frame, 1)
        
        # 转换颜色空间（MediaPipe需要RGB）
        rgb_frame = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
        
        # MediaPipe检测
        results = self.pose.process(rgb_frame)
        
        # 初始化返回值
        processed_frame = frame.copy()
        current_distance = None
        
        if results.pose_landmarks:
            landmarks = results.pose_landmarks.landmark
            
            # 计算手头距离
            current_distance = self._calculate_hand_head_distance(landmarks)
            
            # 添加到历史记录
            self.distance_history.append(current_distance)
            
            # 使用平滑后的距离
            smoothed_distance = np.mean(list(self.distance_history))
            
            # 更新挠头状态
            self._update_scratch_state(smoothed_distance)
            
            # 绘制骨骼关键点
            if self.show_skeleton:
                self._draw_skeleton(processed_frame, results.pose_landmarks)
                # 绘制检测区域（调试用，显示绿色头部区域和红色脸部排除区域）
                self._draw_detection_zones(processed_frame, landmarks)
            
            # 使用平滑距离进行显示
            display_distance = smoothed_distance
        else:
            # 没有检测到人体
            display_distance = None
            self._reset_state()
        
        # 更新FPS
        self._update_fps()
        
        # 绘制信息面板
        self._draw_info_panel(processed_frame, display_distance)
        
        # 返回统计信息
        stats = {
            'state': self.current_state,
            'distance': display_distance,
            'duration': self.scratch_duration,
            'trigger_count': self.trigger_count,
            'fps': self.fps
        }
        
        return processed_frame, stats
    
    def _calculate_hand_head_distance(self, landmarks):
        """
        改进的距离计算：区分挠头和摸脸 + 自适应检测区域
        
        逻辑：
        1. 根据肩宽自适应调整检测区域大小
        2. 如果手在头部区域（大范围）内 → 可能是挠头
        3. 但如果手在脸部中心区域（小范围）内 → 排除，这是摸脸
        4. 返回到"头部区域且非脸部中心"的最小距离
        
        Args:
            landmarks: MediaPipe检测到的关键点列表
            
        Returns:
            float: 最小距离（归一化坐标）
        """
        # 关键点
        nose = landmarks[0]
        left_eye = landmarks[2]
        right_eye = landmarks[5]
        left_ear = landmarks[7]
        right_ear = landmarks[8]
        left_shoulder = landmarks[11]   # 左肩
        right_shoulder = landmarks[12]  # 右肩
        
        # === 自适应逻辑：根据肩宽计算检测区域 ===
        # 计算肩宽（人在画面中的大小指标）
        shoulder_width = math.sqrt(
            (right_shoulder.x - left_shoulder.x) ** 2 + 
            (right_shoulder.y - left_shoulder.y) ** 2
        )
        
        # 自适应调整 head_zone_radius
        # 距离近 → 人大 → 肩宽大 → 检测圆圈大
        # 距离远 → 人小 → 肩宽小 → 检测圆圈小
        adaptive_head_zone = shoulder_width * self.adaptive_multiplier
        
        # 设置合理的上下限（避免极端值）
        adaptive_head_zone = max(0.20, min(adaptive_head_zone, 0.60))
        
        # 更新当前自适应半径（用于显示）
        self.adaptive_head_zone = adaptive_head_zone
        
        # 头部中心（使用眼睛和耳朵，更准确）
        head_center_x = (left_eye.x + right_eye.x + left_ear.x + right_ear.x) / 4
        head_center_y = (left_eye.y + right_eye.y + left_ear.y + right_ear.y) / 4
        
        # 鼻子位置（脸部中心）
        nose_x = nose.x
        nose_y = nose.y
        
        # 手部关键点索引
        left_wrist = landmarks[15]
        right_wrist = landmarks[16]
        left_index = landmarks[19]
        right_index = landmarks[20]
        
        hands = [left_wrist, right_wrist, left_index, right_index]
        
        # 使用自适应的检测区域（而不是固定值）
        head_zone_radius = adaptive_head_zone
        face_exclude_radius = self.face_exclude_radius
        
        min_distance = 999.0  # 默认大值，表示不在检测区域
        
        for hand in hands:
            # 计算手到头部中心的距离
            dist_to_head = math.sqrt(
                (hand.x - head_center_x) ** 2 + 
                (hand.y - head_center_y) ** 2
            )
            
            # 计算手到鼻子的距离
            dist_to_nose = math.sqrt(
                (hand.x - nose_x) ** 2 + 
                (hand.y - nose_y) ** 2
            )
            
            # 判断逻辑：
            # 1. 手离头部中心足够近（在自适应头部区域内）
            # 2. 但手离鼻子足够远（不在脸部中心）
            # 这样可以检测挠头顶、后脑勺、侧面，但排除摸眼睛、鼻子
            if dist_to_head < head_zone_radius and dist_to_nose > face_exclude_radius:
                # 这是挠头行为
                min_distance = min(min_distance, dist_to_head)
        
        return min_distance
    
    def _update_scratch_state(self, distance):
        """
        根据距离更新挠头状态
        
        Args:
            distance: 手部到头部的距离
        """
        current_time = time.time()
        
        if distance < self.distance_threshold:
            # 手部接近头部
            if self.scratch_start_time is None:
                # 开始计时
                self.scratch_start_time = current_time
                self.current_state = "Warning"
            else:
                # 累计时长
                self.scratch_duration = current_time - self.scratch_start_time
                
                if self.scratch_duration >= self.time_threshold:
                    # 达到时长阈值，触发
                    if self.current_state != "Detected":
                        # 状态变为触发（第一次）
                        self.current_state = "Detected"
                        self.trigger_count += 1
                        self.last_trigger_time = current_time
                else:
                    # 还在计时中
                    self.current_state = "Warning"
        else:
            # 手部离开头部，重置
            self._reset_state()
    
    def _reset_state(self):
        """重置检测状态"""
        if self.current_state != "Normal":
            self.current_state = "Normal"
            self.scratch_start_time = None
            self.scratch_duration = 0.0
    
    def _draw_skeleton(self, frame, pose_landmarks):
        """
        在图像上绘制骨骼关键点
        
        Args:
            frame: 要绘制的图像
            pose_landmarks: MediaPipe检测到的关键点
        """
        # 使用MediaPipe内置绘制工具
        self.mp_drawing.draw_landmarks(
            frame,
            pose_landmarks,
            self.mp_pose.POSE_CONNECTIONS,
            landmark_drawing_spec=self.mp_drawing_styles.get_default_pose_landmarks_style()
        )
        
        # 高亮显示关键点（头部和手部）
        h, w, _ = frame.shape
        
        # 头部关键点（红色大圆）
        head_indices = [0, 7, 8]  # 鼻子、左耳、右耳
        for idx in head_indices:
            landmark = pose_landmarks.landmark[idx]
            cx, cy = int(landmark.x * w), int(landmark.y * h)
            cv2.circle(frame, (cx, cy), 8, (0, 0, 255), -1)
        
        # 手部关键点（蓝色大圆）
        hand_indices = [15, 16, 19, 20]  # 手腕和食指
        for idx in hand_indices:
            landmark = pose_landmarks.landmark[idx]
            cx, cy = int(landmark.x * w), int(landmark.y * h)
            cv2.circle(frame, (cx, cy), 8, (255, 0, 0), -1)
    
    def _draw_detection_zones(self, frame, landmarks):
        """
        绘制检测区域（调试用）- 自适应版本
        显示自适应的头部检测区域和脸部排除区域
        
        Args:
            frame: 要绘制的图像
            landmarks: MediaPipe检测到的关键点列表
        """
        h, w, _ = frame.shape
        
        # 获取关键点
        nose = landmarks[0]
        left_eye = landmarks[2]
        right_eye = landmarks[5]
        left_ear = landmarks[7]
        right_ear = landmarks[8]
        left_shoulder = landmarks[11]
        right_shoulder = landmarks[12]
        
        # 头部中心
        head_x = int(((left_eye.x + right_eye.x + left_ear.x + right_ear.x) / 4) * w)
        head_y = int(((left_eye.y + right_eye.y + left_ear.y + right_ear.y) / 4) * h)
        
        # 鼻子位置
        nose_x = int(nose.x * w)
        nose_y = int(nose.y * h)
        
        # 绘制肩膀连线（用于显示自适应基准）
        left_shoulder_x = int(left_shoulder.x * w)
        left_shoulder_y = int(left_shoulder.y * h)
        right_shoulder_x = int(right_shoulder.x * w)
        right_shoulder_y = int(right_shoulder.y * h)
        cv2.line(frame, (left_shoulder_x, left_shoulder_y), 
                (right_shoulder_x, right_shoulder_y), (255, 255, 0), 2)
        
        # 绘制自适应的头部检测区域（绿色圆圈）
        # 使用当前计算出的自适应半径
        head_radius = int(self.adaptive_head_zone * ((w + h) / 2))
        cv2.circle(frame, (head_x, head_y), head_radius, (0, 255, 0), 2)
        
        # 显示自适应半径值
        adaptive_text = f"Adaptive: {self.adaptive_head_zone:.2f}"
        cv2.putText(frame, adaptive_text, (head_x - 60, head_y - head_radius - 10), 
                    cv2.FONT_HERSHEY_SIMPLEX, 0.5, (0, 255, 0), 1)
        
        # 绘制脸部排除区域（红色圆圈）
        face_radius = int(self.face_exclude_radius * ((w + h) / 2))
        cv2.circle(frame, (nose_x, nose_y), face_radius, (0, 0, 255), 2)
        cv2.putText(frame, "Face Exclude", (nose_x - 50, nose_y + face_radius + 20), 
                    cv2.FONT_HERSHEY_SIMPLEX, 0.5, (0, 0, 255), 1)
    
    def _draw_info_panel(self, frame, distance):
        """
        在图像上绘制信息面板
        
        Args:
            frame: 要绘制的图像
            distance: 当前距离
        """
        # 状态颜色映射
        state_colors = {
            "Normal": (0, 255, 0),      # Green
            "Warning": (0, 255, 255),   # Yellow
            "Detected": (0, 0, 255)     # Red
        }
        
        # 半透明背景（增大高度以容纳新的 Zone 信息）
        overlay = frame.copy()
        cv2.rectangle(overlay, (5, 5), (300, 210), (0, 0, 0), -1)
        cv2.addWeighted(overlay, 0.5, frame, 0.5, 0, frame)
        
        # 文字参数
        font = cv2.FONT_HERSHEY_SIMPLEX
        font_scale = 0.6
        thickness = 2
        line_height = 30
        x = 15
        y = 30
        
        # 状态颜色
        color = state_colors[self.current_state]
        
        # 显示信息（去掉emoji，用文字和彩色圆点）
        status_text = f"Status: {self.current_state}"
        cv2.putText(frame, status_text, (x, y), font, font_scale, color, thickness)
        # 在文字右侧画一个彩色圆点作为状态指示器
        cv2.circle(frame, (x + 220, y - 7), 10, color, -1)
        
        if distance is not None:
            cv2.putText(frame, f"Distance: {distance:.3f}", 
                        (x, y + line_height), font, font_scale, (255, 255, 255), thickness)
        else:
            cv2.putText(frame, "Distance: N/A", 
                        (x, y + line_height), font, font_scale, (128, 128, 128), thickness)
        
        cv2.putText(frame, f"Duration: {self.scratch_duration:.1f}s", 
                    (x, y + line_height * 2), font, font_scale, (255, 255, 255), thickness)
        
        cv2.putText(frame, f"Triggers: {self.trigger_count}", 
                    (x, y + line_height * 3), font, font_scale, (255, 255, 255), thickness)
        
        # 显示自适应半径（用黄色显示，表示这是动态值）
        cv2.putText(frame, f"Zone: {self.adaptive_head_zone:.2f}", 
                    (x, y + line_height * 4), font, font_scale, (0, 255, 255), thickness)
        
        cv2.putText(frame, f"FPS: {self.fps:.1f}", 
                    (x, y + line_height * 5), font, font_scale, (255, 255, 255), thickness)
        
        # 底部提示
        h = frame.shape[0]
        cv2.putText(frame, "Press ESC to exit | Press R to reset counter", 
                    (10, h - 10), font, 0.5, (200, 200, 200), 1)
    
    def _update_fps(self):
        """更新FPS计数"""
        self.frame_count += 1
        elapsed = time.time() - self.fps_start_time
        
        if elapsed > 1.0:  # 每秒更新一次
            self.fps = self.frame_count / elapsed
            self.frame_count = 0
            self.fps_start_time = time.time()
    
    def reset_counter(self):
        """重置触发计数器"""
        self.trigger_count = 0
        print("✅ 计数器已重置")
    
    def get_stats(self):
        """
        获取统计信息
        
        Returns:
            dict: 统计信息
        """
        return {
            'state': self.current_state,
            'duration': self.scratch_duration,
            'trigger_count': self.trigger_count,
            'fps': self.fps
        }
    
    def cleanup(self):
        """清理资源"""
        self.pose.close()

