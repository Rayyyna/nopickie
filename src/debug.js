const { event: { listen } } = window.__TAURI__;

const videoImg = document.getElementById('video');
const statusEl = document.getElementById('status-indicator');
const statCountEl = document.getElementById('stat-count');
const statDurationEl = document.getElementById('stat-duration');

function setStatus(type) {
  statusEl.classList.remove('normal', 'warning', 'detected');
  if (type === 'warning') {
    statusEl.textContent = 'WARNING';
    statusEl.classList.add('warning');
  } else if (type === 'scratch_detected' || type === 'detected') {
    statusEl.textContent = 'DETECTED';
    statusEl.classList.add('detected');
  } else {
    statusEl.textContent = 'NORMAL';
    statusEl.classList.add('normal');
  }
}

async function init() {
  // 帧事件：debug_frame { image_b64, status, count, duration }
  await listen('debug_frame', ({ payload }) => {
    try {
      const p = typeof payload === 'string' ? JSON.parse(payload) : payload;
      if (p.image_b64) {
        videoImg.src = `data:image/jpeg;base64,${p.image_b64}`;
      }
      if (p.status) setStatus(p.status);
      if (typeof p.count === 'number') statCountEl.textContent = `Count: ${p.count}`;
      if (typeof p.duration === 'number') statDurationEl.textContent = `Duration: ${p.duration.toFixed(1)}s`;
    } catch (e) {
      console.error('debug_frame parse error', e);
    }
  });

  // 状态事件兜底
  await listen('warning', ({ payload }) => setStatus('warning'));
  await listen('scratch_detected', ({ payload }) => setStatus('scratch_detected'));
}

window.addEventListener('DOMContentLoaded', init);



