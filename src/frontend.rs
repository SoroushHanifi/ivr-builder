// src/frontend.rs

pub const DASHBOARD_HTML: &str = r###"<!DOCTYPE html>
<html lang="fa" dir="rtl">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>IVR Flow Builder</title>
<link href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500;600&family=Vazirmatn:wght@300;400;500;600;700&display=swap" rel="stylesheet">
<style>
:root {
  --bg:        #0a0e17;
  --surface:   #111827;
  --surface2:  #1a2235;
  --border:    #1f2d42;
  --border2:   #2a3f5f;
  --text:      #c9d8ef;
  --text2:     #6b8ab0;
  --text3:     #3a5070;
  --accent:    #3b82f6;
  --accent2:   #1d4ed8;
  --green:     #10b981;
  --yellow:    #f59e0b;
  --red:       #ef4444;
  --purple:    #8b5cf6;
  --cyan:      #06b6d4;
  --orange:    #f97316;

  /* node type colors */
  --c-menu:    #3b82f6;
  --c-play:    #10b981;
  --c-record:  #f59e0b;
  --c-digit:   #8b5cf6;
  --c-connect: #f97316;
  --c-hangup:  #ef4444;
}

*, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }
body { font-family: 'Vazirmatn', sans-serif; background: var(--bg); color: var(--text);
       height: 100vh; overflow: hidden; display: flex; flex-direction: column; }

/* ─── Header ─── */
header {
  background: var(--surface);
  border-bottom: 1px solid var(--border);
  height: 52px;
  display: flex;
  align-items: center;
  padding: 0 16px;
  gap: 16px;
  flex-shrink: 0;
  z-index: 100;
}
.logo { font-family: 'IBM Plex Mono', monospace; font-size: 13px; font-weight: 600;
        color: var(--accent); display: flex; align-items: center; gap: 8px; }
.logo-icon { font-size: 18px; }
.header-sep { width: 1px; height: 24px; background: var(--border); }
.flow-selector { display: flex; align-items: center; gap: 8px; flex: 1; }
.flow-select {
  background: var(--surface2); border: 1px solid var(--border2);
  color: var(--text); border-radius: 6px; padding: 6px 12px;
  font-family: 'Vazirmatn', sans-serif; font-size: 13px; min-width: 220px; cursor: pointer;
}
.flow-select:focus { outline: none; border-color: var(--accent); }
.btn {
  background: var(--surface2); border: 1px solid var(--border2);
  color: var(--text); border-radius: 6px; padding: 6px 14px;
  font-family: 'Vazirmatn', sans-serif; font-size: 12px; cursor: pointer;
  transition: all .15s; white-space: nowrap; display: flex; align-items: center; gap: 6px;
}
.btn:hover { background: var(--border2); border-color: var(--accent); color: white; }
.btn.primary { background: var(--accent); border-color: var(--accent); color: white; }
.btn.primary:hover { background: var(--accent2); }
.btn.danger { border-color: var(--red); color: var(--red); }
.btn.danger:hover { background: var(--red); color: white; }
.btn.success { border-color: var(--green); color: var(--green); }
.btn.success:hover { background: var(--green); color: white; }

/* ─── Layout ─── */
.workspace {
  display: flex;
  flex: 1;
  overflow: hidden;
}

/* ─── Left Sidebar: Node Palette ─── */
.sidebar-left {
  width: 200px;
  background: var(--surface);
  border-left: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  overflow-y: auto;
}
.sidebar-section { padding: 12px; }
.sidebar-title {
  font-size: 10px; font-weight: 600; text-transform: uppercase;
  letter-spacing: 1.5px; color: var(--text3); margin-bottom: 10px;
  font-family: 'IBM Plex Mono', monospace;
}
.node-palette-item {
  display: flex; align-items: center; gap: 8px;
  padding: 8px 10px; border-radius: 6px;
  border: 1px solid var(--border);
  background: var(--surface2);
  cursor: grab; margin-bottom: 6px;
  font-size: 12px; font-weight: 500;
  transition: all .15s; user-select: none;
}
.node-palette-item:hover { border-color: var(--border2); background: var(--surface2); transform: translateX(-2px); }
.node-palette-item:active { cursor: grabbing; }
.node-dot { width: 8px; height: 8px; border-radius: 50%; flex-shrink: 0; }

/* ─── Canvas ─── */
.canvas-wrap {
  flex: 1;
  position: relative;
  overflow: hidden;
  background: var(--bg);
  background-image:
    radial-gradient(circle, #1a2235 1px, transparent 1px);
  background-size: 24px 24px;
}
#canvas-svg {
  position: absolute; top: 0; left: 0;
  width: 100%; height: 100%;
  pointer-events: none;
}
#canvas-svg .edge { pointer-events: stroke; cursor: pointer; }

#node-container {
  position: absolute; top: 0; left: 0;
  width: 100%; height: 100%;
}

.canvas-hint {
  position: absolute; bottom: 16px; left: 50%; transform: translateX(-50%);
  font-size: 11px; color: var(--text3); font-family: 'IBM Plex Mono', monospace;
  pointer-events: none;
}

/* ─── Flow Node ─── */
.flow-node {
  position: absolute;
  min-width: 160px;
  background: var(--surface);
  border: 1.5px solid var(--border2);
  border-radius: 10px;
  cursor: move;
  user-select: none;
  transition: box-shadow .15s, border-color .15s;
  z-index: 10;
}
.flow-node.selected {
  border-color: var(--accent);
  box-shadow: 0 0 0 3px rgba(59,130,246,.2), 0 8px 24px rgba(0,0,0,.4);
  z-index: 20;
}
.flow-node:hover { box-shadow: 0 4px 20px rgba(0,0,0,.3); }
.flow-node.entry-node { border-color: var(--green); }
.flow-node.entry-node .node-header { background: rgba(16,185,129,.1); }

.node-header {
  padding: 8px 10px;
  border-radius: 8px 8px 0 0;
  display: flex; align-items: center; gap: 8px;
  border-bottom: 1px solid var(--border);
}
.node-type-badge {
  width: 10px; height: 10px; border-radius: 3px; flex-shrink: 0;
}
.node-title { font-size: 12px; font-weight: 600; color: var(--text); flex: 1;
              white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.node-type-label {
  font-size: 9px; font-family: 'IBM Plex Mono', monospace;
  color: var(--text3); text-transform: uppercase; letter-spacing: .5px;
}
.node-entry-badge {
  background: var(--green); color: white; font-size: 9px;
  padding: 1px 5px; border-radius: 3px; font-family: 'IBM Plex Mono', monospace;
}

.node-body { padding: 8px 10px; font-size: 11px; color: var(--text2); }
.node-body .config-preview { font-family: 'IBM Plex Mono', monospace; font-size: 10px;
                              color: var(--text3); margin-top: 4px; }

/* پورت‌های اتصال */
.node-ports {
  padding: 0 10px 8px;
  display: flex; flex-wrap: wrap; gap: 4px;
}
.port-out {
  display: inline-flex; align-items: center; gap: 4px;
  background: var(--surface2); border: 1px solid var(--border2);
  border-radius: 4px; padding: 3px 7px;
  font-size: 10px; font-family: 'IBM Plex Mono', monospace;
  cursor: crosshair; color: var(--text2);
  transition: all .15s;
}
.port-out:hover { border-color: var(--accent); color: var(--accent); }
.port-out.connected { border-color: var(--green); color: var(--green); }
.port-dot { width: 6px; height: 6px; border-radius: 50%; background: currentColor; }

/* ─── Right Sidebar: Properties ─── */
.sidebar-right {
  width: 280px;
  background: var(--surface);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  overflow-y: auto;
}
.props-header {
  padding: 14px; border-bottom: 1px solid var(--border);
  font-size: 12px; font-weight: 600; color: var(--text2);
  font-family: 'IBM Plex Mono', monospace; text-transform: uppercase;
  letter-spacing: 1px;
}
.props-body { padding: 14px; flex: 1; }
.prop-group { margin-bottom: 16px; }
.prop-label {
  font-size: 10px; font-weight: 600; text-transform: uppercase;
  letter-spacing: 1px; color: var(--text3); margin-bottom: 6px;
  font-family: 'IBM Plex Mono', monospace;
}
.prop-input {
  width: 100%; background: var(--surface2); border: 1px solid var(--border2);
  color: var(--text); border-radius: 6px; padding: 7px 10px;
  font-family: 'Vazirmatn', sans-serif; font-size: 12px;
  transition: border-color .15s;
}
.prop-input:focus { outline: none; border-color: var(--accent); }
textarea.prop-input { resize: vertical; min-height: 60px; }
select.prop-input { cursor: pointer; }
.prop-checkbox { display: flex; align-items: center; gap: 8px; font-size: 12px; cursor: pointer; }
.prop-checkbox input { accent-color: var(--accent); }
.prop-number { display: flex; align-items: center; gap: 6px; }
.prop-number .prop-input { flex: 1; }
.prop-number .unit { font-size: 10px; color: var(--text3); white-space: nowrap; }

.transitions-list { margin-top: 4px; }
.transition-item {
  display: flex; align-items: center; gap: 6px; margin-bottom: 6px;
  padding: 6px 8px; background: var(--surface2); border-radius: 6px;
  border: 1px solid var(--border);
}
.transition-trigger {
  font-family: 'IBM Plex Mono', monospace; font-size: 11px;
  background: var(--border2); padding: 2px 6px; border-radius: 4px; min-width: 28px;
  text-align: center; font-weight: 600;
}
.transition-arrow { color: var(--text3); font-size: 12px; }
.transition-target { font-size: 11px; color: var(--text2); flex: 1; }
.transition-del { cursor: pointer; color: var(--text3); font-size: 14px; padding: 0 2px; }
.transition-del:hover { color: var(--red); }

.add-transition-row { display: flex; gap: 6px; margin-top: 8px; }
.add-transition-row .prop-input { flex: 1; font-size: 11px; }

/* ─── Modals ─── */
.modal-backdrop {
  position: fixed; inset: 0; background: rgba(0,0,0,.7);
  display: flex; align-items: center; justify-content: center;
  z-index: 1000;
}
.modal {
  background: var(--surface); border: 1px solid var(--border2);
  border-radius: 12px; padding: 24px; min-width: 360px; max-width: 500px; width: 90%;
  max-height: 80vh; overflow-y: auto;
}
.modal-title { font-size: 15px; font-weight: 600; margin-bottom: 20px; color: var(--text); }
.modal-footer { display: flex; gap: 8px; justify-content: flex-end; margin-top: 20px; }

/* ─── Toast ─── */
#toast {
  position: fixed; bottom: 20px; right: 20px;
  background: var(--surface); border: 1px solid var(--border2);
  border-radius: 8px; padding: 10px 16px; font-size: 13px;
  z-index: 9999; opacity: 0; transform: translateY(10px);
  transition: all .25s; pointer-events: none;
}
#toast.show { opacity: 1; transform: translateY(0); }
#toast.ok   { border-color: var(--green); color: var(--green); }
#toast.err  { border-color: var(--red);   color: var(--red); }

/* ─── Empty state ─── */
.empty-canvas {
  position: absolute; top: 50%; left: 50%; transform: translate(-50%,-50%);
  text-align: center; pointer-events: none; color: var(--text3);
}
.empty-canvas .big { font-size: 48px; margin-bottom: 12px; }
.empty-canvas p { font-size: 13px; }

/* ─── Zoom controls ─── */
.zoom-controls {
  position: absolute; bottom: 16px; right: 16px;
  display: flex; flex-direction: column; gap: 4px;
}
.zoom-btn {
  width: 32px; height: 32px; background: var(--surface);
  border: 1px solid var(--border2); border-radius: 6px;
  display: flex; align-items: center; justify-content: center;
  cursor: pointer; font-size: 16px; color: var(--text2);
  transition: all .15s;
}
.zoom-btn:hover { border-color: var(--accent); color: var(--accent); }
</style>
</head>
<body>

<!-- ─── Header ─── -->
<header>
  <div class="logo"><span class="logo-icon">⬡</span> IVR Builder</div>
  <div class="header-sep"></div>
  <div class="flow-selector">
    <select class="flow-select" id="flow-select" onchange="loadFlow(this.value)">
      <option value="">— انتخاب Flow —</option>
    </select>
    <button class="btn primary" onclick="showCreateFlow()">＋ Flow جدید</button>
    <button class="btn" onclick="renameFlow()" id="btn-rename">✏️ نام</button>
    <button class="btn success" onclick="setEntryNode()" id="btn-entry">⚡ ورودی</button>
    <button class="btn danger" onclick="deleteFlow()" id="btn-del-flow">🗑 حذف</button>
  </div>
  <div class="header-sep"></div>
  <button class="btn" onclick="exportFlow()">📤 Export JSON</button>
</header>

<!-- ─── Workspace ─── -->
<div class="workspace">

  <!-- Left: Palette -->
  <div class="sidebar-left">
    <div class="sidebar-section">
      <div class="sidebar-title">گره‌ها</div>

      <div class="node-palette-item" draggable="true" data-type="menu"
           ondragstart="paletteDrag(event,'menu')">
        <div class="node-dot" style="background:var(--c-menu)"></div>
        منو
      </div>
      <div class="node-palette-item" draggable="true" data-type="play_audio"
           ondragstart="paletteDrag(event,'play_audio')">
        <div class="node-dot" style="background:var(--c-play)"></div>
        پخش صوت
      </div>
      <div class="node-palette-item" draggable="true" data-type="record_audio"
           ondragstart="paletteDrag(event,'record_audio')">
        <div class="node-dot" style="background:var(--c-record)"></div>
        ضبط صوت
      </div>
      <div class="node-palette-item" draggable="true" data-type="receive_digit"
           ondragstart="paletteDrag(event,'receive_digit')">
        <div class="node-dot" style="background:var(--c-digit)"></div>
        دریافت دیجیت
      </div>
      <div class="node-palette-item" draggable="true" data-type="connect_call"
           ondragstart="paletteDrag(event,'connect_call')">
        <div class="node-dot" style="background:var(--c-connect)"></div>
        اتصال تماس
      </div>
      <div class="node-palette-item" draggable="true" data-type="hangup"
           ondragstart="paletteDrag(event,'hangup')">
        <div class="node-dot" style="background:var(--c-hangup)"></div>
        قطع تماس
      </div>
    </div>

    <div class="sidebar-section" style="border-top:1px solid var(--border);">
      <div class="sidebar-title">راهنما</div>
      <div style="font-size:11px;color:var(--text3);line-height:1.7">
        • کشیدن گره به canvas<br>
        • کلیک روی پورت → اتصال<br>
        • کلیک روی گره → ویرایش<br>
        • Del → حذف انتخابی<br>
        • Ctrl+Z → بازگشت (TODO)
      </div>
    </div>
  </div>

  <!-- Center: Canvas -->
  <div class="canvas-wrap" id="canvas-wrap"
       ondragover="event.preventDefault()"
       ondrop="canvasDrop(event)"
       onclick="canvasClick(event)">

    <svg id="canvas-svg" xmlns="http://www.w3.org/2000/svg"></svg>
    <div id="node-container"></div>

    <div class="empty-canvas" id="empty-hint">
      <div class="big">⬡</div>
      <p>یک flow انتخاب کنید<br>سپس گره‌ها را بکشید</p>
    </div>

    <div class="zoom-controls">
      <div class="zoom-btn" onclick="zoom(1.2)">＋</div>
      <div class="zoom-btn" onclick="zoom(0.8)">－</div>
      <div class="zoom-btn" onclick="resetZoom()">↺</div>
    </div>

    <div class="canvas-hint">کلیک راست روی گره برای منوی سریع</div>
  </div>

  <!-- Right: Properties -->
  <div class="sidebar-right">
    <div class="props-header">⚙ Properties</div>
    <div class="props-body" id="props-panel">
      <div style="color:var(--text3);font-size:12px;text-align:center;margin-top:40px">
        یک گره انتخاب کنید
      </div>
    </div>
  </div>

</div>

<!-- ─── Create Flow Modal ─── -->
<div class="modal-backdrop" id="modal-create-flow" style="display:none" onclick="if(event.target===this)this.style.display='none'">
  <div class="modal">
    <div class="modal-title">➕ Flow جدید</div>
    <div class="prop-group">
      <div class="prop-label">نام Flow</div>
      <input class="prop-input" id="new-flow-name" placeholder="مثلاً: IVR اصلی" />
    </div>
    <div class="prop-group">
      <div class="prop-label">توضیحات (اختیاری)</div>
      <textarea class="prop-input" id="new-flow-desc" rows="2" placeholder="توضیح کوتاه..."></textarea>
    </div>
    <div class="modal-footer">
      <button class="btn" onclick="document.getElementById('modal-create-flow').style.display='none'">انصراف</button>
      <button class="btn primary" onclick="createFlow()">ایجاد</button>
    </div>
  </div>
</div>

<!-- ─── Context Menu ─── -->
<div id="ctx-menu" style="display:none;position:fixed;z-index:500;background:var(--surface);
     border:1px solid var(--border2);border-radius:8px;padding:4px;min-width:150px;box-shadow:0 8px 24px rgba(0,0,0,.5)">
</div>

<!-- ─── Toast ─── -->
<div id="toast"></div>

<script>
// ════════════════════════════════════════════
//   STATE
// ════════════════════════════════════════════
let currentFlowId = null;
let flowData = { nodes: [], flow: {} };
let extensions = [];
let selectedNodeId = null;
let connectingFrom = null;   // { nodeId, trigger }
let scale = 1;
let pan = { x: 0, y: 0 };
let dragging = null;         // { nodeId, startMX, startMY, startNX, startNY }

const NODE_COLORS = {
  menu:          'var(--c-menu)',
  play_audio:    'var(--c-play)',
  record_audio:  'var(--c-record)',
  receive_digit: 'var(--c-digit)',
  connect_call:  'var(--c-connect)',
  hangup:        'var(--c-hangup)',
};
const NODE_LABELS = {
  menu:          'منو',
  play_audio:    'پخش صوت',
  record_audio:  'ضبط صوت',
  receive_digit: 'دریافت دیجیت',
  connect_call:  'اتصال تماس',
  hangup:        'قطع تماس',
};

// ════════════════════════════════════════════
//   BOOT
// ════════════════════════════════════════════
async function boot() {
  await loadFlowList();
  await loadExtensions();
  document.addEventListener('keydown', onKeyDown);
  document.addEventListener('mousemove', onMouseMove);
  document.addEventListener('mouseup', onMouseUp);
  document.getElementById('ctx-menu').addEventListener('click', e => e.stopPropagation());
  document.addEventListener('click', () => document.getElementById('ctx-menu').style.display = 'none');
}

async function loadFlowList() {
  const r = await api('GET', '/api/flows');
  const sel = document.getElementById('flow-select');
  sel.innerHTML = '<option value="">— انتخاب Flow —</option>';
  (r.data || []).forEach(f => {
    const o = document.createElement('option');
    o.value = f.id; o.textContent = f.name;
    sel.appendChild(o);
  });
  if (currentFlowId) sel.value = currentFlowId;
}

async function loadExtensions() {
  const r = await api('GET', '/api/extensions');
  extensions = r.data || [];
}

// ════════════════════════════════════════════
//   FLOW MANAGEMENT
// ════════════════════════════════════════════
function showCreateFlow() {
  document.getElementById('new-flow-name').value = '';
  document.getElementById('new-flow-desc').value = '';
  document.getElementById('modal-create-flow').style.display = 'flex';
  setTimeout(() => document.getElementById('new-flow-name').focus(), 50);
}

async function createFlow() {
  const name = document.getElementById('new-flow-name').value.trim();
  if (!name) { toast('نام Flow الزامی است', 'err'); return; }
  const r = await api('POST', '/api/flows', {
    name,
    description: document.getElementById('new-flow-desc').value.trim() || null
  });
  document.getElementById('modal-create-flow').style.display = 'none';
  await loadFlowList();
  await loadFlow(r.data.id);
  toast('Flow ایجاد شد ✓', 'ok');
}

async function renameFlow() {
  if (!currentFlowId) return;
  const name = prompt('نام جدید:', flowData.flow.name);
  if (!name) return;
  await api('PUT', `/api/flows/${currentFlowId}`, { name });
  await loadFlowList();
  flowData.flow.name = name;
  toast('نام آپدیت شد ✓', 'ok');
}

async function deleteFlow() {
  if (!currentFlowId) return;
  if (!confirm(`حذف flow "${flowData.flow.name}"?`)) return;
  await api('DELETE', `/api/flows/${currentFlowId}`);
  currentFlowId = null;
  flowData = { nodes: [], flow: {} };
  renderCanvas();
  await loadFlowList();
  document.getElementById('flow-select').value = '';
  toast('Flow حذف شد', 'ok');
}

async function setEntryNode() {
  if (!selectedNodeId || !currentFlowId) { toast('ابتدا یک گره انتخاب کنید', 'err'); return; }
  await api('PUT', `/api/flows/${currentFlowId}`, { entry_node_id: selectedNodeId });
  flowData.flow.entry_node_id = selectedNodeId;
  renderCanvas();
  toast('گره ورودی تنظیم شد ✓', 'ok');
}

async function loadFlow(id) {
  if (!id) { currentFlowId = null; flowData = { nodes: [], flow: {} }; renderCanvas(); return; }
  currentFlowId = id;
  const r = await api('GET', `/api/flows/${id}`);
  flowData = r.data || { nodes: [], flow: {} };
  selectedNodeId = null;
  renderCanvas();
  renderProps();
  document.getElementById('empty-hint').style.display =
    flowData.nodes.length ? 'none' : 'block';
}

// ════════════════════════════════════════════
//   NODE CREATION
// ════════════════════════════════════════════
function paletteDrag(e, type) {
  e.dataTransfer.setData('nodeType', type);
}

async function canvasDrop(e) {
  if (!currentFlowId) { toast('ابتدا یک flow انتخاب کنید', 'err'); return; }
  const type = e.dataTransfer.getData('nodeType');
  if (!type) return;

  const wrap = document.getElementById('canvas-wrap');
  const rect = wrap.getBoundingClientRect();
  const x = (e.clientX - rect.left - pan.x) / scale;
  const y = (e.clientY - rect.top  - pan.y) / scale;

  const r = await api('POST', `/api/flows/${currentFlowId}/nodes`, {
    node_type: type,
    position_x: x,
    position_y: y,
  });

  flowData.nodes.push(r.data);
  document.getElementById('empty-hint').style.display = 'none';
  renderCanvas();
  selectNode(r.data.id);
  toast(`گره "${NODE_LABELS[type]}" اضافه شد`, 'ok');
}

// ════════════════════════════════════════════
//   CANVAS RENDER
// ════════════════════════════════════════════
function renderCanvas() {
  renderEdges();
  renderNodes();
}

function renderNodes() {
  const container = document.getElementById('node-container');
  const existing = {};
  container.querySelectorAll('.flow-node').forEach(el => {
    existing[el.dataset.id] = true;
  });

  // remove stale
  container.querySelectorAll('.flow-node').forEach(el => {
    if (!flowData.nodes.find(n => n.id === el.dataset.id)) el.remove();
  });

  flowData.nodes.forEach(node => {
    let el = container.querySelector(`[data-id="${node.id}"]`);
    if (!el) {
      el = createNodeEl(node);
      container.appendChild(el);
    } else {
      // re-render inner content
      el.innerHTML = nodeInnerHTML(node);
      bindNodePorts(el, node);
    }
    const nx = node.position_x * scale + pan.x;
    const ny = node.position_y * scale + pan.y;
    el.style.left = nx + 'px';
    el.style.top  = ny + 'px';
    el.style.transform = `scale(${scale})`;
    el.style.transformOrigin = 'top left';
    el.classList.toggle('selected', node.id === selectedNodeId);
    el.classList.toggle('entry-node', node.id === flowData.flow?.entry_node_id);
  });
}

function createNodeEl(node) {
  const el = document.createElement('div');
  el.className = 'flow-node';
  el.dataset.id = node.id;
  el.innerHTML = nodeInnerHTML(node);
  el.addEventListener('mousedown', e => startDrag(e, node.id));
  el.addEventListener('contextmenu', e => showCtxMenu(e, node.id));
  bindNodePorts(el, node);
  return el;
}

function nodeInnerHTML(node) {
  const color = NODE_COLORS[node.node_type] || 'var(--text3)';
  const label = NODE_LABELS[node.node_type] || node.node_type;
  const isEntry = node.id === flowData.flow?.entry_node_id;
  const preview = configPreview(node);

  // خروجی‌ها
  const triggers = getNodeTriggers(node);
  const portsHTML = triggers.map(t => {
    const trans = node.transitions?.find(tr => tr.trigger === t.key);
    const connected = trans && trans.to_node_id;
    return `<span class="port-out ${connected ? 'connected' : ''}"
              data-from="${node.id}" data-trigger="${t.key}"
              title="${t.label}">
              <span class="port-dot"></span>${t.label}
            </span>`;
  }).join('');

  return `
    <div class="node-header">
      <div class="node-type-badge" style="background:${color}"></div>
      <div>
        <div class="node-title">${escHtml(node.name)}</div>
        <div class="node-type-label">${label}</div>
      </div>
      ${isEntry ? '<span class="node-entry-badge">ENTRY</span>' : ''}
    </div>
    ${preview ? `<div class="node-body"><div class="config-preview">${escHtml(preview)}</div></div>` : ''}
    <div class="node-ports">${portsHTML}</div>
  `;
}

function bindNodePorts(el, node) {
  el.querySelectorAll('.port-out').forEach(port => {
    port.addEventListener('click', e => {
      e.stopPropagation();
      startConnection(node.id, port.dataset.trigger);
    });
  });
}

function getNodeTriggers(node) {
  switch (node.node_type) {
    case 'menu': return [
      { key: '1', label: '1' }, { key: '2', label: '2' },
      { key: '3', label: '3' }, { key: '4', label: '4' },
      { key: '5', label: '5' }, { key: '0', label: '0' },
      { key: '*', label: '✱' }, { key: '#', label: '#' },
      { key: 'timeout', label: '⏱' }, { key: 'default', label: '?' }
    ];
    case 'receive_digit': return [
      { key: 'received', label: '✓' },
      { key: 'timeout', label: '⏱' },
    ];
    case 'play_audio':    return [{ key: 'done', label: '→' }];
    case 'record_audio':  return [{ key: 'done', label: '→' }];
    case 'connect_call':  return [
      { key: 'answered', label: '📞' },
      { key: 'busy', label: '🔴' },
      { key: 'no_answer', label: '⏱' },
    ];
    case 'hangup': return [];
    default:       return [{ key: 'default', label: '→' }];
  }
}

function configPreview(node) {
  const c = node.config;
  if (!c) return '';
  switch (node.node_type) {
    case 'menu':
      return c.prompt_text?.substring(0, 35) || '';
    case 'play_audio':
      return c.text_to_speech?.substring(0, 35) || c.audio_file || '';
    case 'record_audio':
      return `⏱ ${c.max_duration_secs}s`;
    case 'receive_digit':
      return c.prompt_text?.substring(0, 35) || '';
    case 'connect_call':
      return `→ ${c.extension}`;
    case 'hangup':
      return c.play_message?.substring(0, 35) || '';
    default: return '';
  }
}

// ─── Edges ───
function renderEdges() {
  const svg = document.getElementById('canvas-svg');
  svg.innerHTML = `
    <defs>
      <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5"
              markerWidth="6" markerHeight="6" orient="auto">
        <path d="M0,0 L10,5 L0,10 z" fill="#3b82f6" opacity=".7"/>
      </marker>
    </defs>
  `;

  flowData.nodes.forEach(fromNode => {
    (fromNode.transitions || []).forEach(tr => {
      if (!tr.to_node_id) return;
      const toNode = flowData.nodes.find(n => n.id === tr.to_node_id);
      if (!toNode) return;

      const fromEl = document.querySelector(`[data-id="${fromNode.id}"]`);
      const toEl   = document.querySelector(`[data-id="${toNode.id}"]`);
      if (!fromEl || !toEl) return;

      const fw = fromEl.offsetWidth * scale;
      const fh = fromEl.offsetHeight * scale;
      const tw = toEl.offsetWidth  * scale;
      const th = toEl.offsetHeight * scale;

      const fx = fromNode.position_x * scale + pan.x + fw / 2;
      const fy = fromNode.position_y * scale + pan.y + fh / 2;
      const tx = toNode.position_x   * scale + pan.x + tw / 2;
      const ty = toNode.position_y   * scale + pan.y + th / 2;

      const cx = (fx + tx) / 2;
      const path = `M${fx},${fy} C${cx},${fy} ${cx},${ty} ${tx},${ty}`;

      const label = tr.trigger || '';
      const midX = (fx + tx) / 2;
      const midY = (fy + ty) / 2;

      svg.innerHTML += `
        <path class="edge" d="${path}" stroke="#3b82f6" stroke-width="1.5"
              stroke-opacity=".5" fill="none" marker-end="url(#arrow)"
              data-tr="${tr.id}" onclick="edgeClick('${tr.id}','${fromNode.id}')"/>
        ${label ? `<text x="${midX}" y="${midY - 6}" fill="#3b82f6" font-size="10"
                         font-family="IBM Plex Mono,monospace" text-anchor="middle"
                         opacity=".8">${escHtml(label)}</text>` : ''}
      `;
    });
  });

  // connecting line preview (while drawing)
  if (connectingFrom) {
    svg.innerHTML += `<line id="connect-preview" stroke="#f59e0b" stroke-width="1.5"
                           stroke-dasharray="5,3" opacity=".7"/>`;
  }
}

// ════════════════════════════════════════════
//   DRAG (move nodes)
// ════════════════════════════════════════════
function startDrag(e, nodeId) {
  if (e.target.classList.contains('port-out')) return;
  if (e.button !== 0) return;
  e.stopPropagation();
  selectNode(nodeId);

  const node = flowData.nodes.find(n => n.id === nodeId);
  dragging = {
    nodeId,
    startMX: e.clientX,
    startMY: e.clientY,
    startNX: node.position_x,
    startNY: node.position_y,
  };
}

function onMouseMove(e) {
  if (dragging) {
    const dx = (e.clientX - dragging.startMX) / scale;
    const dy = (e.clientY - dragging.startMY) / scale;
    const node = flowData.nodes.find(n => n.id === dragging.nodeId);
    if (node) {
      node.position_x = dragging.startNX + dx;
      node.position_y = dragging.startNY + dy;
      const el = document.querySelector(`[data-id="${node.id}"]`);
      if (el) {
        el.style.left = node.position_x * scale + pan.x + 'px';
        el.style.top  = node.position_y * scale + pan.y + 'px';
      }
      renderEdges();
    }
  }

  // connection preview
  if (connectingFrom) {
    const line = document.getElementById('connect-preview');
    if (line) {
      const fromNode = flowData.nodes.find(n => n.id === connectingFrom.nodeId);
      const fromEl   = document.querySelector(`[data-id="${fromNode.id}"]`);
      if (fromEl) {
        const fw = fromEl.offsetWidth * scale;
        const fh = fromEl.offsetHeight * scale;
        const fx = fromNode.position_x * scale + pan.x + fw / 2;
        const fy = fromNode.position_y * scale + pan.y + fh / 2;
        const wrap = document.getElementById('canvas-wrap');
        const rect = wrap.getBoundingClientRect();
        line.setAttribute('x1', fx); line.setAttribute('y1', fy);
        line.setAttribute('x2', e.clientX - rect.left);
        line.setAttribute('y2', e.clientY - rect.top);
      }
    }
  }
}

async function onMouseUp(e) {
  if (dragging) {
    const node = flowData.nodes.find(n => n.id === dragging.nodeId);
    if (node) {
      // save position to server
      await api('PUT', `/api/nodes/${node.id}`, {
        position_x: node.position_x,
        position_y: node.position_y,
      });
    }
    dragging = null;
  }
}

// ════════════════════════════════════════════
//   CONNECTION
// ════════════════════════════════════════════
function startConnection(nodeId, trigger) {
  if (connectingFrom) {
    // cancel
    connectingFrom = null;
    renderEdges();
    return;
  }
  connectingFrom = { nodeId, trigger };
  toast(`انتخاب مقصد برای "${trigger}"...`, 'ok');
  renderEdges();
}

async function canvasClick(e) {
  // if clicking on a node, handle there
  if (e.target.closest('.flow-node')) {
    const nodeEl = e.target.closest('.flow-node');
    const nodeId = nodeEl.dataset.id;

    if (connectingFrom && connectingFrom.nodeId !== nodeId) {
      // make connection
      const r = await api('POST', `/api/nodes/${connectingFrom.nodeId}/transitions`, {
        to_node_id: nodeId,
        trigger: connectingFrom.trigger,
        label: connectingFrom.trigger,
      });

      // update local state
      const fromNode = flowData.nodes.find(n => n.id === connectingFrom.nodeId);
      if (fromNode) {
        if (!fromNode.transitions) fromNode.transitions = [];
        const existing = fromNode.transitions.findIndex(t => t.trigger === connectingFrom.trigger);
        if (existing >= 0) fromNode.transitions[existing] = r.data;
        else fromNode.transitions.push(r.data);
      }

      connectingFrom = null;
      renderCanvas();
      renderProps();
      toast('اتصال ایجاد شد ✓', 'ok');
    } else if (!connectingFrom) {
      selectNode(nodeId);
    }
    return;
  }

  // click on empty canvas
  if (connectingFrom) {
    connectingFrom = null;
    renderEdges();
  } else {
    selectNode(null);
  }
}

async function edgeClick(trId, fromNodeId) {
  if (!confirm('حذف این اتصال؟')) return;
  await api('DELETE', `/api/transitions/${trId}`);
  const fromNode = flowData.nodes.find(n => n.id === fromNodeId);
  if (fromNode) {
    fromNode.transitions = fromNode.transitions.filter(t => t.id !== trId);
  }
  renderCanvas();
  renderProps();
  toast('اتصال حذف شد', 'ok');
}

// ════════════════════════════════════════════
//   SELECTION & PROPERTIES
// ════════════════════════════════════════════
function selectNode(id) {
  selectedNodeId = id;
  document.querySelectorAll('.flow-node').forEach(el =>
    el.classList.toggle('selected', el.dataset.id === id));
  renderProps();
}

function renderProps() {
  const panel = document.getElementById('props-panel');
  if (!selectedNodeId) {
    panel.innerHTML = '<div style="color:var(--text3);font-size:12px;text-align:center;margin-top:40px">یک گره انتخاب کنید</div>';
    return;
  }
  const node = flowData.nodes.find(n => n.id === selectedNodeId);
  if (!node) { panel.innerHTML = ''; return; }

  panel.innerHTML = buildPropsHTML(node);
}

function buildPropsHTML(node) {
  const c = node.config;
  let html = `
    <div class="prop-group">
      <div class="prop-label">نام گره</div>
      <input class="prop-input" value="${escHtml(node.name)}"
             onchange="updateNodeName('${node.id}', this.value)" />
    </div>
    <hr style="border:none;border-top:1px solid var(--border);margin:12px 0">
    <div class="prop-label" style="margin-bottom:10px">⚙ تنظیمات</div>
  `;

  switch (node.node_type) {
    case 'menu':
      html += propText('متن پرامپت', c.prompt_text, `updateConf('${node.id}','prompt_text',this.value)`);
      html += propText('فایل صوتی', c.audio_file || '', `updateConf('${node.id}','audio_file',this.value)`, 'مسیر فایل (اختیاری)');
      html += propNum('تایم‌اوت', c.timeout_secs, `updateConf('${node.id}','timeout_secs',+this.value)`, 'ثانیه');
      html += propNum('حداکثر تلاش', c.max_retries, `updateConf('${node.id}','max_retries',+this.value)`);
      break;
    case 'play_audio':
      html += propTextArea('متن TTS', c.text_to_speech || '', `updateConf('${node.id}','text_to_speech',this.value)`);
      html += propText('فایل صوتی', c.audio_file || '', `updateConf('${node.id}','audio_file',this.value)`, 'مسیر فایل (اختیاری)');
      break;
    case 'record_audio':
      html += propNum('حداکثر مدت', c.max_duration_secs, `updateConf('${node.id}','max_duration_secs',+this.value)`, 'ثانیه');
      html += propNum('تایم‌اوت سکوت', c.silence_timeout_secs, `updateConf('${node.id}','silence_timeout_secs',+this.value)`, 'ثانیه');
      html += propText('مسیر ذخیره', c.save_path || '', `updateConf('${node.id}','save_path',this.value)`, '/recordings/...');
      html += propCheck('بیپ قبل از ضبط', c.beep_before, `updateConf('${node.id}','beep_before',this.checked)`);
      break;
    case 'receive_digit':
      html += propText('متن پرامپت', c.prompt_text || '', `updateConf('${node.id}','prompt_text',this.value)`);
      html += propNum('حداکثر دیجیت', c.max_digits, `updateConf('${node.id}','max_digits',+this.value)`);
      html += propNum('تایم‌اوت', c.timeout_secs, `updateConf('${node.id}','timeout_secs',+this.value)`, 'ثانیه');
      html += propText('پایان‌دهنده', c.terminator || '', `updateConf('${node.id}','terminator',this.value)`, 'مثلاً #');
      break;
    case 'connect_call':
      html += `
        <div class="prop-group">
          <div class="prop-label">داخلی مقصد</div>
          <select class="prop-input" onchange="updateConf('${node.id}','extension',this.value)">
            ${extensions.map(e =>
              `<option value="${e.username}" ${e.username === c.extension ? 'selected' : ''}>
                ${e.username} ${e.registered ? '🟢' : '⚫'}
              </option>`
            ).join('')}
            <option value="${c.extension}" ${extensions.find(e=>e.username===c.extension)?'style="display:none"':''}>
              ${c.extension} (دستی)
            </option>
          </select>
          <input class="prop-input" style="margin-top:6px" value="${escHtml(c.extension)}"
                 placeholder="یا وارد کنید"
                 onchange="updateConf('${node.id}','extension',this.value)" />
        </div>
      `;
      html += propNum('تایم‌اوت', c.timeout_secs, `updateConf('${node.id}','timeout_secs',+this.value)`, 'ثانیه');
      break;
    case 'hangup':
      html += propTextArea('پیام خداحافظی', c.play_message || '', `updateConf('${node.id}','play_message',this.value)`);
      break;
  }

  // transitions
  html += `<hr style="border:none;border-top:1px solid var(--border);margin:16px 0">
    <div class="prop-label">🔗 اتصالات (${(node.transitions||[]).length})</div>
    <div class="transitions-list">`;

  (node.transitions || []).forEach(tr => {
    const toNode = flowData.nodes.find(n => n.id === tr.to_node_id);
    html += `
      <div class="transition-item">
        <span class="transition-trigger">${escHtml(tr.trigger || '→')}</span>
        <span class="transition-arrow">→</span>
        <span class="transition-target">${toNode ? escHtml(toNode.name) : '—'}</span>
        <span class="transition-del" onclick="deleteTrans('${tr.id}','${node.id}')">×</span>
      </div>`;
  });

  html += `</div>`;

  // delete button
  html += `
    <hr style="border:none;border-top:1px solid var(--border);margin:16px 0">
    <button class="btn danger" style="width:100%" onclick="deleteNode('${node.id}')">
      🗑 حذف این گره
    </button>`;

  return html;
}

function propText(label, val, onchange, placeholder='') {
  return `<div class="prop-group">
    <div class="prop-label">${label}</div>
    <input class="prop-input" value="${escHtml(val||'')}"
           placeholder="${placeholder}" onchange="${onchange}" />
  </div>`;
}
function propTextArea(label, val, onchange) {
  return `<div class="prop-group">
    <div class="prop-label">${label}</div>
    <textarea class="prop-input" onchange="${onchange}" rows="3">${escHtml(val||'')}</textarea>
  </div>`;
}
function propNum(label, val, onchange, unit='') {
  return `<div class="prop-group">
    <div class="prop-label">${label}</div>
    <div class="prop-number">
      <input type="number" class="prop-input" value="${val||0}" min="0" onchange="${onchange}" />
      ${unit ? `<span class="unit">${unit}</span>` : ''}
    </div>
  </div>`;
}
function propCheck(label, val, onchange) {
  return `<div class="prop-group">
    <label class="prop-checkbox">
      <input type="checkbox" ${val?'checked':''} onchange="${onchange}" />
      ${label}
    </label>
  </div>`;
}

async function updateNodeName(nodeId, name) {
  await api('PUT', `/api/nodes/${nodeId}`, { name });
  const node = flowData.nodes.find(n => n.id === nodeId);
  if (node) { node.name = name; renderCanvas(); }
}

async function updateConf(nodeId, field, value) {
  const node = flowData.nodes.find(n => n.id === nodeId);
  if (!node) return;
  node.config[field] = value;
  await api('PUT', `/api/nodes/${nodeId}`, { config: node.config });
  renderCanvas();
}

async function deleteTrans(trId, fromNodeId) {
  await api('DELETE', `/api/transitions/${trId}`);
  const fromNode = flowData.nodes.find(n => n.id === fromNodeId);
  if (fromNode) fromNode.transitions = fromNode.transitions.filter(t => t.id !== trId);
  renderCanvas();
  renderProps();
  toast('اتصال حذف شد', 'ok');
}

async function deleteNode(nodeId) {
  if (!confirm('حذف این گره؟')) return;
  await api('DELETE', `/api/nodes/${nodeId}`);
  flowData.nodes = flowData.nodes.filter(n => n.id !== nodeId);
  // remove transitions pointing to this node
  flowData.nodes.forEach(n => {
    if (n.transitions) n.transitions = n.transitions.filter(t => t.to_node_id !== nodeId);
  });
  selectedNodeId = null;
  renderCanvas();
  renderProps();
  toast('گره حذف شد', 'ok');
}

// ════════════════════════════════════════════
//   CONTEXT MENU
// ════════════════════════════════════════════
function showCtxMenu(e, nodeId) {
  e.preventDefault();
  e.stopPropagation();
  const menu = document.getElementById('ctx-menu');
  menu.style.left = e.clientX + 'px';
  menu.style.top  = e.clientY + 'px';
  menu.style.display = 'block';
  menu.innerHTML = `
    <div onclick="selectNode('${nodeId}');document.getElementById('ctx-menu').style.display='none'"
         style="padding:7px 12px;cursor:pointer;font-size:12px;border-radius:4px"
         onmouseover="this.style.background='var(--surface2)'" onmouseout="this.style.background=''">
      ✏️ ویرایش
    </div>
    <div onclick="setAsEntry('${nodeId}');document.getElementById('ctx-menu').style.display='none'"
         style="padding:7px 12px;cursor:pointer;font-size:12px;border-radius:4px"
         onmouseover="this.style.background='var(--surface2)'" onmouseout="this.style.background=''">
      ⚡ تنظیم به عنوان ورودی
    </div>
    <div style="height:1px;background:var(--border);margin:4px 0"></div>
    <div onclick="deleteNode('${nodeId}');document.getElementById('ctx-menu').style.display='none'"
         style="padding:7px 12px;cursor:pointer;font-size:12px;color:var(--red);border-radius:4px"
         onmouseover="this.style.background='var(--surface2)'" onmouseout="this.style.background=''">
      🗑 حذف
    </div>
  `;
}

async function setAsEntry(nodeId) {
  await api('PUT', `/api/flows/${currentFlowId}`, { entry_node_id: nodeId });
  flowData.flow.entry_node_id = nodeId;
  renderCanvas();
  toast('گره ورودی تنظیم شد ✓', 'ok');
}

// ════════════════════════════════════════════
//   ZOOM
// ════════════════════════════════════════════
function zoom(factor) {
  scale = Math.min(2, Math.max(0.3, scale * factor));
  renderCanvas();
}
function resetZoom() { scale = 1; pan = {x:0,y:0}; renderCanvas(); }

// ════════════════════════════════════════════
//   KEYBOARD
// ════════════════════════════════════════════
function onKeyDown(e) {
  if (e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA') return;
  if (e.key === 'Delete' || e.key === 'Backspace') {
    if (selectedNodeId) deleteNode(selectedNodeId);
  }
  if (e.key === 'Escape') {
    connectingFrom = null;
    renderEdges();
    selectNode(null);
  }
}

// ════════════════════════════════════════════
//   EXPORT
// ════════════════════════════════════════════
function exportFlow() {
  if (!flowData.flow.id) { toast('ابتدا یک flow انتخاب کنید', 'err'); return; }
  const json = JSON.stringify(flowData, null, 2);
  const blob = new Blob([json], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url; a.download = `${flowData.flow.name || 'flow'}.json`;
  a.click();
}

// ════════════════════════════════════════════
//   API Helper
// ════════════════════════════════════════════
async function api(method, path, body) {
  try {
    const opts = { method, headers: { 'Content-Type': 'application/json' } };
    if (body) opts.body = JSON.stringify(body);
    const r = await fetch(path, opts);
    return await r.json();
  } catch(e) {
    toast('خطا در ارتباط با سرور', 'err');
    return { success: false, data: null };
  }
}

// ════════════════════════════════════════════
//   UTILS
// ════════════════════════════════════════════
function escHtml(s) {
  return String(s||'').replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;').replace(/"/g,'&quot;');
}

function toast(msg, type='ok') {
  const el = document.getElementById('toast');
  el.textContent = msg;
  el.className = `show ${type}`;
  clearTimeout(el._t);
  el._t = setTimeout(() => el.className = '', 3000);
}

// ════════════════════════════════════════════
//   START
// ════════════════════════════════════════════
boot();
</script>
</body>
</html>
"###;
