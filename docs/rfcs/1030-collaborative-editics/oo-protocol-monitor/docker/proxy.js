/*
 * proxy.js — WebSocket man-in-the-middle for OnlyOffice co-editing.
 *
 * The browser is told (via an init-script URL redirect) to open its OO
 * co-editing WebSockets against a *local* `ws://127.0.0.1:PORT/oo` server
 * instead of the real `wss://...onlyoffice...` endpoint. This server accepts
 * that connection, parses the encoded real URL, opens a real upstream
 * `WebSocket` to OnlyOffice, and pipes both directions through a gate.
 *
 * Because we own both sockets (browser<->proxy is plain ws, proxy<->OO is
 * real wss), we see every frame in cleartext and can HOLD any non-noise frame
 * until the user releases it. This is the only approach that can gate RECV:
 * the bytes literally cannot reach the browser's socket.io until we forward.
 *
 * A second WS endpoint `/ctl` is the control channel the in-page panel uses to
 *   - receive decoded events + held-frame notifications (push from Node)
 *   - send commands: {cmd:'release', id}, {cmd:'intercept', on:bool} (aka
 *     "manual flow": when on, every non-noise frame is held), {cmd:'getState'},
 *     {cmd:'cut', user|editor, keep:'dead'|'once'} and {cmd:'revive', user|editor}
 *     (disconnection support, see makeSession).
 *
 * Every socket death is emitted as a `[Transport WebSocket] ws-close` event
 * carrying who ended the connection (client / server / monitor) and the
 * close code, so a capture shows its disconnect boundaries per participant.
 *
 * Decoding mirrors monitor.js's Engine.IO v4 + Socket.IO v4 decoder, but in
 * Node. Identity (editor username) is derived the same way: from the `auth`
 * event / SIO CONNECT auth payload.
 *
 * Non-noise (ping/pong/noop) frames are NEVER gated — gating them makes
 * engine.io time the connection out and the server closes the socket.
 */
'use strict';
const http = require('http');
const { WebSocketServer, WebSocket } = require('ws');

// ---- Engine.IO v4 + Socket.IO v4 frame decoder (Node port of monitor.js) --
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
  // misleading double-`type` in the exported logs. Consumers read the result
  // as the real event (c.type, c.user, ...).
  return p;
}
function authFromConnect(d) {
  if (!d || d.sio !== 'connect' || !d.payload) return null;
  var data = d.payload.data || d.payload;
  if (data && data.type === 'auth' && data.user && data.user.username) return data.user.username;
  return null;
}
function authFromMessage(c) {
  if (c && c.type === 'auth' && c.user && c.user.username)
    return c.user.username;
  return null;
}
function isNoise(d) {
  return d && d.eio && (d.eio === 'ping' || d.eio === 'pong' || d.eio === 'noop');
}

// ---- session bookkeeping --------------------------------------------------
// Each OO WebSocket (browser side) is a "Session". It owns:
//   - browserWs : the ws connection from the browser
//   - upstreamWs: the real ws connection to OnlyOffice
//   - holds     : Map holdId -> {dir, raw, meta, gen}
//   - gen       : generation (per browser-side socket)
//   - editor    : editor id (from the URL query the browser sent us)
//   - user      : resolved username (once auth seen)
// All decoded events are broadcast to every connected /ctl panel.

function createProxy(opts) {
  opts = opts || {};
  const PORT = opts.port || 0;       // 0 = pick a free port
  const state = {
    intercept: false,      // aka "manual flow": hold every non-noise frame
    sessions: new Map(),   // browserWs -> session
    holds: new Map(),      // holdId -> {sid, dir, raw, meta, gen}
    holdSeq: 0,
    sidSeq: 0,
    panels: new Set(),     // connected /ctl ws
    events: [],            // ring buffer of recent events (for late panels)
    deadEditors: new Set(), // editor ids whose sockets are killed on sight
                            // ("connection lost" stays lost until `revive`)
  };

  function broadcast(obj) {
    const msg = JSON.stringify(obj);
    for (const p of state.panels) {
      if (p.readyState === WebSocket.OPEN) {
        try { p.send(msg); } catch (e) {}
      }
    }
  }
  // keep a small ring buffer so a panel connecting late can catch up
  function pushEvent(ev) {
    state.events.push(ev);
    if (state.events.length > 2000) state.events.shift();
    broadcast({ t: 'event', ev: ev });
  }
  function log(...a) { if (opts.verbose) console.log('[proxy]', ...a); }
  // always-on critical log (connections / upstream errors), independent of --verbose
  function info(...a) { console.log('[proxy]', ...a); }
  function decodeURIComponentSafe(s) { try { return decodeURIComponent(s); } catch (e) { return String(s); } }

  function summarize(p) {
    try {
      switch (p && p.type) {
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

  // ---- one OO session ----------------------------------------------------
  function makeSession(browserWs, realUrl, editor) {
    // A cut editor stays cut: kill its reconnection attempts on sight, and
    // silently — the client retries every ~2s per the `settings.reconnection`
    // of its auth event, and those bare attempts carry nothing worth recording
    // (emitting them would also never let a capture quiesce). The cut itself
    // is recorded by session.cut(); `revive` clears the mark.
    if (editor && state.deadEditors.has(editor)) {
      log('killing reconnection attempt of dead editor=' + editor);
      try { browserWs.terminate(); } catch (e) {}
      return;
    }
    const sid = ++state.sidSeq;
    GENS[editor || '?'] = (GENS[editor || '?'] || 0) + 1;
    const gen = GENS[editor || '?'];
    const session = {
      sid: sid, gen: gen, editor: editor || '?',
      user: null,
      browserWs: browserWs,
      upstreamWs: null,
      holds: new Map(),
      closed: false,
      closeInfo: null,     // who ended the connection once known
                           // ({by:'client'|'server'|'monitor', code, reason})
    };
    state.sessions.set(browserWs, session);

    function emit(dir, kind, meta, extra) {
      const ev = Object.assign({
        t: Date.now(), sid: sid, gen: gen, editor: session.editor,
        dir: dir, kind: kind, meta: meta,
        user: session.user || null,
      }, extra || {});
      pushEvent(ev);
    }
    function setUser(name) {
      if (name && session.user !== name) {
        session.user = name;
        // Backfill the user onto all this session's already-emitted events so
        // rows rendered before the auth frame (e.g. ws-open / engine open) get
        // the right author. Identity is keyed by sid (per-connection) because
        // the editor id from the iframe URL is often empty ('?'), which would
        // otherwise make every session share one editor->user slot.
        for (const ev of state.events) if (ev.sid === sid) ev.user = name;
        broadcast({ t: 'user', sid: sid, editor: session.editor, user: name });
      }
    }

    emit('open', 'ws', realUrl);
    // open the upstream. realUrl is the original wss://... url the browser
    // intended. Node's ws handles TLS for us. NOTE: ws's WebSocket signature is
    // (url, protocols, options) — pass subprotocol as 2nd arg, options as 3rd.
    let up;
    try {
      const proto = browserWs.__ooProtocol || undefined;
      const opts = {
        headers: { Origin: new URL(realUrl).origin },
        perMessageDeflate: false,
      };
      up = proto ? new WebSocket(realUrl, proto, opts) : new WebSocket(realUrl, undefined, opts);
    } catch (e) {
      emit('engine', 'error', String(e && e.message || e).slice(0, 120));
      try { browserWs.close(); } catch (c) {}
      return;
    }
    session.upstreamWs = up;

    up.on('open', () => { info('upstream connected for editor=' + session.editor + ' sid=' + sid + ' -> ' + realUrl); });
    up.on('unexpected-response', (req, res) => {
      info('upstream rejected (HTTP ' + res.statusCode + ') for ' + realUrl);
      emit('engine', 'error', 'upstream HTTP ' + res.statusCode);
    });
    up.on('error', (err) => {
      if (session.closed) return;
      emit('engine', 'error', String(err && err.message || err).slice(0, 120));
    });
    up.on('close', (code, reason) => {
      if (session.closed) return;
      // The document server ended the connection: remember who hung up (the
      // ws-close row is emitted once the browser leg follows) and forward its
      // close code/reason so the client sees what the server sent. An abnormal
      // server-side death cannot be relayed as a close frame (codes like 1006
      // are reserved) — terminate the browser leg instead, so the client sees
      // a lost connection too.
      if (!session.closeInfo) session.closeInfo = { by: 'server', code: code, reason: (reason || '').toString() };
      if (code == null || code === 1005 || code === 1006 || code === 1015 || code < 1000 || code > 4999) {
        try { browserWs.terminate(); } catch (e) {}
      } else {
        try { browserWs.close(code, reason); } catch (e) { try { browserWs.close(); } catch (e2) {} }
      }
    });

    // ---- frames from upstream (server -> browser) = RECV ----------------
    up.on('message', (data, isBinary) => {
      // `data` is Buffer; for text frames decode as utf8.
      const str = isBinary ? null : data.toString('utf8');
      const dec = str != null ? decodeEIO(str) : null;
      if (isNoise(dec)) { forwardToBrowser(str != null ? str : data, isBinary); return; }
      let c = null;
      if (str != null) c = classify(dec);
      if (state.intercept && !isNoise(dec)) {
        // manual-flow: hold the frame; emit ONLY the held event (no duplicate
        // normal recv) so the panel shows exactly one actionable row.
        hold('recv', str != null ? str : data, isBinary, c, dec);
      } else {
        if (c) emit('recv', 'msg', c);
        else if (dec && dec.eio) emit('recv', 'eio', dec);
        else if (isBinary) emit('recv', 'binary', { len: data.length });
        forwardToBrowser(str != null ? str : data, isBinary);
      }
    });

    function forwardToBrowser(payload, isBinary) {
      if (session.closed || browserWs.readyState !== WebSocket.OPEN) return;
      try { browserWs.send(payload, { binary: !!isBinary }); } catch (e) {}
    }

    // ---- frames from browser (client -> server) = SEND -----------------
    browserWs.on('message', (data, isBinary) => {
      const str = isBinary ? null : data.toString('utf8');
      const dec = str != null ? decodeEIO(str) : null;
      if (isNoise(dec)) { forwardToUpstream(str != null ? str : data, isBinary); return; }
      // learn identity early from SIO CONNECT auth + from auth message
      const name = authFromConnect(dec) || authFromMessage(classify(dec));
      if (name) setUser(name);
      let c = null;
      if (str != null) c = classify(dec);
      if (state.intercept && !isNoise(dec)) {
        hold('send', str != null ? str : data, isBinary, c, dec);
      } else {
        if (c) emit('send', 'msg', c);
        else if (dec && dec.eio) emit('send', 'eio', dec);
        else if (isBinary) emit('send', 'binary', { len: data.length });
        forwardToUpstream(str != null ? str : data, isBinary);
      }
    });

    function forwardToUpstream(payload, isBinary) {
      if (session.closed || !up || up.readyState !== WebSocket.OPEN) return;
      try { up.send(payload, { binary: !!isBinary }); } catch (e) {}
    }

    // ---- hold a frame ---------------------------------------------------
    function hold(dir, raw, isBinary, c, dec) {
      const hid = 'h' + (++state.holdSeq);
      let meta;
      if (c) meta = c;
      else if (dec && dec.eio) meta = dec;
      else meta = { raw: (typeof raw === 'string' ? raw : '<binary ' + raw.length + 'B>').slice(0, 80) };
      const h = { id: hid, sid: sid, dir: dir, raw: raw, isBinary: !!isBinary, meta: meta, gen: gen, session: session };
      session.holds.set(hid, h);
      state.holds.set(hid, h);
      emit('held', (c ? 'msg' : (dec && dec.eio ? 'eio' : 'raw')), meta, { holdId: hid, holdDir: dir });
    }

    // ---- release a held frame (called from /ctl) ------------------------
    session.release = function (hid) {
      const h = session.holds.get(hid);
      if (!h) return false;          // not ours / already gone
      session.holds.delete(hid);
      state.holds.delete(hid);
      if (session.closed) return false;
      if (h.dir === 'recv') {
        if (browserWs.readyState !== WebSocket.OPEN) return false;
        try { browserWs.send(h.raw, { binary: h.isBinary }); return true; }
        catch (e) { return false; }
      } else {
        if (!up || up.readyState !== WebSocket.OPEN) return false;
        try { up.send(h.raw, { binary: h.isBinary }); return true; }
        catch (e) { return false; }
      }
    };
    session.releaseAll = function (dir) {
      for (const hid of Array.from(session.holds.keys())) {
        if (dir && this.holds.get(hid) && this.holds.get(hid).dir !== dir) continue;
        this.release(hid);
      }
    };
    session.close = function () {
      if (session.closed) return;
      session.closed = true;
      const info = session.closeInfo || { by: 'client', code: 1006, reason: '' };
      emit('engine', 'ws-close', wsCloseText(info));
      // any holds for this session are now stale
      for (const hid of Array.from(session.holds.keys())) {
        state.holds.delete(hid);
        broadcast({ t: 'stale', id: hid });
      }
      session.holds.clear();
      // Mirror the loss to the document server: an abnormal client-side death
      // must not become a clean close frame towards OnlyOffice — the server
      // has to see the same connection-lost it would see in real life.
      const abnormal = info.code === 1006 || info.by === 'monitor';
      if (up) { try { abnormal ? up.terminate() : up.close(); } catch (e) {} }
      try { browserWs.close(); } catch (e) {}
      state.sessions.delete(browserWs);
    };
    // The "connection lost" path: kill both legs with no close frame.
    session.cut = function () {
      if (session.closed) return;
      session.closeInfo = { by: 'monitor', code: 1006, reason: 'connection cut' };
      session.close();
    };

    browserWs.on('close', (code, reason) => {
      if (!session.closeInfo) session.closeInfo = { by: 'client', code: code, reason: (reason || '').toString() };
      session.close();
    });
    browserWs.on('error', () => {
      if (!session.closeInfo) session.closeInfo = { by: 'client', code: 1006, reason: 'error' };
      session.close();
    });
  }

  // Human-readable `[Transport WebSocket] ws-close` row meta (the record
  // parser skips transport rows; this documents who hung up and how).
  function wsCloseText(info) {
    const by = { client: 'client', server: 'server', monitor: 'monitor' }[info.by] || String(info.by);
    const cause = info.by === 'monitor' ? ' (connection cut)' :
      (info.reason ? ' (' + String(info.reason).slice(0, 40) + ')' : '');
    return by + ' close, code ' + (info.code == null ? '?' : info.code) + cause;
  }

  // Sessions matching a user name (Alice…) and/or an editor id
  // (document-editor-1…).
  function findSessions(user, editor) {
    return Array.from(state.sessions.values()).filter((s) =>
      (!user || s.user === user) && (!editor || s.editor === editor));
  }

  // per-editor generation count (mirrors monitor.js GEN)
  const GENS = {};

  // ---- HTTP + WS server ---------------------------------------------------
  const server = http.createServer((req, res) => {
    if (req.url === '/health') { res.writeHead(200, { 'content-type': 'text/plain' }); res.end('ok'); return; }
    res.writeHead(404); res.end('not found');
  });
  const wss = new WebSocketServer({ server: server });

  wss.on('connection', (ws, req) => {
    const path = (req.url || '').split('?')[0];
    if (path === '/ctl') {
      handleControl(ws, req);
    } else if (path === '/oo') {
      handleOo(ws, req);
    } else {
      try { ws.close(1000, 'unknown path'); } catch (e) {}
    }
  });

  function handleOo(ws, req) {
    // The browser was redirected to ws://127.0.0.1:PORT/oo?<encoded real url>
    // (and optionally &__proto=...&__editor=...). Recover the real url.
    const u = new URL(req.url, 'http://127.0.0.1');
    const realEnc = u.searchParams.get('u');
    log('/oo connection from editor=' + (u.searchParams.get('e') || '?') + ' real=' + decodeURIComponentSafe(realEnc));
    info('/oo connection: editor=' + (u.searchParams.get('e') || '?') + ' -> ' + decodeURIComponentSafe(realEnc));
    if (!realEnc) { try { ws.close(1000, 'missing u'); } catch (e) {} return; }
    let realUrl;
    try { realUrl = decodeURIComponent(realEnc); } catch (e) { try { ws.close(1000, 'bad u'); } catch (x) {} return; }
    const editor = u.searchParams.get('e') || null;
    ws.__ooProtocol = u.searchParams.get('p') || undefined; // subprotocol pass-through
    makeSession(ws, realUrl, editor);
  }

  function handleControl(ws, req) {
    state.panels.add(ws);
    // send current state + recent history on connect
    ws.send(JSON.stringify({ t: 'hello', intercept: state.intercept, events: state.events, users: getUsers() }));
    ws.on('message', (raw) => {
      let msg; try { msg = JSON.parse(raw.toString('utf8')); } catch (e) { return; }
      if (!msg || !msg.cmd) return;
      switch (msg.cmd) {
        case 'intercept': {
          state.intercept = !!msg.on;
          broadcast({ t: 'intercept', on: state.intercept });
          log('manualFlow=' + state.intercept);
          break;
        }
        case 'release': {
          const h = state.holds.get(msg.id);
          let ok = false;
          if (h && h.session && h.session.release) ok = h.session.release(msg.id);
          broadcast({ t: 'released', id: msg.id, ok: !!ok });
          break;
        }
        case 'releaseAll': {
          // Release every held frame across all sessions (used when switching
          // back to auto-flow so paused frames are flushed, not lost).
          for (const sess of state.sessions.values()) sess.releaseAll();
          broadcast({ t: 'releasedAll' });
          break;
        }
        case 'cut': {
          // Abruptly kill the connection(s) matching `user` (Alice…) and/or
          // `editor` (document-editor-1…): no close frame on either leg (the
          // "connection lost" path). `keep: 'once'` lets the client's stock
          // auto-reconnect (~2s, per the settings.reconnection of its auth
          // event) go through — the session-restore scenario; any other `keep`
          // (the panel button default) also kills every reconnection attempt
          // of the same editor until `revive`, so the participant stays away.
          const targets = findSessions(msg.user, msg.editor);
          const keepDead = msg.keep !== 'once';
          for (const s of targets) {
            if (keepDead && s.editor) state.deadEditors.add(s.editor);
            s.cut();
          }
          if (keepDead && !targets.length && msg.editor) state.deadEditors.add(msg.editor);
          info('cut ' + (targets.length || 'no') + ' session(s)' +
            (msg.user ? ' of ' + msg.user : '') + (msg.editor ? ' (' + msg.editor + ')' : '') +
            (keepDead ? ', reconnection attempts killed until revive' : ' (once)'));
          broadcast({ t: 'cut', user: msg.user || null, editor: msg.editor || null, count: targets.length });
          break;
        }
        case 'revive': {
          // Let a cut editor connect again (its client retries by itself).
          if (msg.editor) state.deadEditors.delete(msg.editor);
          if (msg.user) for (const s of state.sessions.values())
            if (s.user === msg.user && s.editor) state.deadEditors.delete(s.editor);
          broadcast({ t: 'revive', user: msg.user || null, editor: msg.editor || null });
          break;
        }
        case 'getState': {
          ws.send(JSON.stringify({ t: 'state', intercept: state.intercept, users: getUsers(), holds: state.holds.size, sessions: state.sessions.size, dead: Array.from(state.deadEditors) }));
          break;
        }
      }
    });
    ws.on('close', () => { state.panels.delete(ws); });
    ws.on('error', () => { state.panels.delete(ws); });
  }
  function getUsers() {
    const m = {};
    for (const s of state.sessions.values()) if (s.user) m[s.sid] = s.user;
    return m;
  }

  return new Promise((resolve, reject) => {
    server.on('error', reject);
    server.listen(PORT, '127.0.0.1', () => {
      const addr = server.address();
      const actualPort = addr.port;
      resolve({
        port: actualPort,
        server,
        state,
        close: () => { for (const s of state.sessions.values()) s.close(); server.close(); wss.close(); },
      });
    });
  });
}

module.exports = { createProxy };
