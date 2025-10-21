const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

// UI 元素
let statusBadge;
let currentState;
let triggerCount;
let frameCount;
let startBtn;
let stopBtn;
let eventLog;

// 状态变量
let isRunning = false;
let eventCount = 0;
const MAX_LOG_ENTRIES = 50;

// 系统通知已删除，改用自定义弹窗

// 初始化
window.addEventListener("DOMContentLoaded", async () => {
  // 获取 UI 元素
  statusBadge = document.getElementById("status-badge");
  currentState = document.getElementById("current-state");
  triggerCount = document.getElementById("trigger-count");
  frameCount = document.getElementById("frame-count");
  startBtn = document.getElementById("start-btn");
  stopBtn = document.getElementById("stop-btn");
  eventLog = document.getElementById("event-log");
  
  // 绑定按钮事件
  startBtn.addEventListener("click", startDetection);
  stopBtn.addEventListener("click", stopDetection);
  
  // 监听 Python 事件
  setupEventListeners();
  
  // 监听菜单栏启动事件
  listen("detection_started_from_menu", () => {
    addLog("success", "✅ 检测已从菜单栏启动");
    isRunning = true;
    updateUIState();
  });
  
  // 监听菜单栏停止事件
  listen("detection_stopped_from_menu", () => {
    addLog("info", "🛑 检测已从菜单栏停止");
    isRunning = false;
    updateUIState();
    currentState.textContent = "-";
    statusBadge.textContent = "已停止";
    statusBadge.className = "badge badge-normal";
  });
  
  // 添加欢迎日志
  addLog("info", "🎉 NoPickie 已就绪，点击「启动检测」开始");
});

// 启动检测
async function startDetection() {
  try {
    addLog("info", "🚀 正在启动检测器...");
    const result = await invoke("start_detection");
    addLog("success", result);
    
    isRunning = true;
    updateUIState();
  } catch (error) {
    addLog("error", `❌ 启动失败: ${error}`);
  }
}

// 停止检测
async function stopDetection() {
  try {
    addLog("info", "🛑 正在停止检测器...");
    const result = await invoke("stop_detection");
    addLog("success", result);
    
    isRunning = false;
    updateUIState();
    
    // 重置显示
    currentState.textContent = "-";
    statusBadge.textContent = "已停止";
    statusBadge.className = "badge badge-normal";
  } catch (error) {
    addLog("error", `❌ 停止失败: ${error}`);
  }
}

// 更新 UI 状态
function updateUIState() {
  startBtn.disabled = isRunning;
  stopBtn.disabled = !isRunning;
  
  if (isRunning) {
    statusBadge.textContent = "运行中";
    statusBadge.className = "badge badge-running";
  } else {
    statusBadge.textContent = "已停止";
    statusBadge.className = "badge badge-normal";
  }
}

// 设置事件监听器
function setupEventListeners() {
  // 启动事件
  listen("started", (event) => {
    addLog("success", "✅ " + event.payload.message);
  });
  
  // 检测器就绪
  listen("detector_ready", (event) => {
    addLog("success", "✅ " + event.payload.message);
  });
  
  // 摄像头就绪
  listen("camera_ready", (event) => {
    addLog("success", "📹 " + event.payload.message);
  });
  
  // 状态变化
  listen("state_changed", (event) => {
    const state = event.payload.state;
    currentState.textContent = state;
    
    // 更新徽章
    if (state === "Normal") {
      statusBadge.textContent = "正常";
      statusBadge.className = "badge badge-normal";
    } else if (state === "Warning") {
      statusBadge.textContent = "警告";
      statusBadge.className = "badge badge-warning";
    } else if (state === "Detected") {
      statusBadge.textContent = "检测到！";
      statusBadge.className = "badge badge-detected";
    }
    
    addLog("info", `🔄 状态变化: ${event.payload.previous_state} → ${state}`);
  });
  
  // 检测到挠头
  listen("scratch_detected", (event) => {
    const count = event.payload.trigger_count;
    const duration = event.payload.duration;
    const distance = event.payload.distance;
    
    triggerCount.textContent = count;
    
    addLog("warning", `⚠️ 检测到挠头！次数: ${count}, 持续: ${duration}s, 距离: ${distance}`);
    
    // 注释掉：通知已经由 Rust 后端直接发送，不需要前端调用了
    // sendScratchNotification(count, duration);
  });
  
  // 心跳（每秒）
  listen("heartbeat", (event) => {
    const state = event.payload.state;
    const frames = event.payload.frames;
    const triggers = event.payload.triggers;
    
    frameCount.textContent = frames;
    triggerCount.textContent = triggers;
    
    // 不记录心跳到日志，避免刷屏
  });
  
  // 错误
  listen("error", (event) => {
    addLog("error", "❌ 错误: " + event.payload.message);
  });
  
  // 摄像头权限需要
  listen("camera_permission_needed", (event) => {
    const message = event.payload.message;
    const help = event.payload.help;
    
    addLog("warning", `⚠️ ${message}`);
    addLog("info", `💡 ${help}`);
    
    // 恢复按钮状态，允许用户重新启动
    isRunning = false;
    updateUIState();
    
    // 重置状态显示
    currentState.textContent = "-";
    statusBadge.textContent = "需要权限";
    statusBadge.className = "badge badge-warning";
  });
  
  // 停止
  listen("stopped", (event) => {
    addLog("info", "🛑 " + event.payload.message);
    isRunning = false;
    updateUIState();
  });
  
  // 清理
  listen("cleanup", (event) => {
    addLog("info", "🧹 " + event.payload.message);
  });
}

// 添加日志
function addLog(type, message) {
  eventCount++;
  
  const time = new Date().toLocaleTimeString();
  const entry = document.createElement("div");
  entry.className = `log-entry log-${type}`;
  entry.textContent = `[${time}] ${message}`;
  
  // 插入到顶部
  eventLog.insertBefore(entry, eventLog.firstChild);
  
  // 限制日志条数
  while (eventLog.children.length > MAX_LOG_ENTRIES) {
    eventLog.removeChild(eventLog.lastChild);
  }
}
