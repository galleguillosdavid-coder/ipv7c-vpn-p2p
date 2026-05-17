// Soul Vision v3.0 — main.js
// Tauri frontend: consume los comandos Rust via invoke() y /api/ del nodo Python

const { invoke } = window.__TAURI__.core;

// ── Estado global ──────────────────────────────────────────
let nodePort = 8765;
let snapshot = null;
let pollTimer = null;
let soulMapCtx = null;
let animFrame = null;

// ── DOM refs ───────────────────────────────────────────────
const portInput    = document.getElementById('port-input');
const btnConnect   = document.getElementById('btn-connect');
const btnToggle    = document.getElementById('btn-toggle');
const statusDot    = document.getElementById('status-dot');
const statusLabel  = document.getElementById('status-label');
const peersList    = document.getElementById('peers-list');
const logStream    = document.getElementById('log-stream');
const chatLog      = document.getElementById('chat-log');
const chatInput    = document.getElementById('chat-input');
const btnChat      = document.getElementById('btn-chat');
const canvas       = document.getElementById('soul-map');

// Metrics
const valDid     = document.getElementById('val-did');
const valHash    = document.getElementById('val-hash');
const valPeers   = document.getElementById('val-peers');
const valLatency = document.getElementById('val-latency');
const valProfile = document.getElementById('val-profile');
const valSos     = document.getElementById('val-sos');
const metricSos  = document.getElementById('metric-sos');

// ── Soul Map Canvas setup ──────────────────────────────────
function initCanvas() {
  const dpr = window.devicePixelRatio || 1;
  const rect = canvas.getBoundingClientRect();
  canvas.width  = rect.width  * dpr;
  canvas.height = rect.height * dpr;
  soulMapCtx = canvas.getContext('2d');
  soulMapCtx.scale(dpr, dpr);
  drawSoulMap([]);
}

function drawSoulMap(peers) {
  if (!soulMapCtx) return;
  const W = canvas.getBoundingClientRect().width;
  const H = canvas.getBoundingClientRect().height;
  const ctx = soulMapCtx;
  
  ctx.clearRect(0, 0, W, H);

  // Fondo con rejilla sutil
  ctx.strokeStyle = 'rgba(30,45,74,0.4)';
  ctx.lineWidth = 0.5;
  const grid = 40;
  for (let x = 0; x < W; x += grid) { ctx.beginPath(); ctx.moveTo(x,0); ctx.lineTo(x,H); ctx.stroke(); }
  for (let y = 0; y < H; y += grid) { ctx.beginPath(); ctx.moveTo(0,y); ctx.lineTo(W,y); ctx.stroke(); }

  const cx = W / 2;
  const cy = H / 2;
  const now = Date.now() / 1000;

  // Nodo central (este nodo)
  const outerR = 60 + 10 * Math.sin(now * 1.5);
  const grad = ctx.createRadialGradient(cx, cy, 0, cx, cy, outerR);
  grad.addColorStop(0,   'rgba(0,212,255,0.35)');
  grad.addColorStop(0.5, 'rgba(124,58,237,0.15)');
  grad.addColorStop(1,   'rgba(0,212,255,0)');
  ctx.beginPath();
  ctx.arc(cx, cy, outerR, 0, Math.PI * 2);
  ctx.fillStyle = grad;
  ctx.fill();

  ctx.beginPath();
  ctx.arc(cx, cy, 10, 0, Math.PI * 2);
  ctx.fillStyle = '#00d4ff';
  ctx.shadowColor = '#00d4ff';
  ctx.shadowBlur = 18;
  ctx.fill();
  ctx.shadowBlur = 0;

  // Etiqueta nodo local
  ctx.fillStyle = 'rgba(0,212,255,0.8)';
  ctx.font = '10px JetBrains Mono';
  ctx.textAlign = 'center';
  ctx.fillText('THIS NODE', cx, cy + 22);

  // Nodos pares en órbita
  if (peers.length > 0) {
    const orbitR = Math.min(W, H) * 0.32;
    peers.forEach((peer, i) => {
      const angle = (i / peers.length) * Math.PI * 2 - Math.PI / 2;
      const pulse = Math.sin(now * 2 + i) * 4;
      const px = cx + (orbitR + pulse) * Math.cos(angle);
      const py = cy + (orbitR + pulse) * Math.sin(angle);
      const isOnline = peer.online;

      // Línea de conexión
      ctx.beginPath();
      ctx.moveTo(cx, cy);
      ctx.lineTo(px, py);
      ctx.strokeStyle = isOnline
        ? `rgba(16,185,129,${0.2 + 0.15 * Math.sin(now * 3 + i)})`
        : 'rgba(239,68,68,0.15)';
      ctx.lineWidth = 1;
      ctx.setLineDash([4, 6]);
      ctx.stroke();
      ctx.setLineDash([]);

      // Nodo par
      ctx.beginPath();
      ctx.arc(px, py, 7, 0, Math.PI * 2);
      ctx.fillStyle = isOnline ? '#10b981' : '#ef4444';
      ctx.shadowColor  = isOnline ? '#10b981' : '#ef4444';
      ctx.shadowBlur   = 12;
      ctx.fill();
      ctx.shadowBlur = 0;

      // Alias
      const alias = (peer.alias || peer.did || 'Peer').slice(0, 10);
      ctx.fillStyle = isOnline ? 'rgba(16,185,129,0.9)' : 'rgba(239,68,68,0.7)';
      ctx.font = '9px JetBrains Mono';
      ctx.textAlign = 'center';
      ctx.fillText(alias, px, py + 18);
    });
  }

  animFrame = requestAnimationFrame(() => drawSoulMap(peers));
}

// ── Polling del snapshot ───────────────────────────────────
async function fetchSnapshot() {
  try {
    const data = await invoke('get_node_snapshot', { port: nodePort });
    snapshot = data;
    updateUI(data);
    setStatus('online', `Nodo activo · Puerto ${nodePort}`);
    btnToggle.disabled = false;
  } catch (err) {
    setStatus('offline', `Sin conexión (puerto ${nodePort})`);
    btnToggle.disabled = true;
    addLog(`[ERROR] ${err}`, 'error');
  }
}

function startPolling() {
  if (pollTimer) clearInterval(pollTimer);
  fetchSnapshot();
  pollTimer = setInterval(fetchSnapshot, 2000);
}

// ── Actualizar UI con datos del snapshot ──────────────────
function updateUI(data) {
  // Metrics
  valDid.textContent     = (data.node_did || '—').slice(0, 20) + '…';
  valHash.textContent    = (data.integrity_hash || '—').slice(0, 32) + '…';
  valPeers.textContent   = data.active_peers ?? '0';
  
  const latencies = Object.values(data.peers || {}).map(p => p.latency_ms).filter(Boolean);
  const avgLat = latencies.length ? (latencies.reduce((a,b) => a+b, 0) / latencies.length).toFixed(1) : '—';
  valLatency.textContent = avgLat !== '—' ? `${avgLat} ms` : '— ms';
  
  valProfile.textContent = data.network_type || '—';
  
  const sosActive = data.sos_mode_active === true;
  valSos.textContent = sosActive ? '⚠ ACTIVO' : 'Inactivo';
  metricSos.classList.toggle('active', sosActive);

  // Status dot
  if (sosActive) {
    statusDot.className = 'status-dot sos';
    statusLabel.textContent = 'SOS Mode — Malla Crítica';
  }

  // Peer cards
  const peersData = data.peers || {};
  const peersArr = Object.entries(peersData).map(([did, p]) => ({
    did, alias: p.alias || did.slice(0,8), online: p.online ?? false,
    latency: p.latency_ms
  }));

  peersList.innerHTML = peersArr.length === 0
    ? '<div class="empty-state">Sin pares detectados</div>'
    : peersArr.map(p => `
        <div class="peer-card">
          <div class="peer-alias">${p.alias}</div>
          <div class="peer-did">${p.did.slice(0,24)}…</div>
          <div class="peer-meta">
            <span class="peer-badge ${p.online ? 'online' : 'offline'}">${p.online ? 'Online' : 'Offline'}</span>
            <span class="peer-lat">${p.latency != null ? p.latency.toFixed(1) + ' ms' : '— ms'}</span>
          </div>
        </div>`).join('');

  // Soul Map
  if (animFrame) cancelAnimationFrame(animFrame);
  drawSoulMap(peersArr);

  // Logs
  const logs = data.recent_logs || [];
  logs.forEach(line => addLog(line));
}

// ── Helpers ────────────────────────────────────────────────
function setStatus(state, label) {
  statusDot.className = `status-dot ${state}`;
  statusLabel.textContent = label;
}

let logLines = [];
function addLog(line, type = '') {
  if (logLines.includes(line)) return;
  logLines = [...logLines.slice(-200), line];
  const div = document.createElement('div');
  div.className = `log-line ${type}`;
  div.textContent = line;
  logStream.appendChild(div);
  logStream.scrollTop = logStream.scrollHeight;
}

// ── Eventos ────────────────────────────────────────────────
btnConnect.addEventListener('click', () => {
  nodePort = parseInt(portInput.value, 10) || 8765;
  startPolling();
});

btnToggle.addEventListener('click', async () => {
  try {
    await invoke('toggle_node', { port: nodePort });
    addLog('[VPN] Toggle enviado al nodo', 'ok');
  } catch (e) {
    addLog(`[ERROR] Toggle: ${e}`, 'error');
  }
});

async function sendChat() {
  const msg = chatInput.value.trim();
  if (!msg) return;
  chatInput.value = '';
  appendChatMsg(msg, 'user');
  try {
    const reply = await invoke('ai_chat', { port: nodePort, message: msg });
    appendChatMsg(reply, 'ai');
  } catch (e) {
    appendChatMsg(`Error: ${e}`, 'ai');
  }
}

function appendChatMsg(text, who) {
  const div = document.createElement('div');
  div.className = `chat-msg ${who}`;
  div.textContent = text;
  chatLog.appendChild(div);
  chatLog.scrollTop = chatLog.scrollHeight;
}

btnChat.addEventListener('click', sendChat);
chatInput.addEventListener('keydown', e => { if (e.key === 'Enter') sendChat(); });

window.addEventListener('resize', () => {
  if (animFrame) cancelAnimationFrame(animFrame);
  initCanvas();
  drawSoulMap(snapshot ? Object.values(snapshot.peers || {}) : []);
});

// ── Init ───────────────────────────────────────────────────
initCanvas();
startPolling();
appendChatMsg('IPv7C Soul Vision v3.0 iniciado. Conectando al nodo soberano…', 'ai');
