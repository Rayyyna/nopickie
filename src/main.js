const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

// UI 元素
let statusBadge;
let currentState;
let triggerCount;
let frameCount;
let startBtn;
let stopBtn;
let toggleDebugBtn;
let openScreenshotsBtn;
let eventLog;

// 统计UI元素
let todayCount;
let weekLabel;
let prevWeekBtn;
let nextWeekBtn;
let weekChart;
let weekChartInstance;

// 状态变量
let isRunning = false;
let eventCount = 0;
const MAX_LOG_ENTRIES = 50;

// 统计变量
let currentWeekOffset = 0;

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
  toggleDebugBtn = document.getElementById("toggle-debug-btn");
  openScreenshotsBtn = document.getElementById("open-screenshots-btn");
  eventLog = document.getElementById("event-log");
  
  // 统计UI元素
  todayCount = document.getElementById("today-count");
  weekLabel = document.getElementById("week-label");
  prevWeekBtn = document.getElementById("prev-week-btn");
  nextWeekBtn = document.getElementById("next-week-btn");
  weekChart = document.getElementById("weekChart");
  
  // 绑定按钮事件
  startBtn.addEventListener("click", startDetection);
  stopBtn.addEventListener("click", stopDetection);
  toggleDebugBtn.addEventListener("click", toggleDebugWindow);
  openScreenshotsBtn.addEventListener("click", openScreenshotsFolder);
  
  // 统计按钮事件
  prevWeekBtn.addEventListener("click", () => switchWeek(-1));
  nextWeekBtn.addEventListener("click", () => switchWeek(1));
  
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
    // 停止时刷新统计
    loadTodayStats();
  });
  
  // 初始化统计数据
  await loadTodayStats();
  await loadWeekStats(0);
  
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
  
  // 检测到手贱行为
  listen("scratch_detected", async (event) => {
    const count = event.payload.trigger_count;
    const duration = event.payload.duration;
    const distance = event.payload.distance;
    
    triggerCount.textContent = count;
    
    addLog("warning", `😏 又手贱了！第 ${count} 次，持续 ${duration.toFixed(1)}s`);
    
    // 记录触发到统计
    try {
      await invoke("record_trigger");
      // 刷新今天的统计
      await loadTodayStats();
    } catch (error) {
      console.error("记录触发失败:", error);
    }
    
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

// 切换调试窗口
async function toggleDebugWindow() {
  try {
    await invoke("toggle_debug_window");
    addLog("info", "🧪 切换调试窗口");
  } catch (error) {
    addLog("error", `❌ 切换调试窗口失败: ${error}`);
  }
}

// 打开截图文件夹
async function openScreenshotsFolder() {
  try {
    await invoke("open_screenshots_folder");
    addLog("success", "📂 已打开截图文件夹");
  } catch (error) {
    addLog("error", `❌ 打开文件夹失败: ${error}`);
  }
}

// ============ 统计功能 ============

// 加载今天统计（简化版）
async function loadTodayStats() {
  try {
    const stats = await invoke("get_today_stats");
    
    // 只更新今天触发次数
    todayCount.textContent = stats.trigger_count;
    
  } catch (error) {
    console.error("加载今天统计失败:", error);
  }
}

// 加载周统计
async function loadWeekStats(weekOffset) {
  try {
    const weekData = await invoke("get_week_stats", { weekOffset });
    
    currentWeekOffset = weekOffset;
    
    // 更新周标签
    weekLabel.textContent = weekData.week_label;
    
    // 更新按钮状态
    prevWeekBtn.disabled = !weekData.can_go_prev;
    nextWeekBtn.disabled = !weekData.can_go_next;
    
    // 渲染图表
    renderWeekChart(weekData);
    
  } catch (error) {
    console.error("加载周统计失败:", error);
  }
}

// 切换周
async function switchWeek(direction) {
  const newOffset = currentWeekOffset + direction;
  await loadWeekStats(newOffset);
}

// 渲染周图表（简化版：只显示触发次数，不包括今天）
function renderWeekChart(weekData) {
  const labels = [];
  const data = [];
  const weekdays = ['一', '二', '三', '四', '五', '六', '日'];
  
  // 提取数据（图表不包括今天的数据）
  weekData.days.forEach((day, index) => {
    const weekStart = new Date(weekData.week_start);
    weekStart.setDate(weekStart.getDate() + index);
    const dateStr = weekStart.getDate().toString().padStart(2, '0');
    labels.push(`${dateStr}\n${weekdays[index]}`);
    
    if (day && day.trigger_count !== undefined) {
      data.push(day.trigger_count);
    } else {
      data.push(null); // 今天或无数据
    }
  });
  
  // 销毁旧图表
  if (weekChartInstance) {
    weekChartInstance.destroy();
  }
  
  // 创建新图表（简化版：显示触发次数）
  weekChartInstance = new Chart(weekChart, {
    type: 'line',
    data: {
      labels: labels,
      datasets: [{
        label: '触发次数',
        data: data,
        borderColor: '#FDB750',
        backgroundColor: 'rgba(253, 183, 80, 0.1)',
        borderWidth: 3,
        pointRadius: 5,
        pointBackgroundColor: '#FDB750',
        pointBorderColor: '#fff',
        pointBorderWidth: 2,
        pointHoverRadius: 7,
        tension: 0.3,
        spanGaps: false // 不连接null数据点
      }]
    },
    options: {
      responsive: true,
      maintainAspectRatio: true,
      plugins: {
        legend: {
          display: false
        },
        tooltip: {
          callbacks: {
            label: function(context) {
              if (context.parsed.y === null) {
                return '无数据';
              }
              return `${context.parsed.y} 次`;
            }
          }
        }
      },
      scales: {
        y: {
          beginAtZero: true,
          ticks: {
            stepSize: 1,
            callback: function(value) {
              return Math.floor(value);
            }
          },
          grid: {
            color: 'rgba(0, 0, 0, 0.05)'
          }
        },
        x: {
          grid: {
            display: false
          }
        }
      }
    }
  });
}

