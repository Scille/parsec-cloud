// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Editics debug panel — a DOM overlay that shows every event exchanged at both
// protocol layers (OnlyOffice ↔ editics client ↔ Parsec server). Extracted
// verbatim from the former `client/public/onlyoffice-editics-client.js` `Panel`
// singleton (todo step_2 §3.1). Not stripped for release yet (deferred, §8).
//
// This is browser-only (uses `document`); it is never loaded by tests.
//
// Flow titles (each with its own color) make the direction unambiguous without
// the "client"/"server" ambiguity (our editics client is the OnlyOffice
// "server" but the Parsec "client"):
//   - `OO -> editics`            editor sends to the editics client, local only.
//   - `OO <- editics`            editics client sends to the editor, local only.
//   - `OO -> editics -> server`  editor -> editics client, forwarded to Parsec.
//   - `OO <- editics <- server`  Parsec -> editics client, forwarded to editor.
//   - `editics -> server`        editics client -> Parsec, no OnlyOffice side.
//   - `editics <- server`        Parsec -> editics client, no OnlyOffice side.

export const Panel = (function () {
  let root, logEl, statusEl, filterSel;
  let entryCount = 0;
  const MAX_ENTRIES = 300;
  // `filter`: 'all' | 'oo' | 'net'. Hides entries that don't match the chosen
  // layer (paired entries always match).
  let filter = 'all';

  function ensure() {
    if (root) return;
    const style = document.createElement('style');
    style.textContent = `
        #ed-panel { position: fixed; right: 0; bottom: 0; width: 560px; max-height: 60vh;
          display: flex; flex-direction: column; font: 11px/1.4 monospace; background: #1e1e1eee;
          color: #ddd; border-top-left-radius: 6px; z-index: 999999; box-shadow: 0 0 12px #0008; }
        #ed-panel.collapsed { max-height: unset; }
        #ed-panel.collapsed .ed-body { display: none; }
        #ed-panel .ed-header { display: flex; justify-content: space-between; align-items: center;
          padding: 4px 8px; background: #333; cursor: pointer; user-select: none;
          border-top-left-radius: 6px; }
        #ed-panel .ed-body { display: flex; flex-direction: column; min-height: 0; }
        #ed-panel .ed-toolbar { display: flex; align-items: center; gap: 6px; padding: 4px 8px;
          border-bottom: 1px solid #444; }
        #ed-panel .ed-toolbar select { font: 10px/1 monospace; background: #222; color: #ddd;
          border: 1px solid #555; border-radius: 3px; padding: 2px 4px; }
        #ed-panel .ed-toolbar button { font: 10px/1 monospace; background: #444; color: #eee;
          border: 1px solid #666; border-radius: 3px; padding: 3px 6px; cursor: pointer; }
        #ed-panel .ed-toolbar button:hover { background: #555; }
        #ed-panel .ed-log { overflow-y: auto; padding: 4px 8px; flex: 1; }
        #ed-panel .ed-entry { padding: 2px 0; border-bottom: 1px solid #2a2a2a; }
        #ed-panel .ed-entry[hidden] { display: none; }
        #ed-panel .ed-entry summary { cursor: pointer; white-space: pre-wrap; word-break: break-word; }
        #ed-panel .ed-section { margin: 2px 0 4px 12px; }
        #ed-panel .ed-section-label { color: #888; font-size: 10px; margin-bottom: 1px; }
        #ed-panel .ed-entry pre { margin: 0; white-space: pre-wrap; word-break: break-word; }
        #ed-panel .ed-oo pre { color: #f2c94c; }
        #ed-panel .ed-net pre { color: #9cdcfe; }
        #ed-panel .ed-flow { font-size: 10px; font-weight: bold; padding: 1px 6px; border-radius: 3px;
          margin-right: 4px; white-space: nowrap; }
        #ed-panel .ed-flow-oo-in   { background: #6b5a1a; color: #ffe08a; } /* OO -> editics            (gold)     */
        #ed-panel .ed-flow-oo-out  { background: #3d5a1a; color: #c8e08a; } /* OO <- editics            (olive)    */
        #ed-panel .ed-flow-oo2srv  { background: #5a1a6b; color: #e08aff; } /* OO -> editics -> server  (purple)   */
        #ed-panel .ed-flow-srv2oo  { background: #1a5a6b; color: #8ae0ff; } /* OO <- editics <- server  (teal)     */
        #ed-panel .ed-flow-net-out { background: #1a3d6b; color: #8ab8ff; } /* editics -> server        (blue)     */
        #ed-panel .ed-flow-net-in  { background: #6b1a3d; color: #ff8ac8; } /* editics <- server        (magenta)  */
        #ed-panel .ed-dir { color: #aaa; }
        #ed-panel .ed-type { color: #fff; font-weight: bold; }
        #ed-panel .ed-time { color: #666; }
        #ed-panel .ed-status { padding: 4px 8px; border-top: 1px solid #444; color: #9cdcfe; }
      `;
    document.head.appendChild(style);

    root = document.createElement('div');
    root.id = 'ed-panel';
    root.innerHTML = `
        <div class="ed-header">
          <span>Editics protocol (OnlyOffice ↔ editics client ↔ Parsec server)</span>
          <span class="ed-toggle">–</span>
        </div>
        <div class="ed-body">
          <div class="ed-toolbar">
            <select class="ed-filter" title="Filter by protocol layer">
              <option value="all">all layers</option>
              <option value="oo">OnlyOffice only</option>
              <option value="net">Editics (network) only</option>
            </select>
            <button class="ed-clear" title="Clear the log">clear</button>
          </div>
          <div class="ed-log"></div>
          <div class="ed-status"></div>
        </div>
      `;
    document.body.appendChild(root);
    logEl = root.querySelector('.ed-log');
    statusEl = root.querySelector('.ed-status');
    filterSel = root.querySelector('.ed-filter');
    const header = root.querySelector('.ed-header');
    const toggle = root.querySelector('.ed-toggle');
    header.addEventListener('click', () => {
      root.classList.toggle('collapsed');
      toggle.textContent = root.classList.contains('collapsed') ? '+' : '–';
    });
    filterSel.addEventListener('change', () => {
      filter = filterSel.value;
      applyFilter();
    });
    root.querySelector('.ed-clear').addEventListener('click', (ev) => {
      ev.stopPropagation();
      logEl.innerHTML = '';
      entryCount = 0;
    });
  }

  function applyFilter() {
    const entries = logEl.querySelectorAll('.ed-entry');
    entries.forEach((el) => {
      const layers = el.getAttribute('data-layers') || '';
      const show = filter === 'all' || layers === 'paired' || layers.indexOf(filter) !== -1;
      el.hidden = !show;
    });
  }

  function setStatus(text) {
    ensure();
    statusEl.textContent = text;
  }

  // Derive the flow title (`ed-flow` CSS class + label) from an entry's
  // `oo`/`net` directions. The title describes the whole path the event
  // travels, so paired entries get a single combined title:
  //   - OO in  + NET out  -> `OO -> editics -> server` (editor -> us -> Parsec)
  //   - NET in  + OO out  -> `OO <- editics <- server` (Parsec -> us -> editor)
  //   - OO in  only        -> `OO -> editics`
  //   - OO out only        -> `OO <- editics`
  //   - NET out only       -> `editics -> server`
  //   - NET in  only       -> `editics <- server`
  function flowTitle(hasOo, hasNet, ooDir, netDir) {
    if (hasOo && hasNet) {
      // Paired: the OO and NET directions are consistent (in/out), so the
      // combined title reads end-to-end. The editor side drives the title.
      if (ooDir === 'in') return { cls: 'ed-flow-oo2srv', label: 'OO -> editics -> server' };
      return { cls: 'ed-flow-srv2oo', label: 'OO <- editics <- server' };
    }
    if (hasOo) return ooDir === 'in' ? { cls: 'ed-flow-oo-in', label: 'OO -> editics' } : { cls: 'ed-flow-oo-out', label: 'OO <- editics' };
    return netDir === 'out'
      ? { cls: 'ed-flow-net-out', label: 'editics -> server' }
      : { cls: 'ed-flow-net-in', label: 'editics <- server' };
  }

  function safeStringify(obj) {
    try {
      return typeof obj === 'string' ? obj : JSON.stringify(obj, null, 1);
    } catch (_e) {
      return String(obj);
    }
  }

  // `entry` shape:
  //   { type, note, oo: {dir, payload}|null, net: {dir, payload}|null }
  // `oo.dir`: 'in' = editor -> editics, 'out' = editics -> editor.
  // `net.dir`: 'out' = editics -> server, 'in' = server -> editics.
  function log(entry) {
    ensure();
    const hasOo = !!entry.oo;
    const hasNet = !!entry.net;
    const layers = hasOo && hasNet ? 'paired' : hasOo ? 'oo' : 'net';
    const details = document.createElement('details');
    details.className = 'ed-entry';
    details.setAttribute('data-layers', layers);
    const time = new Date().toLocaleTimeString();
    const summary = document.createElement('summary');
    const flow = flowTitle(hasOo, hasNet, hasOo && entry.oo.dir, hasNet && entry.net.dir);
    const noteSuffix = entry.note ? ` <span class="ed-dir">\u00b7 ${entry.note}</span>` : '';
    summary.innerHTML =
      `<span class="ed-flow ${flow.cls}">${flow.label}</span>` +
      `<span class="ed-type">${entry.type}</span> ` +
      `<span class="ed-time">${time}</span>${noteSuffix}`;
    details.appendChild(summary);

    if (hasOo) {
      const sec = document.createElement('div');
      sec.className = 'ed-section ed-oo';
      sec.innerHTML = '<div class="ed-section-label">OnlyOffice protocol</div>';
      const pre = document.createElement('pre');
      pre.textContent = safeStringify(entry.oo.payload);
      sec.appendChild(pre);
      details.appendChild(sec);
    }
    if (hasNet) {
      const sec = document.createElement('div');
      sec.className = 'ed-section ed-net';
      sec.innerHTML = '<div class="ed-section-label">Editics protocol / network</div>';
      const pre = document.createElement('pre');
      pre.textContent = safeStringify(entry.net.payload);
      sec.appendChild(pre);
      details.appendChild(sec);
    }

    logEl.appendChild(details);
    entryCount++;
    while (entryCount > MAX_ENTRIES && logEl.firstChild) {
      logEl.removeChild(logEl.firstChild);
      entryCount--;
    }
    // Apply current filter to the new entry.
    const show = filter === 'all' || layers === 'paired' || layers.indexOf(filter) !== -1;
    details.hidden = !show;
    // Only auto-scroll to the bottom when the user is already (roughly) at the
    // bottom; if they scrolled up to inspect history, don't yank the view back
    // down on every new event.
    const nearBottom = logEl.scrollHeight - logEl.scrollTop - logEl.clientHeight < details.offsetHeight + 24;
    if (nearBottom) logEl.scrollTop = logEl.scrollHeight;
  }

  return { log, setStatus };
})();
