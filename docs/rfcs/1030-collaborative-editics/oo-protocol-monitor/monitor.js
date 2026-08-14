/*
 * OnlyOffice co-editing protocol monitor — injected in-page script.
 *
 * Patches window.WebSocket (and XHR polling fallback) BEFORE OnlyOffice's
 * socket.io loads, decodes the Engine.IO v4 + Socket.IO v4 framing, and
 * surfaces OnlyOffice's own "message" protocol: {type:"auth"|"cursor"|...}.
 *
 * Architecture: capture happens inside the editor iframes (where the OO
 * WebSockets live). Each decoded event is forwarded to the top frame via
 * postMessage; the top frame renders ONE unified, draggable panel.
 *
 * Editor identity is resolved from the `auth` message (payload.user.username),
 * so the two Collaborate editors show as "John Smith" / "Kate Cage" while the
 * solo scenarios show as "Anonymous". The panel auto-filters by the active
 * scenario tab: Collaborate → John & Kate; other scenarios → Anonymous.
 *
 * Use standalone:  paste into DevTools Console / a Tampermonkey userscript
 *   matching both  https://www.onlyoffice.com/see-it-in-action*
 *   and            https://site.docs.onlyoffice.com/*
 * Use with Playwright: injected via ctx.addInitScript() (see run-demo.js).
 */
(function () {
  'use strict';
  if (window.__OO_MON) return; // guard against double injection
  var IS_TOP = (window === window.top);
  var MON = window.__OO_MON = {
    events: [], paused: false, sockets: 0,
    editorUser: {},            // editorId -> username (from auth)
    users: {},                 // username -> {checked:bool}  (filter chips)
    mode: null                 // 'collab' | 'solo' | null (set by active tab)
  };
  var EDITOR_ID =
    (/[?&]frameEditorId=([^&]*)/.exec(location.search || '') || [, null])[1];

  // ---- Engine.IO v4 + Socket.IO v4 frame decoder ---------------------------
  function decodeEIO(data) {
    if (typeof data !== 'string' || !data.length) return null;
    switch (data[0]) {
      case '0': return { eio: 'open',  payload: tryJson(data.slice(1)) };
      case '1': return { eio: 'close' };
      case '2': return { eio: 'ping' };
      case '3': return { eio: 'pong' };
      case '4': return decodeSIO(data.slice(1));
      case '5': return { eio: 'upgrade' };
      case '6': return { eio: 'noop' };
      default:  return { eio: 'unknown', raw: data.slice(0, 40) };
    }
  }
  function decodeSIO(s) {
    var st = s[0], rest = s.slice(1), ns = '/';
    if (rest[0] === '/') {
      var c = rest.indexOf(',');
      ns = c >= 0 ? rest.slice(0, c) : rest;
      rest = c >= 0 ? rest.slice(c + 1) : '';
    }
    switch (st) {
      case '0': return { sio: 'connect', ns: ns, payload: tryJson(rest) };
      case '1': return { sio: 'disconnect', ns: ns };
      case '2': { var a = tryJson(rest);
        return Array.isArray(a) ? { sio: 'event', ns: ns, name: a[0], args: a.slice(1) }
                                : { sio: 'event', ns: ns, raw: rest.slice(0, 80) }; }
      case '3': return { sio: 'ack', ns: ns, payload: tryJson(rest) };
      case '4': return { sio: 'error', ns: ns, payload: tryJson(rest) };
      case '5': return { sio: 'binary-event', ns: ns };
      default:  return { sio: 'unknown', ns: ns, raw: s.slice(0, 40) };
    }
  }
  function tryJson(s) { if (!s) return null; try { return JSON.parse(s); } catch (e) { return s; } }
  function classify(d) {
    if (!d || d.sio !== 'event' || d.name !== 'message') return null;
    var p = d.args && d.args[0];
    if (!p || typeof p !== 'object') return null;
    return { type: p.type || '?', payload: p };
  }
  // OnlyOffice also sends its `auth` inside the Socket.IO CONNECT packet
  // (SIO type 0), as `40{"data":{"type":"auth","user":{...}}}`. Extract the
  // username from there so we learn the editor's identity one frame earlier
  // (before the first `42` auth event), which lets us label the opening
  // handshake/auth handshake correctly.
  function authFromConnect(d) {
    if (!d || d.sio !== 'connect' || !d.payload) return null;
    var data = d.payload.data || d.payload;
    if (data && data.type === 'auth' && data.user && data.user.username) return data.user.username;
    return null;
  }

  // ---- EIO noise filter ---------------------------------------------------
  // ping/pong/noop are keepalive frames; drop them to keep the log readable.
  function isNoise(d) {
    return d && d.eio && (d.eio === 'ping' || d.eio === 'pong' || d.eio === 'noop');
  }

  // ---- editor identity ----------------------------------------------------
  // A `send auth` message carries payload.user.username for its editor frame.
  // We snapshot the resolved username onto each event (ev.user) at capture time
  // so an event keeps the identity it had when it occurred, even if the same
  // editor frame is later reused by another user (e.g. switching scenarios
  // reuses the `document-editor` frame id).
  function noteAuth(ev) {
    if (ev.kind !== 'msg' || ev.dir !== 'send') return;
    var p = ev.meta && ev.meta.payload;
    if (p && p.type === 'auth' && p.user && p.user.username && ev.editor) {
      var name = p.user.username;
      if (MON.editorUser[ev.editor] !== name) {
        MON.editorUser[ev.editor] = name;
        var g = (ev.gen != null) ? ev.gen : GEN[ev.editor];
        if (g != null) GEN_USER[ev.editor + '#' + g] = name;
        ensureUser(name);
      }
    }
  }
  function ensureUser(name) {
    if (!MON.users[name]) {
      // default: checked (visible). Tab-driven auto-filter overrides later.
      MON.users[name] = { checked: true };
      if (IS_TOP) addUserChip(name);
    }
  }
  var GEN = {}, GEN_USER = {}; // editor -> generation count ; (editor+gen) -> user
  function bumpGen(editor) { GEN[editor] = (GEN[editor] || 0) + 1; return GEN[editor]; }
  function genOf(ev) { return ev.editor && ev.gen != null ? ev.gen : null; }
  function resolveUser(ev) {
    // prefer a user snapshotted on the event; else the per-generation map;
    // else the current editor->user map.
    if (ev.user) return ev.user;
    if (ev.editor && ev.gen != null && GEN_USER[ev.editor + '#' + ev.gen]) return GEN_USER[ev.editor + '#' + ev.gen];
    return (ev.editor && MON.editorUser[ev.editor]) || ev.editor || '?';
  }
  function userOf(ev) { return resolveUser(ev); }

  // ---- record + transport -------------------------------------------------
  function makeEv(dir, kind, meta) {
    var gen = EDITOR_ID ? (GEN[EDITOR_ID] || 0) : null;
    return { t: Date.now(), dir: dir, kind: kind, meta: meta, editor: EDITOR_ID || '?', gen: gen };
  }
  function record(ev) {
    noteAuth(ev);
    // Only snapshot a *resolved* username onto the event. Leave unresolved
    // events (before their auth arrived) un-snapshotted so they pick up their
    // generation's user later via GEN_USER (see resolveUser).
    var u = resolveUser(ev);
    if (u && u !== ev.editor) ev.user = u;
    if (IS_TOP) store(ev); else forward(ev);
  }
  function store(ev) {
    MON.events.push(ev);
    if (MON.events.length > 5000) MON.events.shift();
    if (!MON.paused && visible(ev)) render(ev);
  }
  function forward(ev) {
    try { window.top.postMessage({ __oo_mon: 1, ev: ev }, '*'); } catch (e) {}
  }

  // Patch WebSocket (capture only; OO sockets live in editor frames)
  var OrigWS = window.WebSocket;
  function PatchedWS(url, protocols) {
    var ws = protocols ? new OrigWS(url, protocols) : new OrigWS(url);
    if (/\/doc\/[^/]+\/c\/.*EIO=/.test(url)) {
      MON.sockets++;
      var gen = bumpGen(EDITOR_ID || '?');
      record(makeEv('open', 'ws', url));
      ws.addEventListener('message', function (e) {
        var d = decodeEIO(e.data); if (isNoise(d)) return;
        var c = classify(d);
        if (c) record(makeEv('recv', 'msg', c));
        else if (d && d.eio) record(makeEv('recv', 'eio', d));
      });
      var origSend = ws.send.bind(ws);
      ws.send = function (d) {
        var dec = decodeEIO(d); if (isNoise(dec)) return origSend(d);
        // Learn the editor's username from the SIO CONNECT packet's auth payload
        // (arrives before the first `42` auth event), so the opening handshake
        // is labelled with the right user from the start.
        var name = authFromConnect(dec);
        if (name && EDITOR_ID && MON.editorUser[EDITOR_ID] !== name) {
          MON.editorUser[EDITOR_ID] = name;
          var g = GEN[EDITOR_ID]; if (g != null) GEN_USER[EDITOR_ID + '#' + g] = name;
          ensureUser(name);
        }
        var c = classify(dec);
        if (c) record(makeEv('send', 'msg', c));
        else if (dec && dec.eio) record(makeEv('send', 'eio', dec));
        return origSend(d);
      };
      ws.addEventListener('close', function () {
        record(makeEv('engine', 'close', url));
      });
    }
    return ws;
  }
  PatchedWS.prototype = OrigWS.prototype;
  PatchedWS.CONNECTING = OrigWS.CONNECTING; PatchedWS.OPEN = OrigWS.OPEN;
  PatchedWS.CLOSING = OrigWS.CLOSING; PatchedWS.CLOSED = OrigWS.CLOSED;
  window.WebSocket = PatchedWS;

  // Patch XHR (polling fallback)
  var Oo = XMLHttpRequest.prototype.open, Os = XMLHttpRequest.prototype.send;
  XMLHttpRequest.prototype.open = function (m, u) { this.__u = u; this.__m = m; return Oo.apply(this, arguments); };
  XMLHttpRequest.prototype.send = function (body) {
    var self = this, poll = /[?&]EIO=4&transport=polling/.test(this.__u || '');
    if (poll && body) { var d = decodeEIO(body); if (!isNoise(d)) record(makeEv('send', 'poll', String(body).slice(0, 200))); }
    this.addEventListener('load', function () {
      if (!poll) return;
      (self.responseText || '').split('\n').forEach(function (pkt) {
        if (!pkt) return;
        var d = decodeEIO(pkt); if (isNoise(d)) return;
        var c = classify(d);
        if (c) record(makeEv('recv', 'msg', c));
        else if (d && d.eio) record(makeEv('recv', 'eio', d));
      });
    });
    return Os.apply(this, arguments);
  };

  // ---- top frame: receive forwarded events + observe scenario tabs ---------
  if (IS_TOP) {
    window.addEventListener('message', function (e) {
      var d = e.data;
      if (!d || d.__oo_mon !== 1 || !d.ev) return;
      noteAuth(d.ev);
      // Resolve the user live (deferred resolution): events that arrived
      // before their generation's auth get the right user once that auth lands.
      var u = resolveUser(d.ev);
      if (u && u !== d.ev.editor) d.ev.user = u;
      store(d.ev);
    });

    // Initialize Kate-block state early (referenced by applyScenario below).
    MON.kateBlocked = true; MON.kateConfig = null; MON.kateOrig = null;

    // Observe the scenario tab buttons. "Collaborate" → John & Kate;
    // every other scenario → "Anonymous".
    function activeScenario() {
      var btn = document.querySelector('[class*="actions-tab-button"][class*="active"], [class*="actions-tab-button"].active');
      // fall back: scan by text
      if (!btn) {
        var btns = document.querySelectorAll('button[class*="actions-tab-button"]');
        for (var i = 0; i < btns.length; i++) if (/active/i.test(btns[i].className)) { btn = btns[i]; break; }
      }
      return btn ? (btn.textContent || '').trim() : null;
    }
    function applyScenario(tabText) {
      if (!tabText) return;
      var collab = /collaborate/i.test(tabText);
      MON.mode = collab ? 'collab' : 'solo';
      // Re-arm the Kate block each time we (re)enter Collaborate, so Kate's
      // editor is held back until manually started.
      if (collab) MON.kateBlocked = true;
      for (var name in MON.users) {
        var isAnon = /anonymous/i.test(name);
        MON.users[name].checked = collab ? !isAnon : isAnon;
      }
      syncChips();
      rebuild();
    }
    // The active scenario tab is polled below (interval) which is robust to
    // React updates and any tab-switch trigger (click, keyboard, etc.).
    // Re-apply when the active tab changes for any reason (e.g. launch click).
    var lastTab = null;
    setInterval(function () {
      var t = activeScenario();
      if (t && t !== lastTab) { lastTab = t; applyScenario(t); }
    }, 500);

    // ---- Prevent Kate's editor (document-editor-2) from auto-starting -------
    // The demo constructs both editors via `new DocsAPI.DocEditor(id, config)`.
    // We intercept that constructor: let `document-editor` (John) build normally,
    // but defer `document-editor-2` (Kate) — we keep its config and create the
    // editor only when the user clicks "Start Kate's editor". This lets you
    // edit solo in John first and then observe Kate's late-join behaviour.
    MON.kateBlocked = true;       // Kate hasn't started yet
    MON.kateConfig = null;        // {id, cfg} captured from the deferred constructor
    MON.kateOrig = null;         // the real DocEditor constructor
    (function installDocEditorTrap() {
      if (window.__oo_doceditor_trap) return;
      window.__oo_doceditor_trap = true;
      function wrap() {
        var D = window.DocsAPI;
        if (D && D.DocEditor && !D.DocEditor.__oo_wrapped) {
          var orig = D.DocEditor;
          MON.kateOrig = orig;
          function W(id, cfg) {
            if (id === 'document-editor-2' && MON.kateBlocked) {
              MON.kateConfig = { id: id, cfg: cfg };
              return { __ooDeferred: true };
            }
            return new orig(id, cfg);
          }
          W.__oo_wrapped = true; W.prototype = orig.prototype; W.__oo_orig = orig;
          D.DocEditor = W;
        }
      }
      // DocsAPI may be defined after this script runs; poll until wrapped.
      setInterval(wrap, 50);
    })();
    MON.startKate = function () {
      MON.kateBlocked = false;
      removeKateOverlay();
      if (MON.kateConfig && MON.kateOrig) {
        var k = MON.kateConfig;
        MON.kateConfig = null;
        new MON.kateOrig(k.id, k.cfg); // create Kate's editor for real
      }
    };
    function removeKateOverlay() {
      var o = document.getElementById('oo-kate-overlay'); if (o) o.remove();
    }
    // Periodically ensure the placeholder for Kate's editor shows a
    // "Start Kate's editor" button overlay while she is deferred.
    setInterval(function () {
      if (!MON.kateBlocked) { removeKateOverlay(); return; }
      if (MON.mode !== 'collab') { removeKateOverlay(); return; }
      // Kate's slot is the 2nd actions-content div (grid column 2).
      var contents = Array.from(document.querySelectorAll('div[class*="actions-container"] > div[class*="actions-content"]'));
      var target = contents.length >= 2 ? contents[1] : null;
      if (!target) return;
      var overlay = document.getElementById('oo-kate-overlay');
      if (!overlay) {
        overlay = document.createElement('div');
        overlay.id = 'oo-kate-overlay';
        overlay.style.cssText = 'position:absolute;inset:0;display:flex;align-items:center;justify-content:center;flex-direction:column;gap:10px;background:rgba(250,250,252,.96);border:1px dashed #c8ccd4;border-radius:8px;z-index:5;font:14px/1.4 ui-monospace,monospace;color:#444;text-align:center;padding:16px';
        var label = document.createElement('div'); label.textContent = "Kate's editor is on hold";
        overlay.appendChild(label);
        var b = document.createElement('button');
        b.textContent = "Start Kate's editor";
        b.style.cssText = 'background:#3b5b8c;color:#fff;border:0;border-radius:6px;padding:8px 14px;cursor:pointer;font:inherit';
        b.addEventListener('click', function () { MON.startKate(); });
        overlay.appendChild(b);
        var hint = document.createElement('div');
        hint.style.cssText = 'font-size:11px;color:#888';
        hint.textContent = "edit in John's editor first, then start Kate to observe the late-join";
        overlay.appendChild(hint);
      }
      if (target.style.position !== 'relative') target.style.position = 'relative';
      if (overlay.parentElement !== target) target.appendChild(overlay);
    }, 500);
  }

  // ---- UI (top frame only) ------------------------------------------------
  if (!IS_TOP) return;

  function buildUI() {
    var css = [
      '#oo-mon{position:fixed;bottom:16px;right:16px;width:540px;height:60vh;min-width:320px;min-height:200px;',
      'display:flex;flex-direction:column;z-index:2147483647;font:12px/1.4 ui-monospace,monospace;',
      'color:#1a1a1a;background:rgba(250,250,252,.98);border:1px solid #c8ccd4;',
      'border-radius:8px;box-shadow:0 6px 24px rgba(0,0,0,.25);overflow:hidden}',
      '#oo-mon .hdr{user-select:none;cursor:move;padding:6px 8px;background:#3b5b8c;color:#fff;',
      'font-weight:600;display:flex;align-items:center;gap:6px}',
      '#oo-mon .hdr .lbl{flex:1}',
      '#oo-mon .hdr button{background:rgba(255,255,255,.18);border:0;color:#fff;border-radius:4px;padding:1px 6px;cursor:pointer;font:inherit}',
      '#oo-mon .hdr button:hover{background:rgba(255,255,255,.32)}',
      '#oo-mon .bar{display:flex;gap:4px;padding:4px 6px;background:#eef1f5;border-bottom:1px solid #d8dce3;font-size:11px;align-items:center;flex-wrap:wrap}',
      '#oo-mon .bar .stat{flex:1;color:#555;min-width:60px}',
      '#oo-mon .chips{display:flex;gap:4px;flex-wrap:wrap}',
      '#oo-mon .chip{display:inline-flex;align-items:center;gap:3px;padding:1px 6px;border:1px solid #c8ccd4;border-radius:10px;cursor:pointer;background:#fff;color:#444}',
      '#oo-mon .chip input{margin:0}',
      '#oo-mon .chip.off{opacity:.45}',
      '#oo-mon .list{flex:1;min-height:0;overflow-y:auto;overscroll-behavior:contain}',
      '#oo-mon .row{border-bottom:1px solid #eee}',
      '#oo-mon .row.recv .head{background:#eef7ee} #oo-mon .row.send .head{background:#eef3fb}',
      '#oo-mon .row.engine .head{background:#f3eefb;color:#666}',
      '#oo-mon .head{display:flex;gap:6px;padding:3px 6px;cursor:pointer;align-items:baseline}',
      '#oo-mon .head:hover{filter:brightness(.97)}',
      '#oo-mon .head .d{flex:0 0 16px} #oo-mon .head .ed{flex:0 0 80px;color:#7a5b00;font-size:10px;overflow:hidden;text-overflow:ellipsis}',
      '#oo-mon .head .ts{flex:0 0 66px;color:#888} #oo-mon .head .ty{flex:0 0 96px;font-weight:600}',
      '#oo-mon .head .sum{flex:1;color:#333;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}',
      '#oo-mon .det{user-select:text;-webkit-user-select:text;white-space:pre-wrap;word-break:break-all;',
      'background:#fff;border-top:1px solid #e3e3e3;padding:6px 8px;margin:0;font-size:11px;max-height:40vh;overflow:auto;cursor:text}',
      '#oo-mon .row:not(.open) .det{display:none}',
      '#oo-mon .resize{position:absolute;right:0;bottom:0;width:14px;height:14px;cursor:nwse-resize;',
      'background:linear-gradient(135deg,transparent 50%,#3b5b8c 50%);border-top-left-radius:4px;z-index:2}',
      '/* Lay the two Collaborate editors out side by side instead of stacked, */',
      '/* and widen the container so the editors take more horizontal space. */',
      'div[class*="actions-container"]{display:grid !important;grid-template-columns:1fr 1fr !important;gap:12px !important;',
      'width:auto !important;max-width:none !important;margin:0 16px !important}',
      'div[class*="actions-container"] > div[class*="actions-button-wrapper"]{grid-column:1 / -1}',
      'div[class*="actions-container"] > div[class*="actions-content"]{width:100% !important;max-width:none !important}',
      'div[class*="actions-container"] > div[class*="actions-content"] > iframe{width:100% !important;max-width:none !important}'
    ].join('');
    var st = document.createElement('style'); st.textContent = css;
    (document.head || document.documentElement).appendChild(st);

    var box = document.createElement('div'); box.id = 'oo-mon';
    box.innerHTML =
      '<div class="hdr" id="oo-mon-hdr"><span class="lbl">OO Protocol Monitor</span>' +
      '<button id="oo-mon-copy">copy</button><button id="oo-mon-clear">clear</button><button id="oo-mon-pause">pause</button></div>' +
      '<div class="bar"><div class="chips" id="oo-mon-chips"></div>' +
      '<span class="stat" id="oo-mon-stat">0 events</span></div>' +
      '<div class="list" id="oo-mon-list"></div>' +
      '<div class="resize" id="oo-mon-resize" title="drag to resize"></div>';
    (document.body || document.documentElement).appendChild(box);

    // drag
    var hdr = document.getElementById('oo-mon-hdr');
    var dragging = false, ox = 0, oy = 0;
    hdr.addEventListener('mousedown', function (e) {
      dragging = true; ox = e.clientX - box.offsetLeft; oy = e.clientY - box.offsetTop;
    });
    document.addEventListener('mousemove', function (e) {
      if (!dragging) return;
      box.style.left = (e.clientX - ox) + 'px'; box.style.top = (e.clientY - oy) + 'px';
      box.style.right = 'auto';
    });
    document.addEventListener('mouseup', function () { dragging = false; });

    // resize (drag the bottom-right handle; the top-left corner stays fixed).
    var resizer = document.getElementById('oo-mon-resize');
    var resizing = false, rsx = 0, rsy = 0, rsw = 0, rsh = 0;
    resizer.addEventListener('mousedown', function (e) {
      e.preventDefault(); e.stopPropagation();
      resizing = true;
      // pin the top-left corner so resizing keeps it fixed regardless of the
      // panel's current anchor (bottom/right or left/top after dragging).
      box.style.left = box.offsetLeft + 'px';
      box.style.top = box.offsetTop + 'px';
      box.style.right = 'auto';
      box.style.bottom = 'auto';
      rsx = e.clientX; rsy = e.clientY;
      rsw = box.offsetWidth; rsh = box.offsetHeight;
    });
    document.addEventListener('mousemove', function (e) {
      if (!resizing) return;
      var w = Math.max(320, rsw + (e.clientX - rsx));
      var h = Math.max(200, rsh + (e.clientY - rsy));
      box.style.width = w + 'px';
      box.style.height = h + 'px';
    });
    document.addEventListener('mouseup', function () { resizing = false; });

    document.getElementById('oo-mon-clear').addEventListener('click', function () {
      MON.events.length = 0; rebuild(); updateStat();
    });
    var pb = document.getElementById('oo-mon-pause');
    pb.addEventListener('click', function () {
      MON.paused = !MON.paused; pb.textContent = MON.paused ? 'resume' : 'pause';
    });
    document.getElementById('oo-mon-copy').addEventListener('click', function () {
      var text = MON.events.filter(visible).map(formatBlock).join('\n\n');
      copyText(text, document.getElementById('oo-mon-copy'));
    });
    listEl = document.getElementById('oo-mon-list');
    statEl = document.getElementById('oo-mon-stat');
    chipsEl = document.getElementById('oo-mon-chips');
    // re-sync chips for users already known (e.g. events arrived before UI built)
    Object.keys(MON.users).forEach(addUserChip);
    syncChips();
  }

  var listEl, statEl, chipsEl;

  function ensureUI() {
    if (listEl || !document.body) return;
    if (!document.getElementById('oo-mon')) { try { buildUI(); } catch (e) { return; } }
    listEl = document.getElementById('oo-mon-list');
    statEl = document.getElementById('oo-mon-stat');
    chipsEl = document.getElementById('oo-mon-chips');
  }

  function summarize(p) {
    try {
      switch (p.type) {
        case 'auth': return 'user=' + (p.user && p.user.username);
        case 'cursor': return 'cursor=' + String(p.cursor || '').slice(0, 16);
        case 'saveChanges': return 'changes=' + (p.changes ? p.changes.length : 0) + ' start=' + p.startSaveChanges;
        case 'documentOpen': return 'status=' + (p.data && p.data.status);
        case 'connectState': return 'participants=' + (p.participants ? p.participants.length : 0);
        case 'clientLog': return (p.level || '') + ': ' + String(p.msg || '').slice(0, 50);
        case 'message': return 'chat msg';
        default: return '';
      }
    } catch (e) { return ''; }
  }

  // Markdown block for one event: a heading line + the full JSON payload in a
  // fenced code block.
  //   ### 08:27:18.020  ->  John Smith    auth
  //   ```json
  //   { ...full payload... }
  //   ```
  function typeOf(ev) {
    if (ev.dir === 'open') return 'ws-open';
    if (ev.dir === 'engine') return ev.kind;
    if (ev.kind === 'msg') return ev.meta.type;
    if (ev.kind === 'eio') return ev.meta.eio + (ev.meta.sio ? '/' + ev.meta.sio : '');
    return ev.kind;
  }
  function formatLine(ev) {
    var ts = new Date(ev.t).toISOString().slice(11, 23);
    var dir = ev.dir === 'send' ? '->' : ev.dir === 'recv' ? '<-' : '  ';
    return pad(ts, 13) + '  ' + dir + '  ' + pad(userOf(ev), 14) + '  ' + typeOf(ev);
  }
  function formatBlock(ev) {
    var body = (ev.meta === undefined || ev.meta === null) ? '' : JSON.stringify(ev.meta, null, 2);
    return '### ' + formatLine(ev) + '\n\n```json\n' + body + '\n```';
  }
  function pad(s, n) { s = String(s); return s.length >= n ? s : s + new Array(n - s.length + 1).join(' '); }

  function copyText(text, btn) {
    var done = function () { if (btn) { var o = btn.textContent; btn.textContent = 'copied!'; setTimeout(function () { btn.textContent = o; }, 1200); } };
    try {
      if (navigator.clipboard && navigator.clipboard.writeText) {
        navigator.clipboard.writeText(text).then(done, function () { fallbackCopy(text); done(); });
        return;
      }
    } catch (e) {}
    fallbackCopy(text); done();
  }
  function fallbackCopy(text) {
    var ta = document.createElement('textarea');
    ta.value = text; ta.style.position = 'fixed'; ta.style.top = '-9999px';
    document.body.appendChild(ta); ta.select();
    try { document.execCommand('copy'); } catch (e) {}
    document.body.removeChild(ta);
  }

  function makeRow(ev) {
    var row = document.createElement('div');
    row.className = 'row ' + (ev.dir === 'open' ? 'engine' : ev.dir);
    var d, ty = ev.kind, sum = '';
    if (ev.dir === 'open') { d = '🔌'; ty = 'ws-open'; sum = String(ev.meta).slice(-50); }
    else if (ev.dir === 'engine') { d = '⚙'; ty = ev.kind; sum = String(ev.meta).slice(-50); }
    else if (ev.kind === 'msg') { d = ev.dir === 'send' ? '⬆' : '⬇'; ty = ev.meta.type; sum = summarize(ev.meta.payload); }
    else if (ev.kind === 'eio') { d = '⚙'; ty = ev.meta.eio + (ev.meta.sio ? '/' + ev.meta.sio : ''); }
    else { d = ev.dir === 'send' ? '⬆' : '⬇'; ty = ev.kind; }
    var ts = new Date(ev.t);
    var head = document.createElement('div'); head.className = 'head';
    head.innerHTML =
      '<span class="d">' + d + '</span><span class="ed" title="' + esc(userOf(ev)) + '">' + esc(userOf(ev)) + '</span>' +
      '<span class="ts">' + ts.toISOString().slice(11, 23) + '</span>' +
      '<span class="ty">' + esc(ty) + '</span><span class="sum">' + esc(sum) + '</span>';
    head.addEventListener('click', function () { row.classList.toggle('open'); });
    var det = document.createElement('pre'); det.className = 'det';
    det.textContent = JSON.stringify(ev.meta, null, 2);
    det.addEventListener('mousedown', function (e) { e.stopPropagation(); }, true);
    det.addEventListener('click', function (e) { e.stopPropagation(); }, true);
    row.appendChild(head); row.appendChild(det);
    return row;
  }

  function visible(ev) {
    var u = userOf(ev);
    var chip = MON.users[u];
    if (!chip) return true;          // unknown user → always show
    return chip.checked;
  }

  function render(ev) {
    ensureUI(); if (!listEl) return;
    if (!visible(ev)) return;
    listEl.appendChild(makeRow(ev));
    while (listEl.childNodes.length > 1000) listEl.removeChild(listEl.firstChild);
    listEl.scrollTop = listEl.scrollHeight;
    updateStat();
  }
  function rebuild() {
    ensureUI(); if (!listEl) return;
    listEl.innerHTML = '';
    MON.events.forEach(function (ev) { if (visible(ev)) listEl.appendChild(makeRow(ev)); });
    listEl.scrollTop = listEl.scrollHeight;
    updateStat();
  }
  function updateStat() {
    if (statEl) statEl.textContent = MON.events.length + ' events';
  }

  // ---- user filter chips --------------------------------------------------
  function addUserChip(name) {
    ensureUI(); if (!chipsEl) return;
    if (chipsEl.querySelector('[data-u="' + cssEsc(name) + '"]')) return;
    var label = document.createElement('label');
    label.className = 'chip'; label.dataset.u = name;
    label.title = name;
    label.innerHTML = '<input type="checkbox"><span></span>';
    label.lastChild.textContent = name;
    var cb = label.firstChild;
    cb.checked = MON.users[name].checked;
    cb.addEventListener('change', function () {
      MON.users[name].checked = cb.checked;
      label.classList.toggle('off', !cb.checked);
      rebuild();
    });
    label.classList.toggle('off', !cb.checked);
    chipsEl.appendChild(label);
  }
  function syncChips() {
    if (!chipsEl) return;
    Array.prototype.forEach.call(chipsEl.children, function (label) {
      var u = label.dataset.u, st = MON.users[u];
      if (!st) return;
      label.firstChild.checked = st.checked;
      label.classList.toggle('off', !st.checked);
    });
  }
  function cssEsc(s) { return String(s).replace(/"/g, '&quot;'); }
  function esc(s) {
    return String(s).replace(/[&<>]/g, function (c) {
      return { '&': '&amp;', '<': '&lt;', '>': '&gt;' }[c];
    });
  }

  if (document.body) buildUI();
  else document.addEventListener('DOMContentLoaded', buildUI, { once: true });
})();
