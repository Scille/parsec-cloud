/*
 * OnlyOffice co-editing protocol monitor — injected in-page script (MITM mode).
 *
 * This script no longer decodes frames itself. Instead it works WITH a local
 * WebSocket man-in-the-middle (proxy.js, started by run.js):
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
 * every non-noise frame is held at the proxy and shown as a row (looks like a
 * normal send/recv row — same icon, no special colour) with a `send` button; clicking it releases that one frame (recv → delivered to the
 * browser's socket.io; send → forwarded to OnlyOffice).
 *
 * Editor identity (Alice / Bob / Mallory — see index.html) is resolved in the
 * proxy from the `auth` event and pushed to the panel. The panel can filter
 * the rows per-user with its chips.
 *
 * OnlyOffice version: the doc server announces its build on the wire (the
 * `license` push and the `auth` response carry buildVersion + buildNumber);
 * the panel shows it in its header, and `copy` exports it in a `## Info`
 * block (timestamp of the copy + version) before the `## Events` blocks.
 *
 * The manually-started editors (Bob / Mallory) are deferred behind
 * per-editor "Start …'s editor" overlays. The first one also has a
 * "Re-start …'s editor in manual-flow" button, which reloads the whole page
 * with `manual_flow=true` in the URL and arms manual-flow at the proxy first —
 * so the first client's whole socket handshake can be stepped through frame
 * by frame from a guaranteed clean state (useful to test concurrent joins).
 *
 * The XHR polling fallback is still captured in-page (capture-only; it can't
 * be held this way, but OO uses WebSockets in practice).
 *
 * Use with Playwright: injected via ctx.addInitScript() (see run.js), with
 *   window.__OO_PROXY_PORT set BEFORE this script runs.
 */
(function () {
  'use strict';
  if (window.__OO_MON) return; // guard against double injection
  var IS_TOP = (window === window.top);
  var PROXY_PORT = window.__OO_PROXY_PORT || 0;
  // `manual_flow=true` in the page URL (set by the "Re-start …'s editor in
  // manual-flow" button) requests manual-flow from the very first frame.
  var INITIAL_MANUAL_FLOW = /[?&]manual_flow=true/i.test(location.search || '');
  var MON = window.__OO_MON = {
    events: [], paused: false, sockets: 0,
    editorUser: {},            // editor -> username (from proxy 'user' msgs)
    sidUser: {},               // sid -> username (from proxy 'user' msgs)
    users: {},                 // username -> {checked:bool}  (filter chips)
    intercept: false,         // aka manual-flow: hold non-noise frames (from proxy)
    connected: false,         // /ctl control channel open?
    pageResetting: false,     // true during the page-reload for manual-flow restart
    ooVersion: null,          // OnlyOffice buildVersion in use (e.g. '9.4.1'), from wire events
    ooBuild: null,            // matching buildNumber (e.g. 15)
    editorConfigs: {},        // editor id -> config (kept for "re-join" after a close)
    editorInstances: {},     // editor id -> live DocEditor instance (for "close editor")
    editorNetState: {}        // editor id -> 'online' | 'offline' | 'cut' | 'closed' | ''
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
    // Return the actual OnlyOffice wire event (`p`) directly — NOT a synthetic
    // {type, payload} wrapper. The event already carries its own `type` field
    // (OO's convention), so hoisting it into an envelope only produced a
    // misleading double-`type` in the exported logs. Consumers read `ev.meta`
    // as the real event (ev.meta.type, ev.meta.user, ...).
    return p;
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
    var p = ev.meta;
    if (p && p.type === 'auth' && p.user && p.user.username && ev.editor) {
      var name = p.user.username;
      if (MON.editorUser[ev.editor] !== name) {
        MON.editorUser[ev.editor] = name;
        ensureUser(name);
      }
    }
  }
  // ---- OnlyOffice version (captured from the wire) -----------------------
  // The document server announces its build in two OO events we already
  // decode: the `license` push (right after the socket handshake) and the
  // `auth` response (server→client; the client's own `auth` has no `result`).
  // Reading it from the wire is authoritative and works for both the proxy
  // and the XHR-fallback capture paths (no cross-origin iframe probing).
  function noteVersion(ev) {
    if (MON.ooVersion) return; // already known (survives `clear`)
    var p = ev && ev.meta;
    if (!p || typeof p !== 'object') return;
    var v = null;
    if (p.type === 'license' && p.license && p.license.buildVersion != null) v = p.license;
    else if (p.type === 'auth' && p.result && p.buildVersion != null) v = p;
    if (v) {
      MON.ooVersion = String(v.buildVersion);
      MON.ooBuild = v.buildNumber != null ? v.buildNumber : null;
      updateVersionUI();
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
    noteVersion(ev);
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

  // ---- editor-frame side of the disconnection actions ---------------------
  // The monitor script runs in every frame (Playwright injects init scripts
  // in child frames too), so the top frame — which cannot reach into the
  // cross-origin editor iframe — just posts a message and the editor's own
  // monitor instance runs the action.
  window.addEventListener('message', function (e) {
    var d = e.data;
    if (!d || d.__oo_mon_ctl !== 1) return;
    if (d.cmd === 'oo-close') {
      // The protocol's own graceful goodbye: the editor API call OnlyOffice
      // uses when it must tell the server it is leaving (rights revocation,
      // anonymous mode…). DocsCoApi.disconnect() sends the `close` client
      // event, and the document server closes the connection.
      try {
        var api = window['Asc'] && window['Asc'].editor;
        if (api && typeof api.asc_coAuthoringDisconnect === 'function') api.asc_coAuthoringDisconnect();
      } catch (err) {}
    }
  });

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

  // ---- top frame: control channel + scenario tabs + deferred editors -----
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
        // If the page was opened with ?manual_flow=true (Re-start the first
        // editor in manual-flow), arm manual-flow at the proxy before any OO
        // socket opens. We do NOT reset the proxy here: on reload the previous
        // page's sockets already closed (clearing those sessions + holds at the
        // proxy), and closing active sessions now would race with the first
        // client's reconnecting socket and kill it. Only stale event history
        // lingers, which we just ignore by starting the panel empty (see
        // INITIAL_MANUAL_FLOW below).
        if (INITIAL_MANUAL_FLOW && !MON._initialFlowApplied) {
          MON._initialFlowApplied = true;
          sendCtl({ cmd: 'intercept', on: true });
        }
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
          // In manual-flow-restart mode, start the panel empty (ignore stale
          // history from the previous page; only show frames from this run).
          var evs = Array.isArray(msg.events) ? msg.events : [];
          if (INITIAL_MANUAL_FLOW && !MON._helloSeen) {
            MON.events.length = 0; HOLD_STATE = {};
            evs = [];
          }
          MON._helloSeen = true;
          evs.forEach(function (ev) {
            if (ev.user) { ensureUser(ev.user); MON.sidUser[ev.sid] = ev.user; MON.editorUser[ev.editor] = ev.user; }
            storeProxyEvent(ev);
          });
          // backfill sid->user (getUsers() is keyed by sid)
          if (msg.users) for (var sid in msg.users) { MON.sidUser[sid] = msg.users[sid]; ensureUser(msg.users[sid]); }
          rebuild();
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
        case 'cut': {
          if (msg.editor) MON.editorNetState[msg.editor] = 'cut';
          break;
        }
        case 'revive': {
          if (msg.editor) MON.editorNetState[msg.editor] = MON.editorInstances[msg.editor] ? 'online' : 'closed';
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
        case 'releasedAll': {
          // all held frames flushed (switching back to auto-flow)
          for (var id in HOLD_STATE) {
            var hh = HOLD_STATE[id];
            if (!hh.released) { hh.released = true; hh.ok = true; if (hh.row) updateRowReleased(hh.row, true); }
          }
          break;
        }
      }
    }
    function storeProxyEvent(ev) {
      noteVersion(ev);
      if (ev.editor) {
        // per-editor connection state for the pane control bars (a deferred
        // editor that was never started keeps its empty state)
        if (ev.dir === 'open') MON.editorNetState[ev.editor] = 'online';
        else if (ev.dir === 'engine' && ev.kind === 'ws-close') {
          var prev = MON.editorNetState[ev.editor];
          if (prev !== 'closed' && prev !== 'cut') MON.editorNetState[ev.editor] = 'offline';
        }
      }
      MON.events.push(ev);
      if (MON.events.length > 5000) MON.events.shift();
      if (!MON.paused && visible(ev)) render(ev);
    }
    function sendCtl(obj) {
      if (ctl && ctl.readyState === OrigWS.OPEN) {
        try { ctl.send(JSON.stringify(obj)); } catch (e) {}
      }
    }
    // Expose on the live MON so panel handlers always dispatch through the
    // current run's ctl (robust to the page's reloads, which create a fresh
    // __OO_MON and reconnect /ctl).
    MON.sendCtl = sendCtl;

    // ---- Editor construction trap ----------------------------------------
    // The page builds editors via `new DocsAPI.DocEditor(id, config)`. We
    // wrap the constructor to defer the manually-started clients —
    // `document-editor-2` (Bob) and `document-editor-3` (Mallory) — until their
    // "Start …'s editor" overlay button is clicked, so you can edit solo in
    // the first client first, then watch the others' late-join. Each overlay
    // is labelled with the editor's own user name, read from its config
    // (`cfg.editorConfig.user.name`). The trap also remembers every editor's
    // config and live instance: the disconnection actions (close / cut /
    // re-join) need them. (Re-starting the first editor is handled by a full
    // page reload, not by tearing down DocEditor here — see
    // restartFirstManualFlow.)
    var DEFERRED_EDITOR_IDS = ['document-editor-2', 'document-editor-3'];
    // `?defer1=1` also defers the first editor (Alice) behind an overlay — used
    // to record scenarios where another participant must connect first (e.g.
    // a read-only viewer creating the session).
    if (new URLSearchParams(location.search).get('defer1') === '1') {
      DEFERRED_EDITOR_IDS.push('document-editor-1');
    }
    MON.deferred = {};      // editor id -> { id, cfg } pending construction
    MON.deferStarted = {};  // editor id -> true once started (page-internal
                            // editor rebuilds must not re-defer it)
    MON.deferOrig = null;   // unwrapped DocsAPI.DocEditor
    (function installDocEditorTrap() {
      if (window.__oo_doceditor_trap) return;
      window.__oo_doceditor_trap = true;
      function wrap() {
        var D = window.DocsAPI;
        if (D && D.DocEditor && !D.DocEditor.__oo_wrapped) {
          var orig = D.DocEditor;
          MON.deferOrig = orig;
          function W(id, cfg) {
            MON.editorConfigs[id] = cfg;
            if (DEFERRED_EDITOR_IDS.indexOf(id) >= 0 && !MON.deferStarted[id]) {
              MON.deferred[id] = { id: id, cfg: cfg };
              return { __ooDeferred: true };
            }
            MON.editorInstances[id] = new orig(id, cfg);
            return MON.editorInstances[id];
          }
          W.__oo_wrapped = true; W.prototype = orig.prototype; W.__oo_orig = orig;
          D.DocEditor = W;
        }
      }
      setInterval(wrap, 50);
    })();
    MON.startDeferred = function (id) {
      var e = MON.deferred[id];
      if (!e || !MON.deferOrig) return;
      delete MON.deferred[id];
      MON.deferStarted[id] = true;
      removeDeferredOverlay(id);
      MON.editorInstances[id] = new MON.deferOrig(e.id, e.cfg);
    };
    // Re-start the first editor in manual-flow: reload the whole page with
    // `manual_flow=true` in the URL. On reload, the init script sees that
    // param and (once the /ctl control channel opens) sends {intercept,on:true}
    // to the proxy — so the first client's socket, which only opens after the
    // page rebuilds, is gated from its very first frame and the whole
    // handshake shows up as held rows. Reloading guarantees a fully clean
    // state (the previous page's sockets close on unload, clearing those
    // sessions/holds at the proxy).
    MON.restartFirstManualFlow = function () {
      MON.pageResetting = true;
      try {
        var u = new URL(location.href);
        u.searchParams.set('manual_flow', 'true');
        location.href = u.toString();
      } catch (e) {
        location.href = location.href + (location.href.indexOf('?') >= 0 ? '&' : '?') + 'manual_flow=true';
      }
    };

    // ---- disconnection actions (panel buttons + headless recorder) -------
    // Close the editor the way the Parsec client does when the user closes
    // the file: destroy the DocEditor instance. Nothing is sent on the wire —
    // no `close` client event, the websocket dies with the editor's iframe —
    // and the client cannot auto-reconnect: the editor is gone.
    MON.closeEditor = function (id) {
      var inst = MON.editorInstances[id];
      if (!inst) return false;          // never started (still deferred) or already closed
      delete MON.editorInstances[id];
      try { if (typeof inst.destroyEditor === 'function') inst.destroyEditor(); } catch (e) {}
      MON.editorNetState[id] = 'closed';
      return true;
    };
    // Send the protocol's own `close` client event (the API call OnlyOffice
    // uses when it must tell the server it is leaving, e.g. on rights
    // revocation). The editor is cross-origin, so the request goes through a
    // postMessage handled by the monitor script running inside the editor's
    // own frame (which calls asc_coAuthoringDisconnect()).
    MON.sendCloseEvent = function (id) {
      // The editor api REPLACES the placeholder div with the editor iframe
      // (its src carries frameEditorId=<id>), so look the frame up by its id
      // param — not inside the (now gone) placeholder.
      var frs = document.querySelectorAll('iframe[name="frameEditor"]');
      var fr = null;
      for (var i = 0; i < frs.length; i++) {
        try { if (decodeURIComponent(frs[i].src || '').indexOf('frameEditorId=' + id) >= 0) { fr = frs[i]; break; } }
        catch (e) { /* ignore */ }
      }
      if (!fr || !fr.contentWindow) return false;
      try { fr.contentWindow.postMessage({ __oo_mon_ctl: 1, cmd: 'oo-close' }, '*'); } catch (e) { return false; }
      return true;
    };
    // `close` event + destroy: record the graceful goodbye on the wire (the
    // document server closes the connection after it — the ws-close row
    // shows it), then make sure the editor does not linger or auto-reconnect.
    MON.closeEditorWithCloseEvent = function (id) {
      if (!MON.sendCloseEvent(id)) return MON.closeEditor(id);
      setTimeout(function () { MON.closeEditor(id); }, 2000);
      return true;
    };
    // Lose the connection: the proxy terminates both legs of the socket
    // without a close frame. The editor stays open and auto-reconnects
    // (~2s, per the settings.reconnection of its auth event), but while the
    // editor is marked cut the proxy kills every reconnection attempt, so
    // the participant stays away until reviveConnection/re-join.
    MON.cutConnection = function (id) {
      MON.sendCtl({ cmd: 'cut', editor: id, keep: 'dead' });
      MON.editorNetState[id] = 'cut';
    };
    MON.reviveConnection = function (id) {
      MON.sendCtl({ cmd: 'revive', editor: id });
      MON.editorNetState[id] = MON.editorInstances[id] ? 'online' : 'closed';
    };
    // Bring a participant back. After a cut: just revive — the client's own
    // reconnection retry then completes the session restore (its re-`auth`
    // carries the previous sessionId). After a close: also re-create the
    // editor from its remembered config (a fresh handshake, sessionId null).
    MON.rejoinEditor = function (id) {
      MON.sendCtl({ cmd: 'revive', editor: id });   // clear any cut mark first
      if (MON.deferred[id]) { MON.startDeferred(id); return true; }
      if (MON.editorInstances[id]) return false;   // editor still open: its retry does the rest
      var cfg = MON.editorConfigs[id];
      var D = window.DocsAPI;
      if (!cfg || !D || !D.DocEditor) return false;
      MON.deferStarted[id] = true;                  // never re-defer a re-join
      // Build through the (still installed) wrapped constructor so the new
      // instance gets tracked like the first one.
      var inst = new D.DocEditor(id, cfg);
      MON.editorInstances[id] = inst;
      MON.editorNetState[id] = 'online';
      return true;
    };
    // Display name of a deferred editor (from its config).
    function deferredName(id) {
      var e = MON.deferred[id];
      return (e && e.cfg && e.cfg.editorConfig && e.cfg.editorConfig.user &&
              e.cfg.editorConfig.user.name) || id;
    }
    // Display name of the first editor, resolved from the wire once its
    // `auth` event has been seen (null before that).
    function firstName() {
      return MON.editorUser['document-editor'] || MON.editorUser['document-editor-1'] || null;
    }
    // The N-th `.actions-content` pane hosts `document-editor-N`.
    function paneForEditorId(id, contents) {
      var m = /(\d+)$/.exec(id || '');
      return m ? contents[parseInt(m[1], 10) - 1] : contents[1];
    }
    function removeDeferredOverlay(id) {
      var o = document.getElementById('oo-defer-overlay-' + id); if (o) o.remove();
    }
    setInterval(function () {
      var contents = Array.from(document.querySelectorAll('div[class*="actions-container"] > div[class*="actions-content"]'));
      var pending = Object.keys(MON.deferred);
      // Drop overlays of editors that are no longer pending (started).
      Array.from(document.querySelectorAll('[data-oo-defer-overlay]'))
        .forEach(function (o) {
          if (pending.indexOf(o.getAttribute('data-editor')) < 0) o.remove();
        });
      // Ensure one overlay per pending editor, over its own pane.
      pending.forEach(function (id) {
        var target = paneForEditorId(id, contents);
        if (!target) return;
        var overlay = document.getElementById('oo-defer-overlay-' + id);
        if (!overlay) {
          overlay = document.createElement('div');
          overlay.id = 'oo-defer-overlay-' + id;
          overlay.setAttribute('data-oo-defer-overlay', '1');
          overlay.setAttribute('data-editor', id);
          overlay.style.cssText = 'position:absolute;inset:0;display:flex;align-items:center;justify-content:center;flex-direction:column;gap:10px;background:rgba(250,250,252,.96);border:1px dashed #c8ccd4;border-radius:8px;z-index:5;font:14px/1.4 ui-monospace,monospace;color:#444;text-align:center;padding:16px';
          var label = document.createElement('div'); label.textContent = deferredName(id) + "'s editor is on hold";
          overlay.appendChild(label);
          var b = document.createElement('button');
          b.textContent = 'Start ' + deferredName(id) + "'s editor";
          b.style.cssText = 'background:#3b5b8c;color:#fff;border:0;border-radius:6px;padding:8px 14px;cursor:pointer;font:inherit';
          b.addEventListener('click', function () { MON.startDeferred(id); });
          overlay.appendChild(b);
          // Only on the first deferred overlay: the manual-flow restart button
          // + hint (any overlay can trigger it, one is enough).
          if (id === DEFERRED_EDITOR_IDS[0]) {
            var rb = document.createElement('button');
            rb.style.cssText = 'background:#7a5b00;color:#fff;border:0;border-radius:6px;padding:8px 14px;cursor:pointer;font:inherit';
            rb.addEventListener('click', function () { MON.restartFirstManualFlow(); });
            overlay.appendChild(rb);
            var hint = document.createElement('div');
            hint.style.cssText = 'font-size:11px;color:#888';
            overlay.appendChild(hint);
            overlay.__ooDyn = { rb: rb, hint: hint };
          }
        }
        // Refresh the dynamic texts once the first editor's name is captured.
        if (overlay.__ooDyn) {
          var fn = firstName();
          overlay.__ooDyn.rb.textContent = fn
            ? "Re-start " + fn + "'s editor in manual-flow"
            : 'Re-start the first editor in manual-flow';
          overlay.__ooDyn.hint.textContent = fn
            ? "edit in " + fn + "'s editor first, then start the others — or re-start " + fn + " in manual-flow to step through the handshake"
            : 'edit in the first editor first, then start the others — or re-start it in manual-flow to step through the handshake';
        }
        if (target.style.position !== 'relative') target.style.position = 'relative';
        if (overlay.parentElement !== target) target.appendChild(overlay);
      });
    }, 500);

    // ---- per-editor control bar (disconnect / reconnection) ---------------
    // A small horizontal bar docked to each editor pane's top-right corner,
    // driving the disconnection actions above — the crafting primitives for
    // the 60x record scenarios. It is compact (icon buttons + tooltips) and,
    // like the "OO Protocol Monitor" panel itself, movable: dragging its
    // ⠿ grip detaches it from the pane corner.
    //   ✕ (close)      graceful user-close (editor destroyed, nothing sent)
    //   ✉ (close:msg)  the protocol's `close` event, then the editor destroyed
    //   ⚡ (cut)       connection lost (no close frame); auto-reconnect
    //                  attempts are killed until re-join
    //   ↻ (re-join)    bring the participant back (session restore after a
    //                  cut, fresh editor after a close)
    var HARNESS_EDITORS = ['document-editor-1', 'document-editor-2', 'document-editor-3'];
    function ensureEditorBar(id, pane) {
      var bar = document.getElementById('oo-mon-edctl-' + id);
      if (!bar) {
        bar = document.createElement('div');
        bar.id = 'oo-mon-edctl-' + id;
        // `height:auto` is explicit on purpose: the harness page styles
        // `.actions-content > div { height: 100% }` (for the editor placeholder
        // divs) and this bar is also a div directly under the pane — without
        // the inline override it would stretch to the pane's full height.
        bar.style.cssText = 'position:absolute;top:6px;right:6px;z-index:6;display:flex;align-items:center;gap:3px;' +
          'height:auto;width:auto;' +
          'background:rgba(250,250,252,.92);border:1px solid #c8ccd4;border-radius:6px;padding:1px 3px;' +
          'font:10px/1.2 ui-monospace,monospace;color:#444;white-space:nowrap';
        // ⠿ grip + editor name: the drag handle (like the main panel's header)
        var grip = document.createElement('span');
        grip.style.cssText = 'cursor:move;user-select:none;padding:1px 3px;color:#666;font-weight:700';
        grip.textContent = '⠿';
        var name = document.createElement('span');
        name.style.cssText = 'color:#666';
        grip.appendChild(name);
        bar.appendChild(grip);
        // connection state: a colored dot (the state text is its tooltip)
        var st = document.createElement('span');
        st.style.cssText = 'font-weight:700;color:#999';
        st.textContent = '●';
        bar.appendChild(st);
        bar.__ooStateEl = st;
        bar.__ooNameEl = name;
        bar.__ooDragged = false;
        function mk(label, title, fn) {
          var b = document.createElement('button');
          b.textContent = label;
          b.title = title;
          b.style.cssText = 'border:1px solid #c8ccd4;border-radius:4px;background:#fff;color:#444;cursor:pointer;padding:0 4px;font:inherit';
          b.addEventListener('click', function (e) { e.stopPropagation(); fn(); });
          bar.appendChild(b);
        }
        mk('✕', "Graceful close: destroy this editor the way the Parsec client does when the user closes the file (nothing is sent on the wire, no auto-reconnect)", function () { MON.closeEditor(id); });
        mk('✉', "Send the protocol's `close` client event (the OnlyOffice API call used on rights revocation), then destroy the editor", function () { MON.closeEditorWithCloseEvent(id); });
        mk('⚡', 'Lose the connection: the socket is killed without a close frame; the editor auto-reconnects but its attempts are killed until re-join', function () { MON.cutConnection(id); });
        mk('↻', "Bring the participant back: after a cut let the client's own reconnection complete (session restore); after a close re-create the editor (fresh handshake)", function () { MON.rejoinEditor(id); });
        // drag: like the "OO Protocol Monitor" panel — the first drag detaches
        // the bar from its pane corner (it switches to fixed positioning at
        // its current spot), afterwards it follows the mouse freely.
        var dragging = false, ox = 0, oy = 0;
        grip.addEventListener('mousedown', function (e) {
          var r = bar.getBoundingClientRect();
          bar.style.position = 'fixed';
          bar.style.left = r.left + 'px';
          bar.style.top = r.top + 'px';
          bar.style.right = 'auto';
          bar.__ooDragged = true;
          dragging = true;
          ox = e.clientX - r.left; oy = e.clientY - r.top;
          e.preventDefault();
        });
        document.addEventListener('mousemove', function (e) {
          if (!dragging) return;
          bar.style.left = (e.clientX - ox) + 'px';
          bar.style.top = (e.clientY - oy) + 'px';
        });
        document.addEventListener('mouseup', function () { dragging = false; });
        if (pane.style.position !== 'relative') pane.style.position = 'relative';
        pane.appendChild(bar);
      }
      // refresh the editor name (from its config, once built) and the state
      // dot's color/tooltip
      var cfg = MON.editorConfigs[id];
      var nm = cfg && cfg.editorConfig && cfg.editorConfig.user && cfg.editorConfig.user.name;
      if (bar.__ooNameEl) bar.__ooNameEl.textContent = nm || id;
      var stEl = bar.__ooStateEl;
      if (stEl) {
        var stt = MON.editorNetState[id] || '';
        stEl.textContent = '●';
        stEl.style.color = stt === 'online' ? '#1a7a2a' : stt === 'cut' ? '#b06a00'
          : stt === 'closed' ? '#b00' : stt === 'offline' ? '#888' : '#999';
        stEl.title = (nm || id) + ': ' + (stt || 'not connected');
      }
    }
    setInterval(function () {
      var contents = Array.from(document.querySelectorAll('div[class*="actions-container"] > div[class*="actions-content"]'));
      HARNESS_EDITORS.forEach(function (id) {
        var pane = paneForEditorId(id, contents);
        if (pane) ensureEditorBar(id, pane);
      });
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
      '#oo-mon .hdr .ver{font-weight:400;font-size:10px;background:rgba(255,255,255,.15);padding:1px 6px;border-radius:4px;white-space:nowrap}',
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
      '#oo-mon .row.held.released .head{opacity:.5}',
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
      'background:linear-gradient(135deg,transparent 50%,#3b5b8c 50%);border-top-left-radius:4px;z-index:2}'
      // (The editors' layout comes from index.html's own CSS.)
    ].join('');
    var st = document.createElement('style'); st.textContent = css;
    (document.head || document.documentElement).appendChild(st);

    // Idempotency: if a previous injection (e.g. a reload/redirect re-running
    // init scripts in the same window) already built
    // a panel, drop the old one so there's exactly one #oo-mon — otherwise the
    // later, empty panel would visually overlay the populated one.
    var oldBox = document.getElementById('oo-mon');
    if (oldBox && oldBox !== window.__OO_MON.__box) oldBox.remove();
    // also drop any other stray #oo-mon panels (there can be more than one)
    var strays = document.querySelectorAll('#oo-mon');
    for (var si = 0; si < strays.length; si++) strays[si].remove();

    var box = document.createElement('div'); box.id = 'oo-mon';
    window.__OO_MON.__box = box;
    box.innerHTML =
      '<div class="hdr" id="oo-mon-hdr"><span class="lbl">OO Protocol Monitor</span>' +
      '<span class="ver" id="oo-mon-ver" title="OnlyOffice document-server version in use (captured from the license/auth wire events)">OO ?</span>' +
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
      var turningOn = !MON.intercept;
      MON.intercept = turningOn;
      syncInterceptBtn();
      var live = window.__OO_MON || MON;
      if (turningOn) {
        live.sendCtl({ cmd: 'intercept', on: true });
      } else {
        // switching back to auto-flow: flush all held frames so they aren't lost
        live.sendCtl({ cmd: 'releaseAll' });
        live.sendCtl({ cmd: 'intercept', on: false });
      }
    });
    document.getElementById('oo-mon-copy').addEventListener('click', function () {
      copyText(exportText(), document.getElementById('oo-mon-copy'));
    });
    listEl = document.getElementById('oo-mon-list');
    statEl = document.getElementById('oo-mon-stat');
    chipsEl = document.getElementById('oo-mon-chips');
    Object.keys(MON.users).forEach(addUserChip);
    syncChips();
    syncInterceptBtn();
    updateVersionUI();
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
    if (ev.dir === 'held') return (ev.kind === 'msg' ? ev.meta.type : (ev.meta && ev.meta.eio ? ev.meta.eio : ''));
    if (ev.kind === 'msg') return ev.meta.type;
    if (ev.kind === 'eio') return ev.meta.eio + (ev.meta.sio ? '/' + ev.meta.sio : '');
    return ev.kind;
  }
  // Layer tag prefix for non-OnlyOffice (transport) events, so the exported
  // title and the panel row make the protocol layer explicit. OO application
  // events (Socket.IO `message` payloads: auth, cursor, saveChanges, ...) get
  // no prefix; everything below them in the stack does.
  //   [Transport WebSocket]  — raw socket open / proxy engine errors
  //   [Transport Engine.IO]  — Engine.IO v4 frames (open, ping/pong, ...)
  function layerTag(ev) {
    if (ev.dir === 'open' || ev.dir === 'engine') return '[Transport WebSocket] ';
    if (ev.dir === 'held') {
      if (ev.kind === 'msg') return '';                       // OO event, held
      if (ev.meta && ev.meta.eio) return '[Transport Engine.IO] ';
      return '[Transport WebSocket] ';
    }
    if (ev.kind === 'msg') return '';                        // OO event
    if (ev.kind === 'eio') return '[Transport Engine.IO] ';
    return '';
  }
  function formatLine(ev) {
    var ts = new Date(ev.t).toISOString().slice(11, 23);
    var dir = ev.dir === 'send' ? '->' : ev.dir === 'recv' ? '<-' : ev.dir === 'held' ? '##' : '  ';
    return pad(ts, 13) + '  ' + dir + '  ' + pad(userOf(ev), 14) + '  ' + layerTag(ev) + typeOf(ev);
  }
  function formatBlock(ev) {
    var body = (ev.meta === undefined || ev.meta === null) ? '' : JSON.stringify(ev.meta, null, 2);
    return '### ' + formatLine(ev) + '\n\n```json\n' + body + '\n```';
  }
  function pad(s, n) { s = String(s); return s.length >= n ? s : s + new Array(n - s.length + 1).join(' '); }

  function ooVersionString() {
    if (!MON.ooVersion) return 'unknown';
    return MON.ooBuild != null ? MON.ooVersion + ' (build ' + MON.ooBuild + ')' : MON.ooVersion;
  }
  function updateVersionUI() {
    var el = document.getElementById('oo-mon-ver');
    if (el) el.textContent = 'OO ' + ooVersionString();
  }
  // Copy export: a `## Info` header (timestamp of the copy + the OnlyOffice
  // version in use) comes before the `## Events` blocks.
  function exportText() {
    var info = '## Info\n\n- Timestamp: ' + new Date().toISOString() +
      '\n- OnlyOffice version: ' + ooVersionString();
    var blocks = MON.events.filter(visible).map(formatBlock).join('\n\n');
    return info + '\n\n## Events\n\n' + blocks;
  }

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
    if (ev.dir === 'open') { d = '🔌'; ty = layerTag(ev) + 'ws-open'; sum = String(ev.meta).slice(-50); }
    else if (ev.dir === 'engine') { d = '⚙'; ty = layerTag(ev) + ev.kind; sum = String(ev.meta).slice(-50); }
    else if (ev.dir === 'held') {
      // Held rows look just like normal send/recv rows (same icon, no HELD
      // prefix, no special colour) — only the `send` button signals the hold.
      d = ev.holdDir === 'send' ? '⬆' : '⬇';
      ty = layerTag(ev) + typeOf(ev);
      sum = ev.kind === 'msg' ? summarize(ev.meta) : (ev.meta && ev.meta.eio ? ev.meta.eio : '');
    }
    else if (ev.kind === 'msg') { d = ev.dir === 'send' ? '⬆' : '⬇'; ty = ev.meta.type; sum = summarize(ev.meta); }
    else if (ev.kind === 'eio') { d = '⚙'; ty = layerTag(ev) + ev.meta.eio + (ev.meta.sio ? '/' + ev.meta.sio : ''); }
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
          (window.__OO_MON || MON).sendCtl({ cmd: 'release', id: ev.holdId });
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

  // Only auto-scroll to the bottom if the user is already near the bottom;
  // otherwise keep their scroll position so reading older events isn't
  // constantly disrupted by new arrivals.
  function atBottom(el) {
    return el.scrollHeight - el.scrollTop - el.clientHeight < 40;
  }

  function render(ev) {
    ensureUI(); if (!listEl) return;
    if (!visible(ev)) return;
    var stick = atBottom(listEl);
    listEl.appendChild(makeRow(ev));
    while (listEl.childNodes.length > 1000) listEl.removeChild(listEl.firstChild);
    if (stick) listEl.scrollTop = listEl.scrollHeight;
    updateStat();
  }
  function rebuild() {
    ensureUI(); if (!listEl) return;
    var stick = atBottom(listEl);
    listEl.innerHTML = '';
    MON.events.forEach(function (ev) { if (visible(ev)) listEl.appendChild(makeRow(ev)); });
    if (stick) listEl.scrollTop = listEl.scrollHeight;
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
