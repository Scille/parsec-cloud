/*
 * OnlyOffice co-editing protocol monitor — injected in-page script (MITM mode).
 *
 * This script no longer decodes frames itself. Instead it works WITH a local
 * WebSocket man-in-the-middle (proxy.js, started by run-demo.js):
 *
 *   - It patches window.WebSocket to REDIRECT OnlyOffice's co-editing socket
 *     URL (wss://.../doc/{id}/c/EIO=4) to ws://127.0.0.1:PORT/oo?u=<enc>&e=<editor>.
 *     Everything else about the socket stays 100% native — socket.io/engine.io
 *     gets a real WebSocket with real readyState/MessageEvent. This is the one
 *     robust interception point: no proxy-quacking, no property hijacking.
 *
 *   - The proxy terminates both legs (browser<->proxy plain ws, proxy<->OO
 *     real wss), decodes every frame in Node, and can HOLD any non-noise frame
 *     until released. That is the ONLY way to gate RECV (the bytes cannot reach
 *     the browser's socket.io until the proxy forwards them).
 *
 *   - The top frame opens a control WebSocket to ws://127.0.0.1:PORT/ctl. The
 *     proxy pushes decoded events (incl. held rows with holdIds); the panel
 *     renders them. Releasing a row sends {cmd:'release', id} back over /ctl.
 *
 * Flow control: an `auto-flow` / `manual-flow` toggle (next to copy/clear).
 * In auto-flow (default) frames pass through the proxy freely. In manual-flow
 * every non-noise frame is held at the proxy and shown as a ⏸ row with a ▶
 * `send` button; clicking it releases that one frame (recv → delivered to the
 * browser's socket.io; send → forwarded to OnlyOffice).
 *
 * Editor identity (John Smith / Kate Cage / Anonymous) is resolved in the proxy
 * from the `auth` event and pushed to the panel. The panel still auto-filters
 * by the active scenario tab: Collaborate → John & Kate; others → Anonymous.
 *
 * Kate's editor is deferred behind a "Start Kate's editor" overlay. A second
 * button "Re-start John's editor in manual-flow" tears down John's editor,
 * enables manual-flow, and rebuilds it — so John's whole socket handshake can
 * be stepped through frame by frame (useful to test concurrent joins).
 *
 * The XHR polling fallback is still captured in-page (capture-only; it can't
 * be held this way, but OO uses WebSockets in practice).
 *
 * Use with Playwright: injected via ctx.addInitScript() (see run-demo.js),
 *   with window.__OO_PROXY_PORT set BEFORE this script runs.
 * Standalone (no proxy): the redirect is skipped and you get a passive panel
 *   fed only by the XHR fallback — not very useful. Run via run-demo.js.
 */
(function () {
  'use strict';
  if (window.__OO_MON) return; // guard against double injection
  var IS_TOP = (window === window.top);
  var PROXY_PORT = window.__OO_PROXY_PORT || 0;
  var MON = window.__OO_MON = {
    events: [], paused: false, sockets: 0,
    editorUser: {},            // editor -> username (from proxy 'user' msgs)
    sidUser: {},               // sid -> username (from proxy 'user' msgs)
    users: {},                 // username -> {checked:bool}  (filter chips)
    mode: null,               // 'collab' | 'solo' | null (set by active tab)
    intercept: false,         // aka manual-flow: hold non-noise frames (from proxy)
    connected: false          // /ctl control channel open?
  };
  var EDITOR_ID =
    (/[?&]frameEditorId=([^&]*)/.exec(location.search || '') || [, null])[1];

  // ---- Engine.IO v4 + Socket.IO v4 frame decoder (used only for the XHR
  // polling fallback capture, which still happens in-page) ------------------
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
  function authFromConnect(d) {
    if (!d || d.sio !== 'connect' || !d.payload) return null;
    var data = d.payload.data || d.payload;
    if (data && data.type === 'auth' && data.user && data.user.username) return data.user.username;
    return null;
  }
  function isNoise(d) {
    return d && d.eio && (d.eio === 'ping' || d.eio === 'pong' || d.eio === 'noop');
  }

  // ---- editor identity (in-page, for the XHR fallback only) ---------------
  // The proxy is authoritative for identity over WS; this only labels the
  // rare XHR-poll events.
  function noteAuth(ev) {
    if (ev.kind !== 'msg' || ev.dir !== 'send') return;
    var p = ev.meta && ev.meta.payload;
    if (p && p.type === 'auth' && p.user && p.user.username && ev.editor) {
      var name = p.user.username;
      if (MON.editorUser[ev.editor] !== name) {
        MON.editorUser[ev.editor] = name;
        ensureUser(name);
      }
    }
  }
  function ensureUser(name) {
    if (!MON.users[name]) {
      MON.users[name] = { checked: true };
      if (IS_TOP) addUserChip(name);
    }
  }
  function resolveUser(ev) {
    // Identity is keyed by the proxy's per-connection sid (reliable); the
    // editor id from the iframe URL is often empty ('?'), so keying by it
    // would merge all sessions into one slot.
    if (ev.sid && MON.sidUser[ev.sid]) return MON.sidUser[ev.sid];
    if (ev.user) return ev.user;
    return (ev.editor && MON.editorUser[ev.editor]) || ev.editor || '?';
  }
  function userOf(ev) { return resolveUser(ev); }

  // ---- XHR fallback capture (in-page, passive) ---------------------------
  function makeEv(dir, kind, meta) {
    return { t: Date.now(), dir: dir, kind: kind, meta: meta, editor: EDITOR_ID || '?', source: 'xhr' };
  }
  function record(ev) {
    noteAuth(ev);
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

  // =========================================================================
  // Patch WebSocket: REDIRECT OO co-editing sockets to the local MITM proxy.
  // The returned object is a NATIVE WebSocket — we only rewrite the URL.
  // =========================================================================
  var OrigWS = window.WebSocket;
  var OO_RE = /\/doc\/[^/]+\/c\/.*EIO=/;
  // Only redirect once we've confirmed the proxy is actually listening; if it's
  // not (e.g. CSP blocked the probe, or proxy didn't start), fall through to the
  // native URL so OO still loads instead of pointing at a dead port.
  var PROXY_READY = false;
  function probeProxy() {
    if (!PROXY_PORT) return;
    try {
      // Synchronous probe: the init script runs before OO loads, and a local
      // request returns instantly. This guarantees PROXY_READY is set before
      // the first OO WebSocket is constructed (no race).
      var xhr = new XMLHttpRequest();
      xhr.open('GET', 'http://127.0.0.1:' + PROXY_PORT + '/health', false);
      xhr.send(null);
      if (xhr.status === 200 && /ok/.test(xhr.responseText || '')) {
        PROXY_READY = true;
        console.log('[OO Monitor] proxy ready, redirecting OO sockets to ws://127.0.0.1:' + PROXY_PORT);
      } else {
        console.log('[OO Monitor] proxy health unexpected:', xhr.status);
      }
    } catch (e) {
      console.log('[OO Monitor] proxy NOT reachable at port ' + PROXY_PORT + '; running passive (no redirect)');
    }
  }
  probeProxy();
  function PatchedWS(url, protocols) {
    if (PROXY_READY && typeof url === 'string' && OO_RE.test(url)) {
      try {
        // Pass the editor id + subprotocol through as query params so the proxy
        // can label events and negotiate the same subprotocol upstream.
        var enc = encodeURIComponent(url);
        var editor = EDITOR_ID || '';
        var p = protocols
          ? (Array.isArray(protocols) ? protocols[0] : protocols)
          : '';
        var redir = 'ws://127.0.0.1:' + PROXY_PORT + '/oo?u=' + enc +
          (editor ? '&e=' + encodeURIComponent(editor) : '') +
          (p ? '&p=' + encodeURIComponent(p) : '');
        if (protocols) return new OrigWS(redir, protocols);
        return new OrigWS(redir);
      } catch (e) { /* fall through to native */ }
    }
    return protocols ? new OrigWS(url, protocols) : new OrigWS(url);
  }
  PatchedWS.prototype = OrigWS.prototype;
  PatchedWS.CONNECTING = OrigWS.CONNECTING; PatchedWS.OPEN = OrigWS.OPEN;
  PatchedWS.CLOSING = OrigWS.CLOSING; PatchedWS.CLOSED = OrigWS.CLOSED;
  window.WebSocket = PatchedWS;

  // Patch XHR (polling fallback) — capture only, in-page.
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

  // ---- top frame: control channel + scenario tabs + Kate deferral ---------
  if (IS_TOP) {
    // forwarded XHR-fallback events from child frames
    window.addEventListener('message', function (e) {
      var d = e.data;
      if (!d || d.__oo_mon !== 1 || !d.ev) return;
      noteAuth(d.ev);
      var u = resolveUser(d.ev);
      if (u && u !== d.ev.editor) d.ev.user = u;
      store(d.ev);
    });

    // ---- control channel to the local MITM proxy -------------------------
    var ctl = null;
    function connectCtl() {
      if (!PROXY_PORT) return;
      try {
        ctl = new OrigWS('ws://127.0.0.1:' + PROXY_PORT + '/ctl');
      } catch (e) { setTimeout(connectCtl, 1000); return; }
      ctl.addEventListener('open', function () {
        MON.connected = true;
        syncInterceptBtn();
      });
      ctl.addEventListener('close', function () {
        MON.connected = false; ctl = null;
        syncInterceptBtn();
        setTimeout(connectCtl, 1000); // auto-reconnect
      });
      ctl.addEventListener('error', function () { /* close will fire */ });
      ctl.addEventListener('message', function (e) {
        var msg; try { msg = JSON.parse(e.data); } catch (err) { return; }
        handleProxyMsg(msg);
      });
    }
    // per-row hold state (holdId -> {row, released, ok})
    var HOLD_STATE = {};
    function handleProxyMsg(msg) {
      switch (msg.t) {
        case 'hello': {
          MON.intercept = !!msg.intercept;
          syncInterceptBtn();
          // replay history
          if (Array.isArray(msg.events)) {
            msg.events.forEach(function (ev) {
              if (ev.user) { ensureUser(ev.user); MON.sidUser[ev.sid] = ev.user; MON.editorUser[ev.editor] = ev.user; }
              storeProxyEvent(ev);
            });
          }
          if (msg.users) for (var ed in msg.users) { MON.editorUser[ed] = msg.users[ed]; ensureUser(msg.users[ed]); }
          break;
        }
        case 'event': {
          var ev = msg.ev;
          if (ev.user) { ensureUser(ev.user); MON.sidUser[ev.sid] = ev.user; MON.editorUser[ev.editor] = ev.user; }
          storeProxyEvent(ev);
          break;
        }
        case 'user': {
          MON.sidUser[msg.sid] = msg.user;
          MON.editorUser[msg.editor] = msg.user;
          ensureUser(msg.user);
          // Re-render rows for this session now that we know the user
          // (e.g. ws-open was shown as '?' before the auth frame arrived).
          rebuild();
          break;
        }
        case 'intercept': {
          MON.intercept = !!msg.on;
          syncInterceptBtn();
          break;
        }
        case 'released': {
          var h = HOLD_STATE[msg.id];
          if (h) { h.released = true; h.ok = !!msg.ok; if (h.row) updateRowReleased(h.row, h.ok); }
          break;
        }
        case 'stale': {
          var h2 = HOLD_STATE[msg.id];
          if (h2) { h2.released = true; h2.ok = false; if (h2.row) updateRowReleased(h2.row, false); }
          break;
        }
      }
    }
    function storeProxyEvent(ev) {
      MON.events.push(ev);
      if (MON.events.length > 5000) MON.events.shift();
      if (!MON.paused && visible(ev)) render(ev);
    }
    function sendCtl(obj) {
      if (ctl && ctl.readyState === OrigWS.OPEN) {
        try { ctl.send(JSON.stringify(obj)); } catch (e) {}
      }
    }

    // Initialize Kate-block state early (referenced by applyScenario below).
    MON.kateBlocked = true; MON.kateConfig = null; MON.kateOrig = null;

    // Observe the scenario tab buttons. "Collaborate" → John & Kate;
    // every other scenario → "Anonymous".
    function activeScenario() {
      var btn = document.querySelector('[class*="actions-tab-button"][class*="active"], [class*="actions-tab-button"].active');
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
      if (collab) MON.kateBlocked = true;
      for (var name in MON.users) {
        var isAnon = /anonymous/i.test(name);
        MON.users[name].checked = collab ? !isAnon : isAnon;
      }
      syncChips();
      rebuild();
    }
    var lastTab = null;
    setInterval(function () {
      var t = activeScenario();
      if (t && t !== lastTab) { lastTab = t; applyScenario(t); }
    }, 500);

    // ---- Editor construction trap ----------------------------------------
    // The demo builds editors via `new DocsAPI.DocEditor(id, config)`. We
    // wrap the constructor to:
    //   - defer `document-editor-2` (Kate) until "Start Kate's editor" is
    //     clicked (lets you edit solo in John first, then watch Kate's join).
    //   - remember `document-editor` (John)'s config so "Re-start John's editor
    //     in manual-flow" can tear it down + rebuild it with manual-flow armed
    //     BEFORE its socket opens (so the whole handshake can be stepped).
    MON.kateBlocked = true; MON.kateConfig = null; MON.kateOrig = null;
    MON.johnConfig = null;     // {id, cfg} captured from John's constructor
    MON.johnInstance = null;   // last returned DocEditor instance for John
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
            var inst = new orig(id, cfg);
            if (id === 'document-editor') {
              MON.johnConfig = { id: id, cfg: cfg };
              MON.johnInstance = inst;
            }
            return inst;
          }
          W.__oo_wrapped = true; W.prototype = orig.prototype; W.__oo_orig = orig;
          D.DocEditor = W;
        }
      }
      setInterval(wrap, 50);
    })();
    MON.startKate = function () {
      MON.kateBlocked = false;
      removeKateOverlay();
      if (MON.kateConfig && MON.kateOrig) {
        var k = MON.kateConfig;
        MON.kateConfig = null;
        new MON.kateOrig(k.id, k.cfg);
      }
    };
    // Re-start John's editor: destroy the current instance, arm manual-flow,
    // clear the panel, then rebuild John's editor so its socket handshake is
    // captured frame-by-frame from the very first frame.
    MON.restartJohnManualFlow = function () {
      if (MON.johnInstance) {
        try { if (typeof MON.johnInstance.destroy === 'function') MON.johnInstance.destroy(); } catch (e) {}
        try { if (typeof MON.johnInstance.deInit === 'function') MON.johnInstance.deInit(); } catch (e) {}
        MON.johnInstance = null;
      }
      // remove any leftover OO iframes for John so DocsAPI rebuilds cleanly
      var slot = document.getElementById('document-editor');
      if (slot) slot.innerHTML = '';
      // arm manual-flow (both locally and at the proxy)
      MON.intercept = true; syncInterceptBtn();
      sendCtl({ cmd: 'intercept', on: true });
      // clear the event log for a clean read of the handshake
      MON.events.length = 0; HOLD_STATE = {}; rebuild(); updateStat();
      if (MON.johnConfig && MON.kateOrig) {
        var j = MON.johnConfig;
        MON.johnConfig = null;
        MON.johnInstance = new MON.kateOrig(j.id, j.cfg);
      }
    };
    function removeKateOverlay() {
      var o = document.getElementById('oo-kate-overlay'); if (o) o.remove();
    }
    setInterval(function () {
      if (!MON.kateBlocked) { removeKateOverlay(); return; }
      if (MON.mode !== 'collab') { removeKateOverlay(); return; }
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
        // second button: re-start John in manual-flow
        var rb = document.createElement('button');
        rb.textContent = 'Re-start John\'s editor in manual-flow';
        rb.style.cssText = 'background:#7a5b00;color:#fff;border:0;border-radius:6px;padding:8px 14px;cursor:pointer;font:inherit';
        rb.addEventListener('click', function () { MON.restartJohnManualFlow(); });
        overlay.appendChild(rb);
        var hint = document.createElement('div');
        hint.style.cssText = 'font-size:11px;color:#888';
        hint.textContent = "edit in John's editor first, then start Kate — or re-start John in manual-flow to step through his handshake";
        overlay.appendChild(hint);
      }
      if (target.style.position !== 'relative') target.style.position = 'relative';
      if (overlay.parentElement !== target) target.appendChild(overlay);
    }, 500);

    connectCtl();
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
      '#oo-mon .hdr button.on{background:#ffd34d;color:#3a2a00;font-weight:700}',
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
      '#oo-mon .row.held .head{background:#fff4d6} #oo-mon .row.held.released .head{opacity:.5}',
      '#oo-mon .head{display:flex;gap:6px;padding:3px 6px;cursor:pointer;align-items:baseline}',
      '#oo-mon .head:hover{filter:brightness(.97)}',
      '#oo-mon .head .d{flex:0 0 16px} #oo-mon .head .ed{flex:0 0 80px;color:#7a5b00;font-size:10px;overflow:hidden;text-overflow:ellipsis}',
      '#oo-mon .head .ts{flex:0 0 66px;color:#888} #oo-mon .head .ty{flex:0 0 96px;font-weight:600}',
      '#oo-mon .head .sum{flex:1;color:#333;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}',
      '#oo-mon .head .rl{flex:0 0 56px;text-align:right}',
      '#oo-mon .head .rl button{font:inherit;font-size:10px;padding:0 6px;border:1px solid #b5932b;border-radius:3px;background:#fff1c2;cursor:pointer;color:#5a4200}',
      '#oo-mon .head .rl button:disabled{opacity:.5;cursor:default}',
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
      '<button id="oo-mon-copy">copy</button><button id="oo-mon-clear">clear</button>' +
      '<button id="oo-mon-flow" title="toggle manual flow (hold frames until released)">auto-flow</button></div>' +
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

    // resize
    var resizer = document.getElementById('oo-mon-resize');
    var resizing = false, rsx = 0, rsy = 0, rsw = 0, rsh = 0;
    resizer.addEventListener('mousedown', function (e) {
      e.preventDefault(); e.stopPropagation();
      resizing = true;
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
      MON.events.length = 0; HOLD_STATE = {}; rebuild(); updateStat();
    });
    var flb = document.getElementById('oo-mon-flow');
    flb.addEventListener('click', function () {
      MON.intercept = !MON.intercept;
      syncInterceptBtn();
      sendCtl({ cmd: 'intercept', on: MON.intercept });
    });
    document.getElementById('oo-mon-copy').addEventListener('click', function () {
      var text = MON.events.filter(visible).map(formatBlock).join('\n\n');
      copyText(text, document.getElementById('oo-mon-copy'));
    });
    listEl = document.getElementById('oo-mon-list');
    statEl = document.getElementById('oo-mon-stat');
    chipsEl = document.getElementById('oo-mon-chips');
    Object.keys(MON.users).forEach(addUserChip);
    syncChips();
    syncInterceptBtn();
  }

  var listEl, statEl, chipsEl;

  function syncInterceptBtn() {
    var flb = document.getElementById('oo-mon-flow');
    if (!flb) return;
    flb.textContent = MON.intercept ? 'manual-flow' : 'auto-flow';
    flb.classList.toggle('on', !!MON.intercept);
  }

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

  function typeOf(ev) {
    if (ev.dir === 'open') return 'ws-open';
    if (ev.dir === 'engine') return ev.kind;
    if (ev.dir === 'held') return 'HELD ' + (ev.kind === 'msg' ? ev.meta.type : (ev.meta && ev.meta.eio ? ev.meta.eio : ''));
    if (ev.kind === 'msg') return ev.meta.type;
    if (ev.kind === 'eio') return ev.meta.eio + (ev.meta.sio ? '/' + ev.meta.sio : '');
    return ev.kind;
  }
  function formatLine(ev) {
    var ts = new Date(ev.t).toISOString().slice(11, 23);
    var dir = ev.dir === 'send' ? '->' : ev.dir === 'recv' ? '<-' : ev.dir === 'held' ? '##' : '  ';
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
    else if (ev.dir === 'held') {
      d = '⏸'; ty = typeOf(ev);
      sum = ev.kind === 'msg' ? summarize(ev.meta.payload) : (ev.meta && ev.meta.eio ? ev.meta.eio : '');
    }
    else if (ev.kind === 'msg') { d = ev.dir === 'send' ? '⬆' : '⬇'; ty = ev.meta.type; sum = summarize(ev.meta.payload); }
    else if (ev.kind === 'eio') { d = '⚙'; ty = ev.meta.eio + (ev.meta.sio ? '/' + ev.meta.sio : ''); }
    else { d = ev.dir === 'send' ? '⬆' : '⬇'; ty = ev.kind; }
    var ts = new Date(ev.t);
    var head = document.createElement('div'); head.className = 'head';
    head.innerHTML =
      '<span class="d">' + d + '</span><span class="ed" title="' + esc(userOf(ev)) + '">' + esc(userOf(ev)) + '</span>' +
      '<span class="ts">' + ts.toISOString().slice(11, 23) + '</span>' +
      '<span class="ty">' + esc(ty) + '</span><span class="sum">' + esc(sum) + '</span>';
    if (ev.dir === 'held' && ev.holdId) {
      var rl = document.createElement('span'); rl.className = 'rl';
      var rb = document.createElement('button');
      rb.textContent = 'send'; rb.title = 'release this frame through the proxy';
      var hs = HOLD_STATE[ev.holdId] = HOLD_STATE[ev.holdId] || { released: false, ok: null, row: row };
      if (hs.released) {
        row.classList.add('released');
        rb.textContent = hs.ok === false ? 'stale' : 'sent'; rb.disabled = true;
      } else {
        rb.addEventListener('click', function (e) {
          e.stopPropagation();
          rb.disabled = true; rb.textContent = '…';
          sendCtl({ cmd: 'release', id: ev.holdId });
        });
      }
      rl.appendChild(rb);
      head.appendChild(rl);
    }
    head.addEventListener('click', function () { row.classList.toggle('open'); });
    var det = document.createElement('pre'); det.className = 'det';
    det.textContent = JSON.stringify(ev.meta, null, 2);
    det.addEventListener('mousedown', function (e) { e.stopPropagation(); }, true);
    det.addEventListener('click', function (e) { e.stopPropagation(); }, true);
    row.appendChild(head); row.appendChild(det);
    return row;
  }
  function updateRowReleased(row, ok) {
    row.classList.add('released');
    var btns = row.querySelectorAll('.rl button');
    if (btns.length) { btns[0].textContent = ok ? 'sent' : 'stale'; btns[0].disabled = true; }
  }

  function visible(ev) {
    var u = userOf(ev);
    var chip = MON.users[u];
    if (!chip) return true;
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
