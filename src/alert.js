const { getCurrentWindow } = window.__TAURI__.window;
const { listen } = window.__TAURI__.event;

window.addEventListener("DOMContentLoaded", () => {
  console.log("通知条已加载");
  
  const notificationBar = document.getElementById('notification-bar');
  
  // 监听抖动事件
  listen('shake-alert', () => {
    console.log("🔔 收到抖动请求");
    triggerShake();
  });
  
  // 关闭按钮事件
  const closeBtn = document.getElementById('close-btn');
  if (closeBtn) {
    closeBtn.addEventListener('click', async (e) => {
      e.stopPropagation();
      await closeNotification();
    });
  }
  
  // ESC 键关闭
  document.addEventListener('keydown', async (e) => {
    if (e.key === 'Escape') {
      await closeNotification();
    }
  });
  
  // 3秒自动关闭
  setTimeout(async () => {
    await closeNotification();
  }, 3000);
  
  // 触发抖动动画
  function triggerShake() {
    // 移除并重新添加 class，强制重启动画
    notificationBar.classList.remove('shake-animation');
    void notificationBar.offsetWidth; // 触发 reflow
    notificationBar.classList.add('shake-animation');
    
    // 动画结束后移除 class
    setTimeout(() => {
      notificationBar.classList.remove('shake-animation');
    }, 500);
  }
});

// 关闭通知条
async function closeNotification() {
  try {
    const window = getCurrentWindow();
    await window.hide();
    console.log("✅ 通知条已隐藏");
  } catch (error) {
    console.error('关闭失败:', error);
  }
}
