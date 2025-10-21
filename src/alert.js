const { getCurrentWindow } = window.__TAURI__.window;

window.addEventListener("DOMContentLoaded", () => {
  console.log("通知条已加载");
  
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
