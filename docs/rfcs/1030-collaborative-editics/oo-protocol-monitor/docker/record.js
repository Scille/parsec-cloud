#!/usr/bin/env node
/**
 * record.js — automated scenario recorder for the OnlyOffice co-editing
 * protocol monitor.
 *
 * Where `run.js` opens the harness page for a *human* to craft scenarios with
 * the monitor panel, this script replays a fixed list of scenarios (auth,
 * cursor moves, chat messages, document edits — serial / concurrent /
 * while-another-client-is-authenticating —, disconnection/reconnection —
 * close/lose connection, restore/fresh re-join, locks and saves held by a
 * disconnecting participant, across the docx / xlsx / pptx sample documents)
 * with Playwright driving the editors, and writes one record file per
 * scenario to `server/tests/editics/records/new/`.
 *
 * The records are exported in the exact format of the monitor panel's
 * `copy` button (`## Info` + `## Events` + `### <ts> <dir> <user> <type>`
 * blocks), so `server/tests/editics/test_records.py` can parse them as-is:
 * the docker harness page already opens the editors as Alice / Bob / Mallory
 * with the Parsec test-fixture device IDs.
 *
 * Concurrency is crafted with the proxy's manual-flow (same as the panel):
 * OnlyOffice application frames are held at the proxy and released one by
 * one in a scenario-specific order. The recorder reconstructs the *delivered*
 * frame order (an auto-flow frame is recorded when it flows; a held frame is
 * recorded when its release is acknowledged — stale/undelivered frames never
 * appear), so a record always shows what the clients actually processed, in
 * delivery order.
 *
 * Frames that are not OnlyOffice application messages (Engine.IO open,
 * raw Socket.IO connect/ack, binary) always flow through the gate — only
 * `message`-level frames participate in the choreography (keepalive
 * ping/pong/noop are never gated at all, they would time the sockets out).
 * Raw Socket.IO frames are invisible in auto-flow captures, so they are
 * left out of the exported records too (the `[Transport …]` rows the parser
 * skips stay: ws-open, Engine.IO open).
 *
 * Usage:
 *   node record.js                      # run every scenario
 *   node record.js 0_auth_alone 21_message_serial
 *                                       # run only the named scenarios
 *
 * Environment (same as run.js: OO_ENGINE, OO_DS_IMAGE, OO_DS_PORT,
 * OO_HOST_ALIAS, OO_DS_BOOT_TIMEOUT, HEADLESS) — plus:
 *   OO_RECORDS_DIR  output directory (default: server/tests/editics/records/new)
 *   OO_EXISTING_DS_PORT
 *                   reuse an already-running document server on this port
 *                   (e.g. the one started by `npm run with-docker`) instead
 *                   of starting a new container — needed when this script
 *                   runs in an environment that cannot boot the document
 *                   server container itself (e.g. the sandbox: no systemd
 *                   user session, the DS entrypoint hangs on postgres).
 *   OO_STAY_ALIVE   keep the browser + container open after the run (debug)
 *   OO_PROBE=word|cell|slide
 *                   run the UI probe for a document type instead of the
 *                   scenarios: clicks/types/chats around the editor and
 *                   screenshots everything into /tmp/oo-record-probe/ —
 *                   used to calibrate the per-type click/type targets.
 *   OO_SCAN=word|cell|slide
 *                   grid-click Alice's editor (with Bob connected: cursor
 *                   events are only sent when someone else is there) and
 *                   print which points emit `cursor` messages with which
 *                   values — used to calibrate CURSOR_TARGETS.
 */
'use strict';

const fs = require('fs');
const http = require('http');
const net = require('net');
const path = require('path');
const { spawnSync } = require('child_process');

const { chromium } = require('playwright');
const WebSocket = require('ws');
const { createProxy } = require('./proxy');

const DIR = __dirname;
const MONITOR = path.join(DIR, 'monitor.js');
const PAGE_TEMPLATE = path.join(DIR, 'index.html');
const SAMPLE_DOC_DIR = path.join(DIR, '..', '..', '..', '..', '..', 'client', 'tests', 'e2e', 'data', 'imports');
const OUT_DIR = process.env.OO_RECORDS_DIR ||
  path.join(DIR, '..', '..', '..', '..', '..', 'server', 'tests', 'editics', 'records', 'new');
const PROBE_DIR = '/tmp/oo-record-probe';

const DOC_TYPES = {
  word:  { file: 'document.docx',     documentType: 'word',  fileType: 'docx', label: 'Word (.docx)' },
  cell:  { file: 'spreadsheet.xlsx',  documentType: 'cell',  fileType: 'xlsx', label: 'Spreadsheet (.xlsx)' },
  slide: { file: 'presentation.pptx', documentType: 'slide', fileType: 'pptx', label: 'Presentation (.pptx)' },
};
const MIME = {
  docx: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
  xlsx: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
  pptx: 'application/vnd.openxmlformats-officedocument.presentationml.presentation',
};
const DOC_EXT = { word: 'docx', cell: 'xlsx', slide: 'pptx' };

const IMAGE = process.env.OO_DS_IMAGE || 'docker.io/onlyoffice/documentserver:9.3.0';
const CONTAINER = 'oo-protocol-monitor-ds';
const BOOT_TIMEOUT_MS = parseInt(process.env.OO_DS_BOOT_TIMEOUT || '300000', 10);
const HEADLESS = process.env.HEADLESS !== '0';
const STAY_ALIVE = !!process.env.OO_STAY_ALIVE;
const PROBE_DOC = process.env.OO_PROBE || null;

// Timeouts (generous: the document server converts documents on first open,
// three editors may boot at once…)
const EDITOR_LOAD_TIMEOUT = 90000;
const EVENT_TIMEOUT = 45000;
const QUIESCENCE_MS = 2000;
const QUIESCENCE_CAP = 60000;

// ---------------------------------------------------------------------------
// Container engine + lifecycle (mirrors run.js)
// ---------------------------------------------------------------------------

function detectEngine() {
  if (process.env.OO_ENGINE) return process.env.OO_ENGINE;
  for (const e of ['podman', 'docker']) {
    const r = spawnSync(e, ['--version'], { stdio: 'ignore' });
    if (!r.error && r.status === 0) return e;
  }
  console.error('✖ neither `podman` nor `docker` found on PATH — set OO_ENGINE=<bin>');
  process.exit(1);
}
const ENGINE = detectEngine();
const HOST_ALIAS = process.env.OO_HOST_ALIAS ||
  (ENGINE === 'podman' ? 'host.containers.internal' : 'host.docker.internal');

function engine(args, opts) {
  return spawnSync(ENGINE, args, opts || {});
}
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

function httpGet(url, timeoutMs) {
  return new Promise((resolve, reject) => {
    const req = http.get(url, { timeout: timeoutMs }, (res) => {
      let body = '';
      res.setEncoding('utf8');
      res.on('data', (d) => { body += d; });
      res.on('end', () => resolve({ status: res.statusCode, body }));
    });
    req.on('timeout', () => req.destroy(new Error('timeout')));
    req.on('error', reject);
  });
}

function isFree(port) {
  return new Promise((resolve) => {
    const srv = net.createServer();
    srv.once('error', () => resolve(false));
    srv.listen(port, '127.0.0.1', () => srv.close(() => resolve(true)));
  });
}
function freePort() {
  return new Promise((resolve, reject) => {
    const srv = net.createServer();
    srv.once('error', reject);
    srv.listen(0, '127.0.0.1', () => srv.close(() => resolve(srv.address().port)));
  });
}
async function pickDsPort() {
  const want = parseInt(process.env.OO_DS_PORT || '0', 10);
  if (want) return want;
  for (const p of [8080, 8888]) if (await isFree(p)) return p;
  return freePort();
}

let containerStarted = false;
let cleaned = false;
function cleanup() {
  if (cleaned) return;
  cleaned = true;
  if (containerStarted) {
    containerStarted = false;
    console.log('→ stopping the document server container…');
    engine(['stop', '-t', '10', CONTAINER], { stdio: 'ignore' });
  }
}
process.on('SIGINT', () => { cleanup(); process.exit(130); });
process.on('SIGTERM', () => { cleanup(); process.exit(143); });

function containerArgs(dsPort) {
  const args = [
    'run', '-d', '--rm',
    '--name', CONTAINER,
    '-p', '127.0.0.1:' + dsPort + ':80',
    '-e', 'JWT_ENABLED=false',
    '-e', 'ALLOW_META_IP_ADDRESS=true',
    '-e', 'ALLOW_PRIVATE_IP_ADDRESS=true',
  ];
  if (ENGINE === 'docker') args.push('--add-host=host.docker.internal:host-gateway');
  args.push(IMAGE);
  return args;
}

async function waitDsReady(dsPort) {
  const t0 = Date.now();
  console.log('→ waiting for the document server healthcheck…');
  for (;;) {
    const elapsed = Date.now() - t0;
    if (elapsed > BOOT_TIMEOUT_MS) {
      engine(['logs', '--tail', '40', CONTAINER], { stdio: 'inherit' });
      throw new Error('document server healthcheck timed out after ' + Math.round(elapsed / 1000) + 's');
    }
    const st = engine(['inspect', '-f', '{{.State.Running}}', CONTAINER], { encoding: 'utf8' });
    if (st.status !== 0 || String(st.stdout || '').trim() !== 'true') {
      throw new Error('the document server container exited during startup');
    }
    try {
      const r = await httpGet('http://127.0.0.1:' + dsPort + '/healthcheck', 5000);
      if (r.status === 200 && /true/.test(r.body)) {
        console.log('  document server is up after ' + Math.round(elapsed / 1000) + 's');
        return;
      }
    } catch (e) { /* not up yet */ }
    await sleep(1500);
  }
}

// ---------------------------------------------------------------------------
// Harness page server (same template as run.js, but the doc key comes from
// the `key` URL param so every record gets a fresh co-editing session)
// ---------------------------------------------------------------------------

function startPageServer(dsPort, docs) {
  const server = http.createServer();
  return new Promise((resolve, reject) => {
    server.on('error', reject);
    server.listen(0, '0.0.0.0', () => {
      const port = server.address().port;
      const template = fs.readFileSync(PAGE_TEMPLATE, 'utf8');
      const contents = {};
      for (const type in docs) contents[docs[type].file] = fs.readFileSync(docs[type].path);
      function docButtons(type) {
        return Object.keys(docs).map((t) =>
          '<button class="doc-button' + (t === type ? ' active' : '') + '" data-doc="' + t +
          '" title="Open ' + docs[t].file + '">' + docs[t].label + '</button>').join('');
      }
      function renderPage(type, key) {
        const doc = docs[type];
        return template
          .replace(/\{\{DS_PORT\}\}/g, String(dsPort))
          .replace(/\{\{PAGE_PORT\}\}/g, String(port))
          .replace(/\{\{DOC_KEY\}\}/g, key)
          .replace(/\{\{HOST_ALIAS\}\}/g, HOST_ALIAS)
          .replace(/\{\{IMAGE\}\}/g, IMAGE)
          .replace(/\{\{DOC_BUTTONS\}\}/g, docButtons(type))
          .replace(/\{\{DOC_TYPE\}\}/g, doc.documentType)
          .replace(/\{\{FILE_TYPE\}\}/g, doc.fileType)
          .replace(/\{\{DOC_TITLE\}\}/g, doc.file)
          .replace(/\{\{DOC_URL_FILE\}\}/g, doc.file);
      }
      server.on('request', (req, res) => {
        const u = new URL(req.url || '/', 'http://127.0.0.1');
        const p = u.pathname;
        if (req.method === 'GET' && p === '/') {
          let type = u.searchParams.get('doc') || 'word';
          if (!docs[type]) type = 'word';
          const key = u.searchParams.get('key') || 'oom' + Date.now().toString(36);
          res.writeHead(200, { 'content-type': 'text/html; charset=utf-8' });
          res.end(renderPage(type, key));
        } else if (req.method === 'GET' && contents[p.slice(1)]) {
          const doc = Object.values(docs).find((d) => d.file === p.slice(1));
          const body = contents[p.slice(1)];
          res.writeHead(200, { 'content-type': MIME[doc.fileType], 'content-length': body.length });
          res.end(body);
        } else if (req.method === 'POST' && p === '/callback') {
          req.on('data', () => {});
          req.on('end', () => { res.writeHead(200, { 'content-type': 'application/json' }); res.end('{"error":0}'); });
        } else {
          res.writeHead(404);
          res.end('not found');
        }
      });
      resolve({ server, port });
    });
  });
}

// ---------------------------------------------------------------------------
// Recorder — a /ctl client that reconstructs the delivered-frame order.
//
//   - auto-flow frames (dir send/recv) are recorded when they flow
//   - manual-flow frames arrive as `held` events; they are recorded at the
//     moment their `released` ack arrives (stale frames are never recorded)
//   - the proxy backfills the user name of a session onto its earlier events
//     (`{t:'user'}` messages); the same backfill is replayed here
//
// The export replicates the monitor panel's `copy` format.
// ---------------------------------------------------------------------------

class Recorder {
  constructor(port) {
    this.port = port;
    this.events = [];          // delivered events, in order
    this.holds = new Map();    // holdId -> ev (pending held frames)
    this.sidUser = new Map();  // sid -> username
    this.ooVersion = null;
    this.ooBuild = null;
    this.pendingReleased = []; // released acks awaiting pickup
    this.ws = null;
    this.intercept = false;
    this.closed = false;
  }

  connect() {
    return new Promise((resolve, reject) => {
      const ws = new WebSocket('ws://127.0.0.1:' + this.port + '/ctl');
      ws.on('open', () => { this.ws = ws; resolve(); });
      ws.on('error', reject);
      ws.on('message', (raw) => {
        let msg; try { msg = JSON.parse(raw.toString('utf8')); } catch (e) { return; }
        this.handle(msg);
      });
      ws.on('close', () => {
        if (!this.closed) setTimeout(() => this.connect().catch(() => {}), 500);
      });
    });
  }

  send(obj) {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) this.ws.send(JSON.stringify(obj));
  }

  handle(msg) {
    switch (msg.t) {
      case 'hello': {
        // (Re)sync from the proxy's ring buffer — happens on (re)connect.
        this.events.length = 0;
        this.holds.clear();
        this.sidUser.clear();
        for (const ev of msg.events || []) this.onEvent(ev);
        if (msg.users) for (const sid in msg.users) this.setUser(parseInt(sid, 10), msg.users[sid]);
        break;
      }
      case 'event': this.onEvent(msg.ev); break;
      case 'user': this.setUser(msg.sid, msg.user); break;
      case 'released': {
        const ev = this.holds.get(msg.id);
        this.holds.delete(msg.id);
        if (ev && msg.ok) {
          // The frame has actually been delivered to its recipient: record it
          // as a normal flow event, timestamped at release (delivery) time.
          this.push({
            t: Date.now(), sid: ev.sid, gen: ev.gen, editor: ev.editor,
            user: ev.user, dir: ev.holdDir, kind: ev.kind, meta: ev.meta,
          });
        }
        this.pendingReleased.push({ id: msg.id, ok: !!msg.ok });
        break;
      }
      case 'stale': this.holds.delete(msg.id); break;
      case 'releasedAll': {
        // Bulk flush (the scenarios release frame by frame, this is a safety
        // net): deliver pending holds in hold order.
        for (const [, ev] of this.holds) {
          this.push({ t: Date.now(), sid: ev.sid, gen: ev.gen, editor: ev.editor,
            user: ev.user, dir: ev.holdDir, kind: ev.kind, meta: ev.meta });
        }
        this.holds.clear();
        break;
      }
    }
  }

  onEvent(ev) {
    if (ev.user) this.sidUser.set(ev.sid, ev.user);
    if (ev.meta && typeof ev.meta === 'object') {
      const p = ev.meta;
      if ((p.type === 'license' && p.license && !this.ooVersion) ||
          (p.type === 'auth' && p.result && !this.ooVersion)) {
        const lic = p.license || p;
        this.ooVersion = String(lic.buildVersion);
        this.ooBuild = lic.buildNumber != null ? lic.buildNumber : null;
      }
    }
    if (ev.dir === 'held') {
      this.holds.set(ev.holdId, ev);
      return;
    }
    this.push(ev);
  }

  setUser(sid, user) {
    this.sidUser.set(sid, user);
    for (const ev of this.events) if (ev.sid === sid && !ev.user) ev.user = user;
    for (const [, ev] of this.holds) if (ev.sid === sid && !ev.user) ev.user = user;
  }

  push(ev) {
    if (ev.user) this.sidUser.set(ev.sid, ev.user);
    this.events.push(ev);
  }

  userOf(ev) {
    return ev.user || this.sidUser.get(ev.sid) || ev.editor || '?';
  }

  held() { return Array.from(this.holds.values()); }

  // ---- waiting helpers -----------------------------------------------------

  hasEvent(pred) { return this.events.some(pred); }

  // Wait for a pending held frame matching pred.
  async waitHold(pred, timeout = EVENT_TIMEOUT, desc = 'hold') {
    const t0 = Date.now();
    for (;;) {
      for (const ev of this.holds.values()) if (pred(ev)) return ev;
      if (Date.now() - t0 > timeout) throw new Error('timeout waiting for ' + desc);
      await sleep(100);
    }
  }

  // Wait for a delivered event matching pred (scanning from `fromIndex`).
  // Resolves with { ev, index } so callers can chain sequential waits.
  async waitEvent(pred, timeout = EVENT_TIMEOUT, desc = 'event', fromIndex = 0) {
    const t0 = Date.now();
    for (;;) {
      for (let i = fromIndex; i < this.events.length; i++) {
        if (pred(this.events[i])) return { ev: this.events[i], index: i };
      }
      if (Date.now() - t0 > timeout) throw new Error('timeout waiting for ' + desc);
      await sleep(100);
    }
  }

  // Wait until no new delivered event for `ms` (bounded by `cap`).
  async waitQuiescence(ms = QUIESCENCE_MS, cap = QUIESCENCE_CAP) {
    const t0 = Date.now();
    let last = -1, lastChange = Date.now();
    for (;;) {
      if (this.events.length !== last) { last = this.events.length; lastChange = Date.now(); }
      if (Date.now() - lastChange > ms) return;
      if (Date.now() - t0 > cap) return;
      await sleep(150);
    }
  }

  // ---- gate helpers --------------------------------------------------------

  setIntercept(on) {
    this.intercept = !!on;
    this.send({ cmd: 'intercept', on: !!on });
  }

  // Release one held frame, waiting for its release ack.
  async drainOne(ev, timeout = 10000) {
    this.send({ cmd: 'release', id: ev.holdId });
    const ok = await this.waitReleaseAck(ev.holdId, timeout);
    if (!ok) this.holds.delete(ev.holdId); // stale (its session closed): drop
    return ok;
  }

  // Release held frames (in hold order, optionally filtered) one by one.
  async drain(pred, timeout = 60000) {
    const t0 = Date.now();
    for (;;) {
      const pending = this.held().filter(pred || (() => true));
      if (!pending.length) return;
      await this.drainOne(pending[0]);
      if (Date.now() - t0 > timeout) throw new Error('timeout draining holds');
      await sleep(30);
    }
  }

  async waitReleaseAck(holdId, timeout) {
    const t0 = Date.now();
    for (;;) {
      const i = this.pendingReleased.findIndex((r) => r.id === holdId);
      if (i >= 0) { const [ack] = this.pendingReleased.splice(i, 1); return ack.ok; }
      if (Date.now() - t0 > timeout) return false;
      await sleep(20);
    }
  }

  reset() {
    this.events.length = 0;
    this.holds.clear();
  }

  // ---- export (panel `copy` format) ---------------------------------------

  static pad(s, n) { s = String(s); return s.length >= n ? s : s + new Array(n - s.length + 1).join(' '); }
  static layerTag(ev) {
    if (ev.dir === 'open' || ev.dir === 'engine') return '[Transport WebSocket] ';
    if (ev.dir === 'held') {
      if (ev.kind === 'msg') return '';
      if (ev.meta && ev.meta.eio) return '[Transport Engine.IO] ';
      return '[Transport WebSocket] ';
    }
    if (ev.kind === 'msg') return '';
    if (ev.kind === 'eio') return '[Transport Engine.IO] ';
    return '';
  }
  static typeOf(ev) {
    if (ev.dir === 'open') return 'ws-open';
    if (ev.dir === 'engine') return ev.kind;
    if (ev.dir === 'held') return (ev.kind === 'msg' ? ev.meta.type : (ev.meta && ev.meta.eio ? ev.meta.eio : ''));
    if (ev.kind === 'msg') return ev.meta.type;
    if (ev.kind === 'eio') return ev.meta.eio + (ev.meta.sio ? '/' + ev.meta.sio : '');
    return ev.kind;
  }
  static formatLine(ev, user) {
    const ts = new Date(ev.t).toISOString().slice(11, 23);
    const dir = ev.dir === 'send' ? '->' : ev.dir === 'recv' ? '<-' : ev.dir === 'held' ? '##' : '  ';
    return Recorder.pad(ts, 13) + '  ' + dir + '  ' + Recorder.pad(user, 14) + '  ' +
      Recorder.layerTag(ev) + Recorder.typeOf(ev);
  }

  toMarkdown(infoLines) {
    const version = this.ooVersion
      ? this.ooVersion + (this.ooBuild != null ? ' (build ' + this.ooBuild + ')' : '')
      : 'unknown';
    let out = '## Info\n\n- Timestamp: ' + new Date().toISOString() +
      '\n- OnlyOffice version: ' + version;
    for (const line of infoLines || []) out += '\n- ' + line;
    const blocks = this.events
      // raw Socket.IO frames & binary frames are invisible in auto-flow
      // captures; keep the exported records consistent by leaving them out
      .filter((ev) => ev.meta !== undefined && ev.meta !== null)
      .filter((ev) => ev.kind !== 'raw' && ev.kind !== 'binary')
      .map((ev) => {
        const body = JSON.stringify(ev.meta, null, 2);
        return '### ' + Recorder.formatLine(ev, this.userOf(ev)) + '\n\n```json\n' + body + '\n```';
      })
      .join('\n\n');
    return out + '\n\n## Events\n\n' + blocks + '\n';
  }
}

// ---------------------------------------------------------------------------
// Predicates over recorder events
// ---------------------------------------------------------------------------

const isMsg = (ev) => ev.kind === 'msg';
const evUser = (ev) => (ev.user || null);
const msgType = (ev) => (isMsg(ev) && ev.meta && ev.meta.type) || null;
// The frames that must always flow through the gate: only OO application
// messages are choreographed, everything transport-level is released as is.
const isTransportHold = (ev) => ev.kind !== 'msg';

function sentEvent(rec, user, type, extra, fromIndex = 0, timeout = EVENT_TIMEOUT) {
  return rec.waitEvent((ev) => isMsg(ev) && ev.dir === 'send' && evUser(ev) === user &&
    msgType(ev) === type && (!extra || extra(ev)), timeout, user + ' -> ' + type, fromIndex);
}
function recvEvent(rec, user, type, extra, fromIndex = 0, timeout = EVENT_TIMEOUT) {
  return rec.waitEvent((ev) => isMsg(ev) && ev.dir === 'recv' && evUser(ev) === user &&
    msgType(ev) === type && (!extra || extra(ev)), timeout, user + ' <- ' + type, fromIndex);
}
const handshakeEventPred = (who) => (ev) =>
  isMsg(ev) && ev.dir === 'recv' && evUser(ev) === who && msgType(ev) === 'message';
// A client's handshake is complete once it received the (possibly empty)
// getMessages response — the last server event of the auth flow.
const handshakeDone = (rec, who) => recvEvent(rec, who, 'message', null, 0, EVENT_TIMEOUT).then(() => null);

// ---------------------------------------------------------------------------
// Editor helpers
// ---------------------------------------------------------------------------

const EDITORS = {
  Alice: 'document-editor-1',
  Bob: 'document-editor-2',
  Mallory: 'document-editor-3',
};

async function openRecordPage(browser, monitorSrc, proxyPort, pageServerPort, docType, docKey, deferAlice) {
  const ctx = await browser.newContext({ viewport: { width: 1920, height: 1080 }, bypassCSP: true });
  await ctx.addInitScript('window.__OO_PROXY_PORT = ' + proxyPort + ';\n' + monitorSrc);
  const page = await ctx.newPage();
  page.on('pageerror', (e) => console.log('  [pageerror]', String(e.message).slice(0, 120)));
  const url = 'http://127.0.0.1:' + pageServerPort + '/?doc=' + docType + '&key=' + docKey +
    (deferAlice ? '&defer1=1' : '');
  await page.goto(url, { waitUntil: 'domcontentloaded', timeout: 60000 });
  return { ctx, page };
}

function editorFrame(page, who) {
  return page.frames().find((f) => f.url().includes('frameEditorId=' + EDITORS[who]));
}

// Wait for an editor to be rendered (loading mask gone, layout present).
async function waitEditorRendered(page, who, timeout = EDITOR_LOAD_TIMEOUT) {
  const t0 = Date.now();
  for (;;) {
    const frame = editorFrame(page, who);
    if (frame) {
      try {
        const ready = await frame.evaluate(() => {
          for (const m of document.querySelectorAll('.loadmask')) {
            if (m.offsetParent !== null) return false;
          }
          let big = false;
          document.querySelectorAll('body *').forEach((el) => {
            if (big) return;
            const r = el.getBoundingClientRect();
            if (r.width > 300 && r.height > 200) big = true;
          });
          return big;
        });
        if (ready) return frame;
      } catch (e) { /* frame navigating */ }
    }
    if (Date.now() - t0 > timeout) throw new Error('timeout waiting for ' + who + "'s editor to render");
    await sleep(200);
  }
}

async function startEditor(page, who, view = false) {
  // The harness page boots its editors asynchronously (it waits for api.js, then
  // gives the monitor's trap a beat to wrap DocsAPI.DocEditor), and the monitor
  // defers them behind their overlay — so wait until this editor has actually
  // been constructed-and-trapped before starting it.
  // `view`: start the editor as a read-only viewer (the monitor flips the
  // captured config to mode 'view' + permissions.edit false — the same
  // connection the page's `?view=` param produced, but applied at start time,
  // so a participant can also be started, closed and re-joined as a viewer
  // within one record — see connectViewer / rejoinViewer).
  await page.waitForFunction(
    (id) => !!(window.__OO_MON && window.__OO_MON.deferred[id]),
    EDITORS[who],
    { timeout: 60000 }
  );
  await page.evaluate(({ id, view }) => {
    const MON = window.__OO_MON;
    if (MON && MON.deferred[id]) MON.startDeferred(id, view);
    else throw new Error('editor ' + id + ' is not deferred (already started?)');
  }, { id: EDITORS[who], view });
}

// The editor's main view element (the scrollable canvas area), in MAIN-PAGE
// coordinates: getBoundingClientRect() inside an editor iframe is relative to
// that iframe's viewport, while page.mouse.* works on the whole page — the
// iframe's own origin must be added or clicks meant for Bob's / Mallory's
// editors (right-hand and bottom panes) would land in Alice's instead.
async function mainViewBox(page, frame) {
  const box = await frame.evaluate(() => {
    // #editor_sdk first: all three editors (word/cell/slide) have it, and the
    // CURSOR_TARGETS fractions are calibrated against it (via OO_SCAN).
    const sels = ['#editor_sdk', '#id_box_main_view', '#box_document_container', '#box_editor_container',
      '#canvas', '.scroll-container', '#id_box_toolbar_wrapped'];
    for (const sel of sels) {
      const el = document.querySelector(sel);
      if (el && el.offsetWidth > 50 && el.offsetHeight > 50) {
        const r = el.getBoundingClientRect();
        return { x: r.x, y: r.y, width: r.width, height: r.height, sel };
      }
    }
    const r = document.body.getBoundingClientRect();
    return { x: r.x, y: r.y, width: r.width, height: r.height, sel: 'body' };
  });
  const el = await frame.frameElement();
  const fb = await el.boundingBox();
  if (fb) {
    box.pageX = fb.x + box.x;
    box.pageY = fb.y + box.y;
  } else {
    box.pageX = box.x;
    box.pageY = box.y;
  }
  return box;
}

// Click inside an editor at a position relative to its main view box.
async function clickInEditor(page, who, fx, fy, opts = {}) {
  const frame = editorFrame(page, who);
  if (!frame) throw new Error('no frame for ' + who);
  const box = await mainViewBox(page, frame);
  const x = box.pageX + box.width * fx;
  const y = box.pageY + box.height * fy;
  if (opts.double) await page.mouse.dblclick(x, y);
  else await page.mouse.click(x, y, { clickCount: opts.clickCount || 1 });
  return { x: Math.round(x), y: Math.round(y), sel: box.sel };
}

async function typeText(page, text, delay = 40) {
  await page.keyboard.type(text, { delay });
}

// Send a chat message from an editor (left toolbar → chat panel), then close
// the panel so it does not overlay later document clicks.
async function sendChatMessage(page, who, text) {
  const frame = editorFrame(page, who);
  if (!frame) throw new Error('no frame for ' + who);
  const chatBtn = frame.locator('#left-btn-chat');
  if (!(await chatBtn.isVisible().catch(() => false))) {
    throw new Error('chat button not visible for ' + who);
  }
  await chatBtn.click();
  const input = frame.locator('#chat-msg-text');
  await input.waitFor({ state: 'visible', timeout: 10000 });
  await input.fill(text);
  await frame.locator('#chat-msg-btn-add').click();
  const close = frame.locator('#chat-btn-close');
  if (await close.isVisible().catch(() => false)) await close.click();
}

// ---------------------------------------------------------------------------
// Common scenario building blocks (CURRENT is set by the main loop)
// ---------------------------------------------------------------------------

let CURRENT = null; // { rec, page, docType, docKey }

async function connectAlice() {
  const { rec, page } = CURRENT;
  // With `?defer1=1` (see the `deferAlice` scenario field), Alice is deferred
  // behind an overlay like the other editors, so that a scenario can have
  // another participant connect first: start her here in that case.
  const aliceDeferred = await page.evaluate(
    () => !!(window.__OO_MON && window.__OO_MON.deferred['document-editor-1'])
  );
  if (aliceDeferred) await startEditor(page, 'Alice');
  await handshakeDone(rec, 'Alice');
  await waitEditorRendered(page, 'Alice');
  await rec.waitQuiescence(800);
}

async function connectClient(who) {
  const { rec, page } = CURRENT;
  await startEditor(page, who);
  await handshakeDone(rec, who);
  await waitEditorRendered(page, who);
  await rec.waitQuiescence(800);
}

// Connect several clients "at the same time": their `auth` sends are held at
// the proxy until every one of them is ready, then released back to back, so
// the server sees the joins as concurrent. All server events keep flowing in
// hold (≈ emission) order — the waitAuth/auth/unLockDocument interplay of the
// concurrent joins is exactly what the server produced, in order.
async function connectConcurrentClients(whos) {
  const { rec, page } = CURRENT;
  rec.setIntercept(true);
  for (const who of whos) await startEditor(page, who);
  // 1. Let every client boot up to its `auth` send: release transport frames
  //    and `license` recvs (a client only sends its auth once it got the
  //    license — holding the license would deadlock the joins).
  const authsHeld = new Set();
  const t0 = Date.now();
  for (;;) {
    for (;;) {
      const ev = rec.held().find((e) => isTransportHold(e) ||
        (e.holdDir === 'recv' && msgType(e) === 'license'));
      if (!ev) break;
      await rec.drainOne(ev);
    }
    for (const ev of rec.held()) {
      if (ev.holdDir === 'send' && msgType(ev) === 'auth') authsHeld.add(evUser(ev));
    }
    if (whos.every((w) => authsHeld.has(w))) break;
    if (Date.now() - t0 > 120000) throw new Error('timeout in connectConcurrentClients (waiting for the auth sends)');
    await sleep(100);
  }
  // 2. All auths are held at the proxy: release them back to back.
  for (const who of whos) {
    const ev = rec.held().find((e) => e.holdDir === 'send' && msgType(e) === 'auth' && evUser(e) === who);
    if (ev) await rec.drainOne(ev);
  }
  // 3. Deliver everything still held in hold order: the server events of the
  //    concurrent joins flow as the server emitted them, and the clients'
  //    answers (e.g. Alice's unLockDocument) are released as they pile up.
  await rec.drain();
  rec.setIntercept(false);
  for (const who of whos) {
    await handshakeDone(rec, who);
    await waitEditorRendered(page, who);
  }
  await rec.waitQuiescence(800);
}

// Run `action` (an Alice-side cursor move / edit / chat message) while `who`
// is stuck mid-authentication: the newcomer's auth reached the server (it got
// its waitAuth), but its auth response is held at the proxy. Alice's own
// events flow freely (so her client knows the newcomer joined and has
// unlocked), everything addressed to the newcomer stays held.
async function actWhileAuthenticating(who, action) {
  const { rec, page } = CURRENT;
  const releasable = (e) => isTransportHold(e) || e.holdDir === 'send' ||
    // The `license` must flow too: it is a pre-auth event (a client sends its
    // auth only once it got the license — and it carries the participant
    // name, so by user-matching it would wrongly be held like the auth flow).
    (e.holdDir === 'recv' && msgType(e) === 'license') || evUser(e) !== who;
  rec.setIntercept(true);
  await startEditor(page, who);
  const t0 = Date.now();
  for (;;) {
    // release everything except the OO messages addressed to `who`
    for (;;) {
      const ev = rec.held().find(releasable);
      if (!ev) break;
      await rec.drainOne(ev);
    }
    // The window is open once the server has emitted `who`'s auth response
    // (held at the proxy): `who` is authenticated server-side but its client
    // has not processed it yet.
    if (rec.held().some((ev) => evUser(ev) === who && ev.holdDir === 'recv' && msgType(ev) === 'auth')) break;
    if (Date.now() - t0 > 60000) throw new Error('timeout waiting for ' + who + ' to be mid-authentication');
    await sleep(100);
  }
  await rec.waitQuiescence(500);
  // Keep pumping the releasable frames WHILE the action runs, in the
  // background: the action's own sends (e.g. Alice's `cursor`) would else
  // stay held at the proxy, deadlocking the action's wait for them.
  let pumping = true;
  const releaser = (async () => {
    while (pumping) {
      const ev = rec.held().find(releasable);
      if (!ev) { await sleep(50); continue; }
      await rec.drainOne(ev);
    }
  })();
  try {
    await action();
    await rec.waitQuiescence(500);
  } finally {
    pumping = false;
    await releaser;
  }
  // Close the window: deliver everything held for `who`, in order.
  await rec.drain();
  rec.setIntercept(false);
  await handshakeDone(rec, who);
  await waitEditorRendered(page, who);
  await rec.waitQuiescence(800);
}

// ---------------------------------------------------------------------------
// Disconnection / reconnection building blocks
//
// Three ways to leave, mapped to the wire (see the monitor panel's per-editor
// control bar and the proxy's cut/revive commands):
//   - closeEditor                the Parsec "user closes the file" path: the
//                                DocEditor is destroyed, nothing is sent, the
//                                websocket dies with it (no auto-reconnect).
//   - closeEditorWithCloseEvent  the protocol's graceful goodbye: the editor
//                                API sends the `close` client event (the call
//                                OnlyOffice itself uses on rights revocation),
//                                then the editor is destroyed.
//   - cutConnection              the "connection lost" path: the proxy
//                                terminates both legs without a close frame.
//                                With keepDead (default) it also kills the
//                                client's reconnection attempts (~2s apart,
//                                per the settings.reconnection of its auth
//                                event) until reviveConnection — the
//                                participant stays away; with {keepDead:false}
//                                the stock auto-reconnect happens and restores
//                                the session (~2s).
// ---------------------------------------------------------------------------

// The `[Transport WebSocket] ws-close` row the proxy emits for every socket
// death (client close / server close / monitor cut).
const isWsClose = (ev, who) =>
  ev.dir === 'engine' && ev.kind === 'ws-close' && ev.editor === EDITORS[who];

async function waitWsClose(rec, who, timeout = EVENT_TIMEOUT) {
  return rec.waitEvent((ev) => isWsClose(ev, who), timeout, who + ' ws-close');
}

// Graceful close: destroy the editor. Soft-close wait: an editor whose socket
// was already cut produces no new ws-close row.
async function closeEditor(page, rec, who) {
  await page.evaluate((id) => window.__OO_MON.closeEditor(id), EDITORS[who]);
  await waitWsClose(rec, who, 15000).catch(() => {});
  await rec.waitQuiescence(600);
}

// The protocol's `close` client event, then the destroy (the document server
// may close the socket itself on `close` — the record shows whichever
// happens).
async function closeEditorWithCloseEvent(page, rec, who) {
  await page.evaluate((id) => window.__OO_MON.sendCloseEvent(id), EDITORS[who]);
  await waitWsClose(rec, who, 10000).catch(() => {});
  await page.evaluate((id) => window.__OO_MON.closeEditor(id), EDITORS[who]);
  await rec.waitQuiescence(600);
}

// Violent close: cut the connection at the proxy (no close frame on either
// leg — an abnormal disconnect, like a network loss).
async function cutConnection(rec, who, { keepDead = true } = {}) {
  rec.send({ cmd: 'cut', editor: EDITORS[who], keep: keepDead ? 'dead' : 'once' });
  await waitWsClose(rec, who);
  await rec.waitQuiescence(500);
}

// End a keepDead cut: the editor's next reconnection attempt (~2s) flows again
// and completes the session restore — its re-`auth` carries the previous
// `sessionId` (with `sessionTimeConnect`/`sessionTimeIdle`). The document
// server then answers one of two ways (both are the record):
//   - the restored session's `auth` response (nothing changed while away),
//   - or a refusal: `refreshToken` + `error` 4010 "Restore error. Document
//     modified." when another participant changed the document while the
//     client was away (the client reacts by closing the co-authoring
//     session).
async function reviveConnection(rec, page, who) {
  // Everything of `who`'s BEFORE this point belongs to the previous stint
  // (handshake + cut): wait only for what comes after, or the waits would
  // match the original join events.
  const from = rec.events.length;
  rec.send({ cmd: 'revive', editor: EDITORS[who] });
  // Killed reconnection attempts grow the client's socket.io backoff (the
  // `settings.reconnection.delay` the document server sends is the STRING
  // "2s", which defeats the client-side delay cap — observed retry gaps: ~2s,
  // then ~20s, then longer). The wait resolves as soon as the client retries;
  // the cap just has to tolerate a slow next attempt. The client gives up
  // after `attempts` (50), so it is always coming back within this window.
  await rec.waitEvent(
    (ev) => ev.dir === 'open' && ev.editor === EDITORS[who],
    120000, who + ' reconnection ws-open', from);
  await sentEvent(rec, who, 'auth', (ev) => !!ev.meta.sessionId, from, 30000, who + ' restore auth (sessionId)');
  const { ev: answer } = await rec.waitEvent(
    (ev) => isMsg(ev) && ev.dir === 'recv' && evUser(ev) === who &&
      (msgType(ev) === 'auth' || msgType(ev) === 'error'),
    EVENT_TIMEOUT, who + ' restore answer (auth, or the 4010 refusal)');
  if (msgType(answer) === 'auth') await waitEditorRendered(page, who);
  // On the refusal path the editor shows its modified-document state; let
  // its reaction (e.g. the `close` send) flow into the record.
  await rec.waitQuiescence(1000);
}

// Re-join after a graceful close: the editor is re-created from its remembered
// config — a full fresh handshake (auth with a null sessionId), like a user
// opening the document again.
async function rejoinEditor(page, rec, who) {
  const from = rec.events.length; // ditto: only the new stint's events matter
  await page.evaluate((id) => window.__OO_MON.rejoinEditor(id), EDITORS[who]);
  // NOTE: the fresh-vs-restore distinction is on the auth SEND (a fresh join
  // sends `sessionId: null`, a reconnect sends the previous session id) — the
  // auth RESPONSE always carries the server-assigned session id.
  await recvEvent(rec, who, 'auth', null, from, EVENT_TIMEOUT, who + ' fresh re-join auth');
  await rec.waitEvent(
    (ev) => isMsg(ev) && ev.dir === 'recv' && evUser(ev) === who && msgType(ev) === 'message',
    EVENT_TIMEOUT, who + ' re-join getMessages response', from);
  await waitEditorRendered(page, who);
  await rec.waitQuiescence(800);
}

// Re-join after a graceful close, as a read-only viewer: the remembered
// config gets the viewer fields (mode 'view' + permissions.edit false — the
// client-side mode is fixed at editor boot, so switching to view mode
// requires a fresh construction, exactly what rejoinEditor's close path
// does). The completion point is the auth response, like connectViewer.
async function rejoinViewer(page, rec, who) {
  const from = rec.events.length; // ditto: only the new stint's events matter
  await page.evaluate((id) => window.__OO_MON.rejoinEditor(id, true), EDITORS[who]);
  await recvEvent(rec, who, 'auth', null, from, EVENT_TIMEOUT, who + ' viewer re-join auth');
  await waitEditorRendered(page, who);
  await rec.waitQuiescence(800);
}

// ---------------------------------------------------------------------------
// Scenario table
//
// `info` is the description baked into the record's `## Info` block: either a
// prose string (rendered as `- Scenario: <text>`) or an ordered list of steps
// (an array, rendered as `- Scenario:` followed by one indented numbered line
// per step — the preferred form: what happens, in order, with no mention of
// how the harness produced it).
// ---------------------------------------------------------------------------

const SCENARIOS = {};
function scenario(name, docType, info, fn, deferAlice) {
  // `deferAlice`: also defer Alice behind an overlay (see monitor.js's
  // `?defer1=1`), so another participant can connect first.
  SCENARIOS[name] = { name, docType, info, fn, deferAlice };
}

// `## Info` lines for a scenario: the document plus the description (prose or
// the ordered step list — see the Scenario table header).
function scenarioInfoLines(sc, docs) {
  const info = Array.isArray(sc.info)
    ? 'Scenario:\n' + sc.info.map((s, i) => '  ' + (i + 1) + '. ' + s).join('\n')
    : 'Scenario: ' + sc.info;
  return ['Document: ' + docs[sc.docType].file + ' (' + sc.docType + ')', info];
}

// --- auth ------------------------------------------------------------------

scenario('00_auth_alone', 'word', 'Alice opens the document alone.', async () => {
  await connectAlice();
});

scenario('01_auth_serial', 'word', 'Alice opens the document, then Bob joins, then Mallory.', async () => {
  await connectAlice();
  await connectClient('Bob');
  await connectClient('Mallory');
});

scenario('02_auth_concurrent', 'word',
  'Alice opens the document, then Bob and Mallory join at the same time: their auth' +
  ' sends are held at the proxy and released back to back, so both auths reach the' +
  ' server before Alice sends her unLockDocument. The server waitAuths the first' +
  ' joiner (Bob) and resolves the document lock optimistically for the second' +
  ' (Mallory authenticates directly) — the real OnlyOffice 9.3.0 behavior.', async () => {
  await connectAlice();
  await connectConcurrentClients(['Bob', 'Mallory']);
});

// --- cursor moves -----------------------------------------------------------

// Click targets per document type: fractions of the #editor_sdk box (see
// mainViewBox), calibrated with OO_SCAN (grid-click probes reading the emitted
// `cursor` messages; the caret position and the xlsx name box served as
// ground truth — details in the git history of this file).
const CURSOR_TARGETS = {
  word: [
    // The "Title" paragraph (first body line).
    { name: 'body-title', fx: 0.30, fy: 0.10 },
    // The page header: not reachable by plain click in this editor build (the
    // grayed header text ignores mouse clicks), so jump into it through the
    // editor API — a real client-side action producing the very same wire
    // events — then click on the header line to move the caret inside it.
    { name: 'header', fx: 0.30, fy: 0.20, api: 'header' },
  ],
  cell: [
    { name: 'cell-B3', fx: 0.25, fy: 0.125 },
    { name: 'cell-D4', fx: 0.55, fy: 0.20 },
  ],
  slide: [
    // The slide-1 text shape: a ~12px-tall hit zone (main-page y≈411 — the
    // slide auto-zooms to its pane) — clicks above/below it do nothing.
    { name: 'slide-text-a', fx: 0.55, fy: 0.42, double: true },
    { name: 'slide-text-b', fx: 0.68, fy: 0.42, double: true },
  ],
};

// Jump the caret into the page header of `who`'s editor (editor API).
async function enterHeader(page, who) {
  const frame = editorFrame(page, who);
  if (!frame) throw new Error('no frame for ' + who);
  await frame.evaluate(() => {
    const api = window['Asc'] && window['Asc'].editor;
    if (!api || typeof api.GoToHeader !== 'function') {
      throw new Error('GoToHeader not available in this editor');
    }
    api.GoToHeader(0);
  });
}

// Act on a cursor target: a plain (double)click, or the header API jump.
async function clickTarget(page, who, target) {
  if (target.api === 'header') await enterHeader(page, who);
  await clickInEditor(page, who, target.fx, target.fy, { double: target.double });
}

// Click a target and wait for the user's cursor send (a click that does not
// move the caret sends nothing: retry on the other target of the doc type).
// NOTE: cursor sends are throttled client-side to one per second, so the
// first event may take up to ~1s after the click.
async function moveCursor(who, targetIndex) {
  const { rec, page, docType } = CURRENT;
  const targets = CURSOR_TARGETS[docType];
  const target = targets[targetIndex % targets.length];
  const base = rec.events.length;
  await clickTarget(page, who, target);
  try {
    await sentEvent(rec, who, 'cursor', null, base, 8000);
  } catch (e) {
    const alt = targets[(targetIndex + 1) % targets.length];
    await clickTarget(page, who, alt);
    await sentEvent(rec, who, 'cursor', null, base, 8000);
  }
  await rec.waitQuiescence(400);
  return target.name;
}

for (const docType of ['word', 'cell', 'slide']) {
  const t = DOC_EXT[docType];

  scenario('10_cursor_serial_' + t, docType,
    'Alice, Bob and Mallory move their cursors one after the other (serial).', async () => {
    await connectAlice();
    await connectClient('Bob');
    await connectClient('Mallory');
    // Word: entering the header (target 1) auto-activates the header view on
    // the other clients (to show the remote cursor), diverting their body
    // clicks — so everyone does the body move before anyone enters the header.
    for (const who of ['Alice', 'Bob', 'Mallory']) await moveCursor(who, 0);
    for (const who of ['Alice', 'Bob', 'Mallory']) await moveCursor(who, 1);
  });

  scenario('13_cursor_concurrent_' + t, docType,
    'Alice, Bob and Mallory move their cursors at the same time (concurrent).', async () => {
    await connectAlice();
    await connectClient('Bob');
    await connectClient('Mallory');
    const { rec, page } = CURRENT;
    const targets = CURSOR_TARGETS[docType];
    const base = rec.events.length;
    // fire the clicks in quick succession, without waiting for the relays
    await clickTarget(page, 'Alice', targets[0]);
    await clickTarget(page, 'Bob', targets[0]);
    await clickTarget(page, 'Mallory', targets[1]);
    for (const who of ['Alice', 'Bob', 'Mallory']) {
      await sentEvent(rec, who, 'cursor', null, base, 15000);
    }
    await rec.waitQuiescence();
  });

  scenario('16_cursor_while_auth_' + t, docType,
    'Alice moves her cursor while Bob is authenticating (between his auth and its' +
    ' response, held by manual-flow).', async () => {
    await connectAlice();
    await actWhileAuthenticating('Bob', async () => {
      await moveCursor('Alice', 0);
      await moveCursor('Alice', 1);
    });
  });
}

// --- chat messages -----------------------------------------------------------

async function relayedMessageToOthers(fromWho, fromIndex) {
  // the message is relayed to the other participants as a `message` event
  // carrying it in its `messages` array
  const others = { Alice: ['Bob', 'Mallory'], Bob: ['Alice', 'Mallory'], Mallory: ['Alice', 'Bob'] }[fromWho];
  for (const other of others) {
    await recvEvent(CURRENT.rec, other, 'message', (ev) =>
      ev.meta && ev.meta.messages && JSON.stringify(ev.meta.messages).indexOf(fromWho) >= 0,
      fromIndex, 15000);
  }
  await CURRENT.rec.waitQuiescence(400);
}

scenario('20_message_alone', 'word', 'Alice sends a chat message while alone.', async () => {
  await connectAlice();
  await sendChatMessage(CURRENT.page, 'Alice', 'Hello from Alice (alone)');
  await sentEvent(CURRENT.rec, 'Alice', 'message', (ev) => ev.meta && ev.meta.message, 0, 10000);
  await CURRENT.rec.waitQuiescence();
});

scenario('21_message_serial', 'word', 'Alice, Bob and Mallory send chat messages one after the other.', async () => {
  await connectAlice();
  await connectClient('Bob');
  await connectClient('Mallory');
  const { rec } = CURRENT;
  let idx = 0;
  await sendChatMessage(CURRENT.page, 'Alice', 'Hello from Alice');
  ({ index: idx } = await sentEvent(rec, 'Alice', 'message', (ev) => ev.meta && ev.meta.message, idx, 10000));
  await relayedMessageToOthers('Alice', idx);
  await sendChatMessage(CURRENT.page, 'Bob', 'Hello from Bob');
  ({ index: idx } = await sentEvent(rec, 'Bob', 'message', (ev) => ev.meta && ev.meta.message, idx, 10000));
  await relayedMessageToOthers('Bob', idx);
  await sendChatMessage(CURRENT.page, 'Mallory', 'Hello from Mallory');
  ({ index: idx } = await sentEvent(rec, 'Mallory', 'message', (ev) => ev.meta && ev.meta.message, idx, 10000));
  await relayedMessageToOthers('Mallory', idx);
});

scenario('22_message_concurrent', 'word',
  'Alice and Bob send chat messages at the same time (manual-flow holds both sends' +
  ' until both are out).', async () => {
  await connectAlice();
  await connectClient('Bob');
  await connectClient('Mallory');
  const { rec, page } = CURRENT;
  rec.setIntercept(true);
  await sendChatMessage(page, 'Alice', 'Concurrent hello from Alice');
  await rec.waitHold((ev) => ev.holdDir === 'send' && msgType(ev) === 'message' && evUser(ev) === 'Alice');
  await sendChatMessage(page, 'Bob', 'Concurrent hello from Bob');
  await rec.waitHold((ev) => ev.holdDir === 'send' && msgType(ev) === 'message' && evUser(ev) === 'Bob');
  // both chat messages are held: release them back to back, then let the
  // relays flow
  await rec.drain((ev) => ev.holdDir === 'send' && msgType(ev) === 'message', 10000);
  await rec.drain();
  rec.setIntercept(false);
  await rec.waitQuiescence();
});

scenario('23_message_while_auth', 'word',
  'Alice sends a chat message while Bob is authenticating.', async () => {
  await connectAlice();
  await actWhileAuthenticating('Bob', async () => {
    await sendChatMessage(CURRENT.page, 'Alice', 'Hello Bob, from mid-auth Alice');
    await sentEvent(CURRENT.rec, 'Alice', 'message', (ev) => ev.meta && ev.meta.message, 0, 10000);
  });
});

// --- document edits ----------------------------------------------------------

// Edit actions per document type: click target (index into CURSOR_TARGETS),
// then type this text at the caret. In the spreadsheet, the trailing "\n"
// (Enter) commits the cell — without it nothing is saved.
const EDIT_STEPS = {
  word: [
    { target: 0, text: 'Hello from Alice. ' },  // body text ("Title" line)
    { target: 1, text: 'Edited header. ' },     // the page header
  ],
  cell: [
    { target: 0, text: '42\n' },                // cell B3, Enter commits
    { target: 1, text: 'Hello\n' },              // cell D4
  ],
  slide: [
    { target: 0, text: ' (edited)' },           // the slide text shape
  ],
};

// Type an edit and wait for the user's save cycle to settle
// (isSaveLock → saveLock → saveChanges → unSaveLock).
async function editDocument(who, stepIndex) {
  const { page, rec, docType } = CURRENT;
  const step = EDIT_STEPS[docType][stepIndex % EDIT_STEPS[docType].length];
  const target = CURSOR_TARGETS[docType][step.target];
  const base = rec.events.length;
  let idx = 0;
  await clickTarget(page, who, target);
  await rec.waitQuiescence(300);
  // In the spreadsheet the typed value REPLACES the cell, so typing the very
  // same value a previous participant saved is a no-op (no saveChanges at all)
  // — each participant types its own value there.
  const text = docType === 'cell' ? who + ': ' + step.text : step.text;
  await typeText(page, text);
  ({ index: idx } = await sentEvent(rec, who, 'saveChanges', null, base, 20000));
  await recvEvent(rec, who, 'unSaveLock', null, base, 20000);
  await rec.waitQuiescence(500);
  return idx;
}

for (const docType of ['word', 'cell', 'slide']) {
  const t = DOC_EXT[docType];

  scenario('30_edit_alone_' + t, docType, 'Alice edits the document while alone.', async () => {
    await connectAlice();
    for (let i = 0; i < EDIT_STEPS[docType].length; i++) await editDocument('Alice', i);
  });

  scenario('33_edit_serial_' + t, docType,
    'Alice, Bob and Mallory edit the document one after the other.', async () => {
    await connectAlice();
    await connectClient('Bob');
    await connectClient('Mallory');
    for (let i = 0; i < EDIT_STEPS[docType].length; i++) await editDocument('Alice', i);
    for (let i = 0; i < EDIT_STEPS[docType].length; i++) await editDocument('Bob', i);
    for (let i = 0; i < EDIT_STEPS[docType].length; i++) await editDocument('Mallory', i);
  });

  scenario('36_edit_concurrent_' + t, docType,
    'Alice, Bob and Mallory start editing at the same time: their isSaveLocks are' +
    ' held by manual-flow and released back to back, so the server sees the three' +
    ' lock requests before granting any save lock.', async () => {
    await connectAlice();
    await connectClient('Bob');
    await connectClient('Mallory');
    const { rec, page, docType: dt } = CURRENT;
    const target = CURSOR_TARGETS[dt][0];
    // place the carets first, in auto-flow (clicks also move the keyboard focus)
    await clickInEditor(page, 'Alice', target.fx, target.fy, { double: target.double });
    await clickInEditor(page, 'Bob', target.fx, target.fy, { double: target.double });
    await clickInEditor(page, 'Mallory', target.fx, target.fy, { double: target.double });
    await rec.waitQuiescence(500);
    rec.setIntercept(true);
    // type in each editor: the first keystroke triggers an isSaveLock, held;
    // the click to switch editors releases its cursor send right away.
    // In the spreadsheet the trailing Enter commits the cell (see EDIT_STEPS).
    const suffix = dt === 'cell' ? '\n' : '';
    const words = { Alice: 'Alice' + suffix, Bob: 'Bob' + suffix, Mallory: 'Mal' + suffix };
    for (const who of ['Alice', 'Bob', 'Mallory']) {
      await clickInEditor(page, who, target.fx, target.fy, { double: target.double });
      await rec.drain((ev) => ev.holdDir === 'send' && msgType(ev) === 'cursor', 5000).catch(() => {});
      await typeText(page, words[who]);
      await rec.waitHold((ev) => ev.holdDir === 'send' && msgType(ev) === 'isSaveLock' && evUser(ev) === who,
        20000, who + ' isSaveLock');
    }
    // all three isSaveLocks are held: release them back to back
    await rec.drain((ev) => msgType(ev) === 'isSaveLock', 10000);
    await rec.drain();
    rec.setIntercept(false);
    await rec.waitQuiescence();
  });

  scenario('39_edit_while_auth_' + t, docType,
    'Alice edits the document while Bob is authenticating.', async () => {
    await connectAlice();
    await actWhileAuthenticating('Bob', async () => {
      for (let i = 0; i < EDIT_STEPS[docType].length; i++) await editDocument('Alice', i);
    });
  });
}

// --- view mode (read-only participant) ---------------------------------------
//
// A view-mode participant is started with a viewer config applied by the
// monitor at start time (`startEditor` with `view`: mode 'view' +
// permissions.edit false — the same fields the harness page's `?view=` param
// sets, see index.html; applying them at start time is what lets scenario 45
// re-join a closed participant as a viewer). With this document server build
// a viewer is a *live* one (license `liveViewerSupport: true` + client
// `coEditingMode: "fast"`): he receives the saveChanges/authChanges/cursor/
// message relays but not the getLock/releaseLock ones, never gets a `waitAuth`
// (the auth lock counts non-view participants only), never holds it, and only
// ever sends auth/getMessages/message (chat)/clientLog — the client-side view
// mode gates everything else, and the server disconnects a viewer sending
// getLock/isSaveLock/saveChanges.
//
// Note: a viewer does not count toward the client-side edit-user count either,
// so an editor working with only viewers present stays in single-user mode
// and sends no `cursor` messages (onlyOffice's own single-user behavior).

// Connect a view-mode participant. The viewer's handshake is the same as an
// editor's (auth response, possibly authChanges first, documentOpen,
// getMessages/message), but the completion point used here is the `auth`
// response so that a missing getMessages would not break the scenario.
async function connectViewer(who) {
  const { rec, page } = CURRENT;
  await startEditor(page, who, true);
  await recvEvent(rec, who, 'auth', null, 0, EVENT_TIMEOUT);
  await waitEditorRendered(page, who);
  await rec.waitQuiescence(800);
}

scenario('40_view_auth_join', 'word',
  'Alice (edit) connects and sends a chat message, then Bob joins in view' +
  ' (read-only) mode: the server authenticates him directly (a viewer never' +
  ' gets a waitAuth), his auth response carries the chat history and the' +
  ' participant list with his `view: true` flag, and he can chat back.', async () => {
  await connectAlice();
  await sendChatMessage(CURRENT.page, 'Alice', 'Hello from Alice (before Bob joins)');
  await sentEvent(CURRENT.rec, 'Alice', 'message', (ev) => ev.meta && ev.meta.message, 0, 10000);
  await CURRENT.rec.waitQuiescence(400);
  await connectViewer('Bob');
  await sendChatMessage(CURRENT.page, 'Bob', 'Hello from Bob the viewer');
  await sentEvent(CURRENT.rec, 'Bob', 'message', (ev) => ev.meta && ev.meta.message, 0, 10000);
  await CURRENT.rec.waitQuiescence();
});

for (const docType of ['word', 'cell', 'slide']) {
  const t = DOC_EXT[docType];

  scenario('41_view_edit_with_viewer_' + t, docType,
    'Alice (edit) connects, Bob joins in view mode, then Alice moves her' +
    ' cursor, edits the document and sends a chat message, and Bob sends one' +
    ' back. NOTE: the cursor move emits nothing here — a viewer does not count' +
    ' toward the client-side edit-user count, so Alice stays in single-user' +
    ' mode and sends no cursor (the omission is part of the record); her save' +
    ' changes are on the contrary relayed to Bob (live viewer).', async () => {
    await connectAlice();
    await connectViewer('Bob');
    // The cursor move is expected to emit NOTHING (see the info above): just
    // click and let the record document the omission.
    await clickTarget(CURRENT.page, 'Alice', CURSOR_TARGETS[docType][0]);
    await CURRENT.rec.waitQuiescence(1500);
    await editDocument('Alice', 0);
    await sendChatMessage(CURRENT.page, 'Alice', 'Hello from Alice to the viewer');
    await sentEvent(CURRENT.rec, 'Alice', 'message', (ev) => ev.meta && ev.meta.message, 0, 10000);
    await sendChatMessage(CURRENT.page, 'Bob', 'Hello from Bob the viewer');
    await sentEvent(CURRENT.rec, 'Bob', 'message', (ev) => ev.meta && ev.meta.message, 0, 10000);
    await CURRENT.rec.waitQuiescence();
  });

  scenario('42_view_viewer_first_' + t, docType,
    'Bob (view) connects FIRST — the session (and the document conversion) is' +
    ' created by a read-only participant —, then Alice (edit) joins: no waitAuth' +
    ' is involved (a viewer never holds the auth lock) and Alice can edit the' +
    ' document; her save changes are relayed to Bob (live viewer).', async () => {
    await connectViewer('Bob');
    await connectAlice();
    await editDocument('Alice', 0);
    await CURRENT.rec.waitQuiescence();
  }, true);

  scenario('43_view_waitauth_with_viewer_' + t, docType,
    'Alice (edit) connects, Bob joins in view mode, then Mallory (edit)' +
    ' joins: the auth-lock dance (waitAuth / unLockDocument) happens between' +
    ' the two editors only — the viewer is counted out and stays silent despite' +
    ' receiving the connectState broadcasts (waitAuth: true). Then Alice and' +
    ' Mallory move their cursors (two editors: the client-side collaborative' +
    ' mode is on), relayed to Bob too.', async () => {
    await connectAlice();
    await connectViewer('Bob');
    await connectClient('Mallory');
    await moveCursor('Alice', 0);
    await moveCursor('Mallory', 0);
    await CURRENT.rec.waitQuiescence();
  });

  scenario('44_view_join_mid_edit_' + t, docType,
    'Alice (edit) connects and edits the document, then Bob (view) joins' +
    ' mid-edit: he receives the backlog of changes as authChanges' +
    ' (+ authChangesAck, live viewer) before his auth response, then Alice' +
    ' edits again and her save changes are relayed to him.', async () => {
    await connectAlice();
    await editDocument('Alice', 0);
    await connectViewer('Bob');
    await editDocument('Alice', 1);
    await CURRENT.rec.waitQuiescence();
  });
}

// Alice switches from editor to read-only viewer mid-record: she edits,
// closes her editor, then re-opens the document as a viewer — the mode change
// requires a fresh editor (see rejoinViewer), which is also the manual flow:
// ✕ close, then a viewer re-join.
scenario('45_view_rejoin_as_viewer', 'word', [
  'Alice (edit) opens the document alone and edits it.',
  'Alice closes her editor (graceful close: nothing is sent, the websocket dies with it), then re-opens the document as a read-only viewer: her fresh handshake carries `mode: "view"` + `permissions.edit: false` — she is authenticated directly (a viewer never gets a waitAuth) and receives the document state, including her own earlier changes, as authChanges.',
  'Mallory (edit) then joins and edits: her save changes are relayed to Alice (live viewer).',
], async () => {
  const { rec, page } = CURRENT;
  await connectAlice();
  await editDocument('Alice', 0);
  await closeEditor(page, rec, 'Alice');
  await rejoinViewer(page, rec, 'Alice');
  await connectClient('Mallory');
  await editDocument('Mallory', 0);
  await rec.waitQuiescence();
});

// --- element locks: join with locks, joiner edit, release --------------------
//
// These scenarios produce records where the `locks` field of the `auth`
// server event carries actual lock info (it has only ever been empty in the
// other records: their participants always connected before any lock-taking
// edit happened). The document lock store is filled by the `getLock` client
// message, and the `auth` event snapshots it when a participant completes its
// join — a dict for the word editor, a list for cell/slide, see sendAuthInfo
// in DocsCoServer.js. Word/slide take element locks for plain typing in
// strict co-editing mode only (the editor API call behind the "Co-editing
// mode: Strict" toolbar button); the spreadsheet locks the edited cell block
// even in fast mode, and a solo client never releases its locks (its
// saveChanges carries releaseLocks=false), so they persist until a
// collaborative save.

// The per-type step of "Alice takes her lock(s) while alone" (the `base`
// event index is captured before): the spreadsheet edits two cells (the
// locks accumulate, giving Bob's auth event several lock entries), word and
// slide edit a single element in strict mode.
async function aliceTakesLocks(elements, isCell) {
  const { rec, page } = CURRENT;
  let base = rec.events.length;
  if (isCell) {
    await clickTarget(page, 'Alice', elements[0]);
    await typeText(page, 'Alice locks B3\n');
    await sentEvent(rec, 'Alice', 'getLock', null, base, 20000, 'Alice B3 getLock');
    await recvEvent(rec, 'Alice', 'unSaveLock', null, base, 20000);
    await rec.waitQuiescence(800);
    base = rec.events.length;
    await clickTarget(page, 'Alice', elements[1]);
    await typeText(page, 'Alice locks D4');  // no Enter: the cell edit stays open
    await sentEvent(rec, 'Alice', 'getLock', null, base, 20000, 'Alice D4 getLock');
  } else {
    await clickTarget(page, 'Alice', elements[0]);
    await typeText(page, 'Locked by Alice. ');
    await sentEvent(rec, 'Alice', 'getLock', null, base, 20000, 'Alice getLock');
  }
  await rec.waitQuiescence(800);
}

async function lockJoinEditScenario(docType) {
  const { rec, page } = CURRENT;
  const elements = LOCK_ELEMENTS[docType];
  const isCell = docType === 'cell';
  const isSlide = docType === 'slide';

  // 1. Alice connects alone; word/slide need strict co-editing for plain
  //    typing to take an element lock (the spreadsheet locks in fast mode).
  await connectAlice();
  if (!isCell) await setStrictCoEditing(page, 'Alice', true);

  // 2. Alice takes her lock(s).
  await aliceTakesLocks(elements, isCell);

  // 3. Bob joins: his auth event must carry the lock table (a dict for word,
  //    a list for cell/slide). Word/slide: Bob also switches to strict, so
  //    his own edits take locks too.
  let base = rec.events.length;
  await connectClient('Bob');
  await recvEvent(
    rec, 'Bob', 'auth',
    (ev) => {
      const locks = ev.meta.locks;
      return !!locks && (Array.isArray(locks) ? locks.length : Object.keys(locks).length) > 0;
    },
    base, EVENT_TIMEOUT, 'Bob auth with non-empty locks'
  );
  if (!isCell) await setStrictCoEditing(page, 'Bob', true);

  // 4. Bob tries the element locked by Alice: his editor knows the lock table
  //    from its auth event, so the attempt is refused client-side — beyond
  //    the cursor move of the click, nothing happens on the wire.
  await clickTarget(page, 'Bob', elements[0]);
  await typeText(page, 'Bob tries the locked element');
  await rec.waitQuiescence(1500);

  // 5. Bob edits an element of his own (word: the second paragraph, cell: a
  //    third cell; slide: none, see step 7): with 2+ participants the client
  //    is in collaborative mode, so his saveChanges carries releaseLocks=true
  //    and the server releases his lock, the saveChanges relay to Alice
  //    carrying the released-lock descriptors. Word: strict collaborative
  //    changes are only flushed by an explicit save (see flushEditorSave).
  if (!isSlide) {
    base = rec.events.length;
    await clickTarget(page, 'Bob', isCell ? elements[2] : elements[1]);
    await typeText(page, isCell ? 'Bob locks D2\n' : 'Bob locks the subtitle. ');
    await sentEvent(rec, 'Bob', 'getLock', null, base, 20000, 'Bob own getLock');
    if (!isCell) await flushEditorSave(page, 'Bob');
    await sentEvent(rec, 'Bob', 'saveChanges', null, base, 20000);
    await recvEvent(rec, 'Alice', 'saveChanges', (ev) => (ev.meta.locks || []).length > 0,
      base, 20000, 'Alice saveChanges relay with Bob released lock');
    await rec.waitQuiescence(800);
  }

  // 6. Alice saves again — now collaboratively, so her saveChanges carries
  //    releaseLocks=true and the server releases her lock(s) (cell: the two
  //    blocks she holds), the relay to Bob carrying the descriptors.
  base = rec.events.length;
  await clickTarget(page, 'Alice', isCell ? elements[1] : elements[0]);
  await typeText(page, isCell ? ' Alice releases.\n' : ' Alice releases. ');
  if (!isCell) await flushEditorSave(page, 'Alice');
  await sentEvent(rec, 'Alice', 'saveChanges', (ev) => ev.meta.releaseLocks === true,
    base, 20000, 'Alice collaborative saveChanges');
  await recvEvent(rec, 'Bob', 'saveChanges', (ev) => (ev.meta.locks || []).length > 0,
    base, 20000, 'Bob saveChanges relay with Alice released locks');
  await recvEvent(rec, 'Alice', 'unSaveLock', null, base, 20000);
  await rec.waitQuiescence(800);

  // 7. The element Alice released is now free: Bob first saves to apply the
  //    foreign changes arrived with Alice's relay (in strict mode they are
  //    held until then — the "changes" notification *is* the save button,
  //    see web-apps), then locks and edits the element.
  if (!isCell) {
    await flushEditorSave(page, 'Bob');
    // Applying re-renders the document (which can swallow a click landing
    // during it): let it settle first.
    await rec.waitQuiescence(800);
  }
  base = rec.events.length;
  await clickTarget(page, 'Bob', elements[0]);
  await typeText(page, isCell ? 'Bob takes B3\n' : 'Bob takes the element. ');
  await sentEvent(rec, 'Bob', 'getLock', null, base, 20000);
  if (!isCell) await flushEditorSave(page, 'Bob');
  await sentEvent(rec, 'Bob', 'saveChanges', null, base, 20000);
  await rec.waitQuiescence(800);
}

scenario('50_lock_join_edit_xlsx', 'cell',
  'Alice locks two cell blocks (B3, then D4 — cell locks accumulate while she' +
  ' is alone) and Bob joins mid-edit: his auth event carries the lock table as' +
  ' a list (empty in every record so far). Bob try on B3 is refused' +
  ' client-side (his editor knows the locks from its auth event), he locks' +
  ' and edits D2 instead (his saveChanges — collaborative — releases his' +
  ' lock, the relay to Alice carrying its descriptor), then Alice saves' +
  ' collaboratively: her saveChanges releases both her blocks and Bob' +
  ' receives their descriptors, leaving B3 free for Bob to lock and edit.', async () => {
  await lockJoinEditScenario('cell');
});

scenario('51_lock_join_edit_docx', 'word',
  'Alice switches to strict co-editing (the API behind the Co-editing Mode:' +
  ' Strict button) and locks the Title paragraph by typing in it, then Bob' +
  ' joins: his auth event carries the lock table as a dict (empty in every' +
  ' record so far). Bob (also strict) try on the Title is refused' +
  ' client-side, he locks and edits the Subtitle instead (his saveChanges —' +
  ' collaborative — releases his paragraph lock, the relay to Alice carrying' +
  ' its descriptor), then Alice saves collaboratively: her saveChanges' +
  ' releases her paragraph lock and Bob receives its descriptor, leaving the' +
  ' Title free for Bob to lock and edit.', async () => {
  await lockJoinEditScenario('word');
});

scenario('52_lock_join_edit_pptx', 'slide',
  'Alice switches to strict co-editing and locks the slide text shape by' +
  ' typing in it, then Bob joins: his auth event carries the lock table as a' +
  ' list (empty in every record so far). Bob (also strict) cannot edit the' +
  ' locked shape — the slide has no other reachable text —, then Alice saves' +
  ' collaboratively: her saveChanges releases her shape lock and Bob receives' +
  ' its descriptor, leaving the shape free for Bob to lock and edit.', async () => {
  await lockJoinEditScenario('slide');
});

// --- disconnection / reconnection -------------------------------------------
//
// Three ways to leave (see the helpers above):
//   - close            the Parsec "user closes the file" path — the editor is
//                      destroyed, nothing is sent, no auto-reconnect
//   - close:msg        the protocol's `close` client event (the OnlyOffice API
//                      call used on rights revocation), then the destroy
//   - cut              "connection lost" — the socket dies without a close
//                      frame; the editor auto-reconnects (~2s) but its
//                      attempts are killed until revived (keepDead), so the
//                      participant stays away as long as the scenario needs
// And two ways back:
//   - restore          revive the cut connection: the client's own retry
//                      completes the session restore (auth carrying the
//                      previous sessionId)
//   - re-join (fresh)  re-create a closed editor: a full new handshake
//                      (auth with a null sessionId), like opening the file
//                      again
//
// The proxy records every socket death as a `[Transport WebSocket] ws-close`
// row, so each record shows its disconnect boundaries per participant (the
// record parser skips transport rows — interpreting them, e.g. as "drop this
// participant's connection now", is left to the test harness for now).

scenario('600_disconnect_close_alone', 'word', [
  'Alice opens the document alone.',
  "Alice closes her editor: the client sends no `close` event, the connection is closed (code 1001) and the editor does not reconnect.",
], async () => {
  await connectAlice();
  await closeEditor(CURRENT.page, CURRENT.rec, 'Alice');
});

scenario('601_disconnect_close_event_alone', 'word', [
  'Alice opens the document alone.',
  'Alice sends the `close` client event (the event the editor sends when it must tell the server it is leaving, e.g. on rights revocation).',
  'The document server answers with a new session token (refreshToken), then closes the connection after about 10 seconds.',
  'Alice closes her editor.',
], async () => {
  await connectAlice();
  await closeEditorWithCloseEvent(CURRENT.page, CURRENT.rec, 'Alice');
});

scenario('602_disconnect_close_rejoin_alone', 'word', [
  'Alice opens the document alone.',
  'Alice edits the document.',
  'Alice closes her editor: the client sends no `close` event and the connection is closed (code 1001).',
  'Alice re-joins with a new editor: her auth carries a null sessionId (a fresh handshake, not a connection restore), and the document server replays her own earlier changes as authChanges.',
], async () => {
  const { rec, page } = CURRENT;
  await connectAlice();
  await editDocument('Alice', 0);
  await closeEditor(page, rec, 'Alice');
  await rejoinEditor(page, rec, 'Alice');
});

scenario('603_disconnect_lost_reconnect_alone', 'word', [
  'Alice opens the document alone.',
  'Alice edits the document.',
  'Alice loses her connection (the websocket is closed without a close frame).',
  "About 2 seconds later Alice's editor reconnects by itself and sends its auth with the previous sessionId, sessionTimeConnect and sessionTimeIdle.",
  'The document server restores her edition session: the auth response and the lock table follow.',
], async () => {
  const { rec, page } = CURRENT;
  await connectAlice();
  await editDocument('Alice', 0);
  const base = rec.events.length;
  await cutConnection(rec, 'Alice', { keepDead: false }); // let the stock auto-reconnect happen
  await sentEvent(rec, 'Alice', 'auth', (ev) => !!ev.meta.sessionId, base, 30000, 'Alice restore auth (sessionId)');
  await recvEvent(rec, 'Alice', 'auth', null, base, 30000, 'Alice restore auth response');
  await waitEditorRendered(page, 'Alice');
  await rec.waitQuiescence();
});

scenario('604_disconnect_close_bob_joins_rejoin', 'word', [
  'Alice opens the document alone.',
  'Alice edits the document.',
  'Alice closes her editor: the client sends no `close` event and the connection is closed (code 1001).',
  'Bob joins while Alice is away and edits the document.',
  'Alice re-joins with a new editor: her auth carries a null sessionId and she receives the state Bob changed meanwhile.',
], async () => {
  const { rec, page } = CURRENT;
  await connectAlice();
  await editDocument('Alice', 0);
  await closeEditor(page, rec, 'Alice');
  await connectClient('Bob');
  await editDocument('Bob', 0);
  await rejoinEditor(page, rec, 'Alice');
});

scenario('605_disconnect_lost_bob_joins_reconnect', 'word', [
  'Alice opens the document alone.',
  'Alice edits the document.',
  'Alice loses her connection (the websocket is closed without a close frame).',
  'Bob joins and edits the document while Alice is away.',
  "Alice's connection comes back: her editor sends its auth with the previous sessionId.",
  'The document changed while she was away, so the document server refuses the restore with error 4010 "Restore error. Document modified.", and Alice\'s client answers by closing the co-authoring session.',
], async () => {
  const { rec, page } = CURRENT;
  await connectAlice();
  await editDocument('Alice', 0);
  await cutConnection(rec, 'Alice'); // keepDead: she stays away
  await connectClient('Bob');
  await editDocument('Bob', 0);
  await reviveConnection(rec, page, 'Alice');
});

scenario('606_disconnect_lost_with_lock_bob_takes_it', 'word', [
  'Alice opens the document alone and switches to strict co-editing.',
  'Alice takes the lock on the Title paragraph and edits it.',
  'Alice loses her connection (the websocket is closed without a close frame).',
  'Bob joins while Alice is away.',
  "Bob takes the element Alice had locked: the document server released Alice's lock when she disconnected, so Bob's getLock is granted and he edits and saves the element.",
  "Alice's connection comes back: her restore auth is refused with error 4010 \"Restore error. Document modified.\" since the document changed while she was away.",
], async () => {
  const { rec, page } = CURRENT;
  await connectAlice();
  await setStrictCoEditing(page, 'Alice', true);
  await aliceTakesLocks(LOCK_ELEMENTS.word, false);
  await cutConnection(rec, 'Alice');
  await connectClient('Bob');
  // Bob (also strict) tries the element Alice had locked: his editor knows
  // the lock table from its auth event — a released lock lets his getLock
  // through, a surviving lock makes his client refuse the edit client-side.
  const base = rec.events.length;
  await setStrictCoEditing(page, 'Bob', true);
  await clickTarget(page, 'Bob', LOCK_ELEMENTS.word[0]);
  await typeText(page, 'Bob takes the element. ');
  await flushEditorSave(page, 'Bob');
  try {
    await sentEvent(rec, 'Bob', 'getLock', null, base, 8000, "Bob getLock on Alice's element");
  } catch (e) { /* refused client-side — the record documents it */ }
  try {
    await sentEvent(rec, 'Bob', 'saveChanges', null, base, 20000, 'Bob saveChanges');
  } catch (e) { /* no edit happened */ }
  await rec.waitQuiescence(1000);
  await reviveConnection(rec, page, 'Alice');
});

scenario('607_disconnect_close_bob_present_rejoin', 'word', [
  'Alice opens the document.',
  'Bob joins: Alice and Bob edit together.',
  'Alice closes her editor: the client sends no `close` event and the connection is closed (code 1001).',
  'Bob receives the connectState without Alice.',
  'Alice re-joins with a new editor.',
], async () => {
  const { rec, page } = CURRENT;
  await connectAlice();
  await connectClient('Bob');
  const base = rec.events.length;
  await closeEditor(page, rec, 'Alice');
  await recvEvent(rec, 'Bob', 'connectState', (ev) =>
    !(ev.meta.participants || []).some((p) => p.username === 'Alice'),
    base, 30000, 'Bob connectState without Alice');
  await rec.waitQuiescence(800);
  await rejoinEditor(page, rec, 'Alice');
});

scenario('608_disconnect_lost_bob_present_reconnect', 'word', [
  'Alice opens the document.',
  'Bob joins: Alice and Bob edit together.',
  'Alice loses her connection (the websocket is closed without a close frame).',
  'Bob sees Alice leave (connectState without Alice) and edits the document alone.',
  "Alice's connection comes back: her editor sends its auth with the previous sessionId.",
  'The document changed while she was away, so the document server refuses the restore with error 4010 "Restore error. Document modified.", and Alice\'s client answers by closing the co-authoring session.',
], async () => {
  const { rec, page } = CURRENT;
  await connectAlice();
  await connectClient('Bob');
  const base = rec.events.length;
  await cutConnection(rec, 'Alice');
  await recvEvent(rec, 'Bob', 'connectState', (ev) =>
    !(ev.meta.participants || []).some((p) => p.username === 'Alice'),
    base, 30000, 'Bob connectState without Alice');
  await editDocument('Bob', 0);
  await reviveConnection(rec, page, 'Alice');
});

scenario('609_disconnect_lost_mid_save_bob_saves', 'word', [
  'Alice opens the document.',
  'Bob joins: Alice and Bob edit together.',
  'Alice starts a save: the document server grants her the save lock, but her changes are not committed yet.',
  'While her save is in flight, Alice loses her connection (the websocket is closed without a close frame).',
  'The document server cancels Alice\'s save when tearing her connection down, so Bob can take the save lock and his save cycle completes.',
  "Alice's connection comes back: her restore auth is refused with error 4010 \"Restore error. Document modified.\" since the document changed while she was away.",
], async () => {
    const { rec, page } = CURRENT;
    await connectAlice();
    await connectClient('Bob');
    // 1. Get Alice mid-save deterministically: manual-flow gates every frame;
    //    everything keeps flowing except Alice's own application frames, and
    //    her save-lock dance is allowed through (so she legitimately holds the
    //    save lock) — only her saveChanges stays held at the proxy.
    rec.setIntercept(true);
    const releasable = (e) => isTransportHold(e) || evUser(e) !== 'Alice' ||
      msgType(e) === 'isSaveLock' || msgType(e) === 'saveLock';
    let pumping = true;
    const releaser = (async () => {
      while (pumping) {
        const ev = rec.held().find(releasable);
        if (ev) await rec.drainOne(ev);
        else await sleep(50);
      }
    })();
    try {
      // typing triggers Alice's isSaveLock → saveLock → saveChanges (held)
      await clickTarget(page, 'Alice', CURSOR_TARGETS.word[0]);
      await typeText(page, 'Alice mid-save typing. ');
      await rec.waitHold((ev) => ev.holdDir === 'send' && msgType(ev) === 'saveChanges' && evUser(ev) === 'Alice',
        30000, 'Alice saveChanges held (mid-save)');
    } finally {
      pumping = false;
      await releaser;
    }
    // 2. Alice is mid-save: cut her connection (her held saveChanges goes
    //    stale — it never reaches the server).
    await cutConnection(rec, 'Alice');
    rec.setIntercept(false);
    await rec.drain().catch(() => {});
    // 3. Bob saves: the server must have cancelled Alice's pending save when
    //    tearing her session down, so his save cycle goes through.
    const base = rec.events.length;
    await editDocument('Bob', 0);
    await recvEvent(rec, 'Bob', 'unSaveLock', null, base, 20000, 'Bob save completed');
    // 4. Alice's connection is restored (her editor believed its saveChanges
    //    was sent — the record shows how the restore reconciles that; Bob
    //    modified the document, so the 4010 refusal is the expected answer).
    await reviveConnection(rec, page, 'Alice');
  });

scenario('610_disconnect_waitauth_holder_lost', 'word', [
  'Alice opens the document, so she holds the document auth lock.',
  'Bob starts joining: the document server answers his auth with waitAuth, so he waits for Alice to release the lock.',
  'While Bob is waiting, Alice loses her connection (the websocket is closed without a close frame).',
  "The document server releases the auth lock on Alice's disconnect and resolves Bob's waitAuth: Bob receives his auth response and completes his join.",
  "Alice's connection comes back: her editor sends its auth with the previous sessionId, and since the document was not modified while she was away, the document server restores her session (auth response and lock table); the collaboration with Bob resumes.",
], async () => {
    const { rec, page } = CURRENT;
    await connectAlice();
    // Arm manual-flow and start Bob, holding everything addressed to him while
    // the rest flows (Alice sees the join attempt and her own sends go
    // through), until the server has waitAuth'ed Bob.
    rec.setIntercept(true);
    const releasable = (e) => isTransportHold(e) || e.holdDir === 'send' ||
      (e.holdDir === 'recv' && msgType(e) === 'license') || evUser(e) !== 'Bob';
    await startEditor(page, 'Bob');
    const pump = async () => {
      for (;;) {
        const ev = rec.held().find(releasable);
        if (!ev) break;
        await rec.drainOne(ev);
      }
    };
    let t0 = Date.now();
    for (;;) {
      await pump();
      if (rec.held().some((ev) => evUser(ev) === 'Bob' && ev.holdDir === 'recv' && msgType(ev) === 'waitAuth')) break;
      if (Date.now() - t0 > 60000) throw new Error('timeout waiting for Bob to be waitAuth-ed');
      await sleep(100);
    }
    // Alice loses her connection while Bob is stuck waiting for her unlock.
    await cutConnection(rec, 'Alice');
    // Keep pumping: the server must resolve Bob's waitAuth once the auth-lock
    // holder is gone — his auth response then arrives (held).
    t0 = Date.now();
    for (;;) {
      await pump();
      if (rec.held().some((ev) => evUser(ev) === 'Bob' && ev.holdDir === 'recv' && msgType(ev) === 'auth')) break;
      if (Date.now() - t0 > 60000) throw new Error("timeout waiting for Bob's auth response after Alice's disconnect");
      await sleep(100);
    }
    // Deliver everything (Bob's auth flow). NOTE: intercept is switched off
    // BEFORE the drain — frames arriving while the backlog is being released
    // (e.g. Bob's clientLog/getMessages reactions) must flow directly, or they
    // would stay held forever once the drain has seen an empty backlog.
    rec.setIntercept(false);
    await rec.drain();
    await handshakeDone(rec, 'Bob');
    await waitEditorRendered(page, 'Bob');
    await rec.waitQuiescence(800);
    await reviveConnection(rec, page, 'Alice');
  });

// ---------------------------------------------------------------------------
// Lock probe (OO_LOCK_PROBE=word|cell|slide): investigate how to get actual
// lock info into the `locks` field of the `auth` server event (until now it
// has only been empty in the recorded scenarios: the participants always
// connected before any lock-taking edit happened). Steps: Alice takes an
// element lock while alone (strict co-editing for word/slide — the editor API
// call behind the "Co-editing mode: Strict" toolbar button — or a plain cell
// edit for the spreadsheet, which locks even in fast mode), then Bob joins
// (his auth event should then carry the lock: a dict for word, a list for
// cell/slide, see sendAuthInfo in DocsCoServer.js), Bob attempts the locked
// element, edits his own, and Alice releases (the releaseLocks sequence).
// Each step's relevant wire events are printed.
// ---------------------------------------------------------------------------

// Switch `who`'s editor between fast and strict co-editing: in strict mode the
// client takes element locks for its edits (element locks are also taken in
// fast mode by some actions, but plain typing is not one of them).
async function setStrictCoEditing(page, who, strict) {
  const frame = editorFrame(page, who);
  if (!frame) throw new Error('no frame for ' + who);
  await frame.evaluate((isStrict) => {
    const api = window['Asc'] && window['Asc'].editor;
    if (!api || typeof api.asc_SetFastCollaborative !== 'function') {
      throw new Error('asc_SetFastCollaborative not available in this editor');
    }
    api.asc_SetFastCollaborative(!isStrict);
  }, strict);
}

// Trigger the editor save (the API call behind Ctrl+S / the save button).
// Needed to flush changes in strict co-editing with 2+ participants: the
// client then never auto-saves (see the autoSave gate in asc_docs_api — the
// fast-collaborative and solo modes save on their own, see the records).
async function flushEditorSave(page, who) {
  const frame = editorFrame(page, who);
  if (!frame) throw new Error('no frame for ' + who);
  await frame.evaluate(() => {
    const api = window['Asc'] && window['Asc'].editor;
    if (!api || typeof api.asc_Save !== 'function') {
      throw new Error('asc_Save not available in this editor');
    }
    api.asc_Save();
  });
}

// Lockable elements per document type (calibrated like CURSOR_TARGETS,
// fractions of the #editor_sdk box): word — the paragraph Alice locks then
// the one Bob edits; cell — the two cells Alice locks (cell locks accumulate
// while she is alone) then the cell Bob edits; slide — the single reachable
// text shape (the slide scenarios use the release-then-edit flow).
const LOCK_ELEMENTS = {
  word: [
    { name: 'title', fx: 0.30, fy: 0.10 },
    { name: 'subtitle', fx: 0.35, fy: 0.39 },
  ],
  cell: [
    { name: 'cell-B3', fx: 0.25, fy: 0.125 },
    { name: 'cell-D4', fx: 0.55, fy: 0.20 },
    { name: 'cell-D2', fx: 0.55, fy: 0.106 },
  ],
  slide: [
    { name: 'slide-text-a', fx: 0.55, fy: 0.42, double: true },
    { name: 'slide-text-b', fx: 0.68, fy: 0.42, double: true },
  ],
};

async function runLockProbe(browser, monitorSrc, proxyPort, pageServerPort, docType) {
  const rec = new Recorder(proxyPort);
  await rec.connect();
  const docKey = 'lockprobe' + Date.now().toString(36);
  const { ctx, page } = await openRecordPage(browser, monitorSrc, proxyPort, pageServerPort, docType, docKey);
  CURRENT = { rec, page, docType, docKey };
  await connectAlice();

  const dump = async (label, from) => {
    const evs = rec.events.slice(from).filter((ev) => isMsg(ev));
    console.log('probe: --- ' + label);
    for (const ev of evs) {
      let extra = '';
      if (msgType(ev) === 'auth') extra = ' locks=' + JSON.stringify(ev.meta.locks);
      else if (msgType(ev) === 'getLock') extra = ev.meta.block !== undefined
        ? ' block=' + JSON.stringify(ev.meta.block) : ' locks=' + JSON.stringify(ev.meta.locks);
      else if (msgType(ev) === 'saveChanges') extra = ' releaseLocks=' + ev.meta.releaseLocks
        + ' locks=' + JSON.stringify(ev.meta.locks) + ' n=' + ((ev.meta.changes || []).length)
        + (ev.dir === 'send' ? ' start=' + ev.meta.startSaveChanges + ' end=' + ev.meta.endSaveChanges : '');
      else if (msgType(ev) === 'clientLog') extra = ' ' + JSON.stringify(ev.meta.msg || '').slice(0, 90);
      console.log('probe:    ' + ev.dir + ' ' + evUser(ev) + ' ' + msgType(ev) + extra);
    }
    if (!evs.length) console.log('probe:    (no message events)');
  };

  const elements = LOCK_ELEMENTS[docType];

  // 1. Strict co-editing for word/slide (the spreadsheet locks in fast mode)
  if (docType !== 'cell') {
    await setStrictCoEditing(page, 'Alice', true);
    console.log('probe: Alice switched to strict co-editing');
    await sleep(500);
  }

  // 2. Alice takes the lock on the first element (cell edit kept open: the
  //    spreadsheet takes the block lock on the first keystroke)
  let base = rec.events.length;
  await clickTarget(page, 'Alice', elements[0]);
  await typeText(page, docType === 'cell' ? 'AliceLocked' : 'Locked by Alice. ');
  await sleep(3000);
  await dump('Alice edit (lock taking)', base);

  // 3. Bob joins: his auth event should carry Alice's lock
  base = rec.events.length;
  await connectClient('Bob');
  const bobAuth = rec.events.find(
    (ev) => isMsg(ev) && ev.dir === 'recv' && evUser(ev) === 'Bob' && msgType(ev) === 'auth'
  );
  console.log('probe: Bob auth locks = ' + (bobAuth ? JSON.stringify(bobAuth.meta.locks) : 'NO AUTH EVENT'));
  await dump('Bob join', base);
  if (docType !== 'cell') {
    await setStrictCoEditing(page, 'Bob', true);
    console.log('probe: Bob switched to strict co-editing');
  }

  // 4. Bob attempts the element locked by Alice
  base = rec.events.length;
  await clickTarget(page, 'Bob', elements[0]);
  await typeText(page, 'BobTry');
  await sleep(3000);
  await dump('Bob same-element attempt', base);

  // 5. Bob edits a different element (word/cell) — his own lock + save cycle
  if (docType !== 'slide') {
    base = rec.events.length;
    await clickTarget(page, 'Bob', elements[1]);
    await typeText(page, docType === 'cell' ? 'BobEdit\n' : 'Bob edit here. ');
    await sleep(3000);
    await dump('Bob own-element edit', base);
    if (docType === 'word') {
      // Strict + collaborative mode: the client never auto-saves (see the
      // autoSave gate in asc_docs_api) — flush Bob's batch with the editor
      // save API (what the Ctrl+S button calls).
      base = rec.events.length;
      const savedText = await editorFrame(page, 'Bob').evaluate(() => {
        const api = window['Asc'] && window['Asc'].editor;
        const doc = api && api.WordControl && api.WordControl.m_oLogicDocument;
        // What did Bob's typing actually do to the document?
        const texts = [];
        if (doc) {
          for (const p of doc.Content.slice(0, 8)) {
            texts.push(p && p.GetAllContentText ? p.GetAllContentText().slice(0, 40) : '?');
          }
        }
        if (api && typeof api.asc_Save === 'function') api.asc_Save();
        return texts;
      });
      console.log('probe: Bob document paragraphs: ' + JSON.stringify(savedText));
      await sleep(3000);
      await dump('Bob asc_Save flush?', base);
    }
  }

  // 6. Alice saves again — now collaboratively, so her saveChanges carries
  //    releaseLocks=true and releases her lock(s).
  base = rec.events.length;
  await clickTarget(page, 'Alice', docType === 'cell' ? elements[1] : elements[0]);
  await typeText(page, docType === 'cell' ? ' Alice releases.\n' : ' Alice releases. ');
  if (docType !== 'cell') await flushEditorSave(page, 'Alice');
  await sleep(3000);
  await dump('Alice release sequence', base);

  // 7. Bob applies the now-pending foreign changes (in strict mode they are
  //    held until the user saves — the "changes" notification *is* the save
  //    button), then edits the element Alice released.
  if (docType !== 'cell') {
    await flushEditorSave(page, 'Bob');
    await sleep(1500);
  }
  base = rec.events.length;
  await clickTarget(page, 'Bob', elements[0]);
  await typeText(page, docType === 'cell' ? 'BobAfterRelease\n' : 'Bob after release. ');
  await sleep(2500);
  if (docType !== 'cell') await flushEditorSave(page, 'Bob');
  await sleep(3000);
  await dump('Bob edit after release', base);

  if (!STAY_ALIVE) await ctx.close();
}

// ---------------------------------------------------------------------------
// UI scan (OO_SCAN=word|cell|slide): grid-click Alice's editor with Bob
// connected (cursor events are only emitted to relay to other participants)
// and report which points emit `cursor` messages with which values — the raw
// data used to pick the CURSOR_TARGETS fractions per document type.
// ---------------------------------------------------------------------------

async function runScan(browser, monitorSrc, proxyPort, pageServerPort, docType) {
  const rec = new Recorder(proxyPort);
  await rec.connect();
  const docKey = 'scan' + Date.now().toString(36);
  const { ctx, page } = await openRecordPage(browser, monitorSrc, proxyPort, pageServerPort, docType, docKey);
  CURRENT = { rec, page, docType, docKey };
  await handshakeDone(rec, 'Alice');
  const frame = await waitEditorRendered(page, 'Alice');
  await connectClient('Bob');

  const box = await mainViewBox(page, frame);
  console.log('scan: editor box = ' + JSON.stringify(box));
  const innerGeom = await frame.evaluate(() => {
    const out = [];
    const walk = (el, depth) => {
      if (depth > 6 || out.length > 40) return;
      const r = el.getBoundingClientRect();
      if (r.width > 40 && r.height > 20) {
        out.push('<' + el.tagName.toLowerCase() + '>' + (el.id ? '#' + el.id : '') +
          ' [' + Math.round(r.x) + ',' + Math.round(r.y) + ' ' + Math.round(r.width) + 'x' + Math.round(r.height) + ']');
      }
      for (const c of el.children || []) walk(c, depth + 1);
    };
    const sdk = document.querySelector('#editor_sdk') || document.body;
    for (const c of sdk.children || []) walk(c, 0);
    return out.join('\n');
  });
  console.log('scan: editor inner geometry:\n' + innerGeom);
  if (process.env.OO_SCAN_API) {
    // One-off: dump the editor API object's method names matching
    // subdoc/header/footer keywords — to find how to ask "where is the caret".
    const apiInfo = await frame.evaluate(() => {
      const out = [];
      const api = window['Asc'] && window['Asc'].editor;
      out.push('Asc.editor: ' + (api ? Object.prototype.toString.call(api) : 'none'));
      if (api) {
        const names = new Set();
        for (let o = api; o; o = Object.getPrototypeOf(o)) {
          for (const n of Object.getOwnPropertyNames(o)) names.add(n);
        }
        out.push('methods w/ subdoc|header|footer|headerfooter: ' +
          Array.from(names).filter((n) => /subdoc|header|footer/i.test(n)).join(', '));
        out.push('methods w/ ^(goto|enter|select|put_|start|begin) or text|slide|drawing: ' +
          Array.from(names).filter((n) => /^(goto|enter|select|put_|start|begin)/i.test(n) || (/text|slide|drawing/i.test(n) && !/callback/i.test(n))).slice(0, 100).join(', '));
        out.push('has GetCurrentSubDocument: ' + typeof api.GetCurrentSubDocument);
        out.push('has GetCurrentPage: ' + typeof api.GetCurrentPage);
        try {
          out.push('GetCurrentPage() = ' + api.GetCurrentPage());
        } catch (e) { out.push('GetCurrentPage() err: ' + e); }
      }
      return out.join('\n');
    });
    console.log('scan: editor api:\n' + apiInfo);
    // Where is the header? Jump into it programmatically: the `cursor` event
    // this emits gives the header's cursor value, to be reproduced later with
    // a plain mouse click (the scenario uses clicks, not the API).
    const beforeGoTo = rec.events.length;
    const goToInfo = await frame.evaluate(async () => {
      const api = window['Asc'] && window['Asc'].editor;
      if (!api || typeof api.GoToHeader !== 'function') return 'no GoToHeader';
      try { api.GoToHeader(0); } catch (e) { return 'GoToHeader err: ' + e; }
      await new Promise((r) => setTimeout(r, 1200));
      let active = 'err';
      try { active = api.GetActiveHeader() ? 'yes' : 'no'; } catch (e) {}
      return 'after GoToHeader(0): activeHeader=' + active;
    });
    await sleep(1500);
    const goToCursor = rec.events.slice(beforeGoTo)
      .filter((ev) => isMsg(ev) && ev.dir === 'send' && evUser(ev) === 'Alice' && msgType(ev) === 'cursor');
    console.log('scan: ' + goToInfo + '  cursor sends: ' +
      (goToCursor.length ? goToCursor.map((s) => JSON.stringify(s.meta)).join(' ') : 'none'));
  }

  // Default scan: a 7×9 grid over the editor box. Override with
  // OO_SCAN_REGION="fx0,fy0,fx1,fy1,cols,rows" (fractions of the box) for a
  // focused pass (e.g. around a document header).
  let fx0 = 1 / 7, fy0 = 1 / 9, fx1 = 6 / 7, fy1 = 8 / 9, COLS = 7, ROWS = 9;
  if (process.env.OO_SCAN_REGION) {
    const p = process.env.OO_SCAN_REGION.split(',').map(Number);
    [fx0, fy0, fx1, fy1, COLS, ROWS] = p;
  }
  const hits = [];
  for (let r = 0; r < ROWS; r++) {
    for (let c = 0; c < COLS; c++) {
      const fx = fx0 + (fx1 - fx0) * (COLS === 1 ? 0 : c / (COLS - 1));
      const fy = fy0 + (fy1 - fy0) * (ROWS === 1 ? 0 : r / (ROWS - 1));
      const x = Math.round(box.pageX + box.width * fx);
      const y = Math.round(box.pageY + box.height * fy);
      const before = rec.events.length;
      const reps = parseInt(process.env.OO_SCAN_REPEAT || '1', 10);
      for (let i = 0; i < reps; i++) {
        if (docType === 'slide') await page.mouse.dblclick(x, y);
        else await page.mouse.click(x, y);
        // 1st dblclick on a slide shape only *selects* it; a following one
        // enters its text — for slide, give the 2nd dblclick a beat to land.
        if (i + 1 < reps) await sleep(400);
      }
      // >1.3s: the client throttles `cursor` sends to one per second, so a
      // shorter wait can read before the send has gone out.
      await sleep(1400);
      const sends = rec.events.slice(before)
        .filter((ev) => isMsg(ev) && ev.dir === 'send' && evUser(ev) === 'Alice' && msgType(ev) === 'cursor');
      // Context probe: what the editor UI says about where the caret/selection
      // now is (#headerfooter contextual tab = word header/footer active,
      // #ce-cell-name = xlsx selected cell).
      let ctx = '';
      try {
        ctx = await frame.evaluate(() => {
          const vis = (el) => el && !!(el.offsetParent || (el.offsetWidth > 0 && el.offsetHeight > 0));
          const parts = [];
          const hf = document.querySelector('#headerfooter');
          if (hf) parts.push('headerTab=' + vis(hf));
          const tb = document.querySelector('#table');
          if (tb) parts.push('tableTab=' + vis(tb));
          const cn = document.querySelector('#ce-cell-name');
          if (cn) parts.push('cellName=' + (cn.value || cn.textContent || ''));
          const ta = document.querySelector('#area_id');
          if (ta) {
            const r = ta.getBoundingClientRect();
            parts.push('caret@' + Math.round(r.x) + ',' + Math.round(r.y));
          }
          const api = window['Asc'] && window['Asc'].editor;
          if (api && typeof api.GetActiveHeader === 'function') {
            try {
              const h = api.GetActiveHeader();
              parts.push('activeHeader=' + (h ? (h.Get_Id ? 'id:' + h.Get_Id() : 'yes') : 'no'));
            } catch (e) { parts.push('activeHeader=err'); }
          }
          return parts.join(' ');
        });
      } catch (e) { ctx = 'ctx-err'; }
      const line = 'scan: fx=' + fx.toFixed(3) + ' fy=' + fy.toFixed(3) +
        ' (' + x + ',' + y + ')' + (ctx ? ' [' + ctx + ']' : '') + ' -> ' +
        (sends.length ? sends.map((s) => JSON.stringify(s.meta)).join(' ') : 'none');
      console.log(line);
      if (sends.length) hits.push(line);
    }
  }
  console.log('scan: hits:');
  for (const h of hits) console.log('  ' + h);
  if (!STAY_ALIVE) await ctx.close();
}

// ---------------------------------------------------------------------------
// UI probe (OO_PROBE=word|cell|slide): clicks/types/chats around one editor
// and screenshots each step, to calibrate CURSOR_TARGETS / EDIT_STEPS.
// ---------------------------------------------------------------------------

async function runProbe(browser, monitorSrc, proxyPort, pageServerPort, docType) {
  fs.rmSync(PROBE_DIR, { recursive: true, force: true });
  fs.mkdirSync(PROBE_DIR, { recursive: true });
  const rec = new Recorder(proxyPort);
  await rec.connect();
  const docKey = 'probe' + Date.now().toString(36);
  const { ctx, page } = await openRecordPage(browser, monitorSrc, proxyPort, pageServerPort, docType, docKey);
  CURRENT = { rec, page, docType, docKey };
  await handshakeDone(rec, 'Alice');
  await waitEditorRendered(page, 'Alice');
  console.log('probe: frames = ' + page.frames().map((f) => f.url().slice(0, 100)).join('\n  '));
  const frame = editorFrame(page, 'Alice');

  // dump the DOM structure of the editor frame (elements big enough to matter)
  const dom = await frame.evaluate(() => {
    const out = [];
    const walk = (el, depth) => {
      if (depth > 7 || out.length > 400) return;
      const r = el.getBoundingClientRect ? el.getBoundingClientRect() : { width: 0 };
      if (r.width > 40 && r.height > 30) {
        out.push('  '.repeat(depth) + '<' + el.tagName.toLowerCase() + '>' +
          (el.id ? '#' + el.id : '') +
          (el.className && typeof el.className === 'string' ? '.' + el.className.trim().split(/\s+/).join('.') : '') +
          ' [' + Math.round(r.x) + ',' + Math.round(r.y) + ' ' + Math.round(r.width) + 'x' + Math.round(r.height) + ']');
      }
      for (const c of el.children || []) walk(c, depth + 1);
    };
    walk(document.body, 0);
    return out.join('\n');
  });
  fs.writeFileSync(path.join(PROBE_DIR, 'dom.txt'), dom);

  const shot = async (name) => page.screenshot({ path: path.join(PROBE_DIR, name + '.png') });
  await shot('0-initial');
  console.log('probe: main view box = ' + JSON.stringify(await mainViewBox(page, frame)));

  // chat while alone
  try {
    await sendChatMessage(page, 'Alice', 'probe chat message (alone)');
    await sleep(700);
    await shot('chat-alone');
    const msgs = rec.events.filter((ev) => isMsg(ev) && ev.dir === 'send' && msgType(ev) === 'message' && ev.meta.message);
    console.log('probe: chat (alone) → message sends: ' + msgs.length +
      (msgs.length ? ' ' + JSON.stringify(msgs[0].meta).slice(0, 200) : ''));
  } catch (e) {
    console.log('probe: chat (alone) failed: ' + e.message);
    await shot('chat-alone-failed');
  }

  // connect Bob, then probe clicks + typing
  await connectClient('Bob');
  const targets = CURSOR_TARGETS[docType];
  for (let i = 0; i < targets.length; i++) {
    const before = rec.events.length;
    const target = targets[i];
    const pos = await clickInEditor(page, 'Alice', target.fx, target.fy, { double: target.double });
    await sleep(800);
    await shot((i + 1) + '-click-' + target.name);
    const cursorSends = rec.events.slice(before)
      .filter((ev) => isMsg(ev) && ev.dir === 'send' && msgType(ev) === 'cursor');
    console.log('probe: click ' + target.name + ' at ' + JSON.stringify(pos) + ' → cursor sends: ' +
      cursorSends.length + (cursorSends.length ? ' ' + JSON.stringify(cursorSends[0].meta).slice(0, 160) : ''));
  }

  // typing probe at the last click position
  let before = rec.events.length;
  await typeText(page, 'probe-typing');
  await sleep(2000);
  await shot('typing');
  const saves = rec.events.slice(before).filter((ev) => isMsg(ev) && ev.dir === 'send' && msgType(ev) === 'saveChanges');
  console.log('probe: typing → saveChanges sends: ' + saves.length);

  // chat while Bob is connected
  try {
    await sendChatMessage(page, 'Alice', 'probe chat message (with Bob)');
    await sleep(700);
    await shot('chat-with-bob');
    before = rec.events.length;
    const msgs = rec.events.filter((ev) => isMsg(ev) && ev.dir === 'send' && msgType(ev) === 'message' && ev.meta.message);
    console.log('probe: chat (with Bob) → message sends: ' + msgs.length);
  } catch (e) {
    console.log('probe: chat (with Bob) failed: ' + e.message);
    await shot('chat-with-bob-failed');
  }

  if (!STAY_ALIVE) await ctx.close();
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

(async () => {
  const only = process.argv.slice(2);
  for (const n of only) {
    if (!SCENARIOS[n] && n !== 'list') {
      console.error('✖ unknown scenario: ' + n + '\n  known: ' + Object.keys(SCENARIOS).join('\n         '));
      process.exit(1);
    }
  }
  if (only.includes('list')) {
    for (const n of Object.keys(SCENARIOS)) {
      const info = Array.isArray(SCENARIOS[n].info) ? SCENARIOS[n].info.join(' ') : SCENARIOS[n].info;
      console.log(n + '  [' + SCENARIOS[n].docType + ']  ' + info);
    }
    return;
  }
  const names = Object.keys(SCENARIOS).filter((n) => !only.length || only.includes(n));
  if (PROBE_DOC) console.log('→ probe mode: ' + PROBE_DOC);

  console.log('→ container engine: ' + ENGINE + '    image: ' + IMAGE);
  const docs = {};
  for (const type of Object.keys(DOC_TYPES)) {
    const doc = Object.assign({}, DOC_TYPES[type], { path: path.join(SAMPLE_DOC_DIR, DOC_TYPES[type].file) });
    if (fs.existsSync(doc.path)) docs[type] = doc;
    else console.log('  (sample document missing, type disabled: ' + type + ' — ' + doc.path + ')');
  }

  const existingDsPort = parseInt(process.env.OO_EXISTING_DS_PORT || '0', 10);
  let dsPort;
  if (existingDsPort) {
    // Reuse an already-running document server (see the env doc above): no
    // container lifecycle, just sanity-check the healthcheck endpoint.
    dsPort = existingDsPort;
    console.log('→ reusing the already-running document server on http://127.0.0.1:' + dsPort);
    const r = await httpGet('http://127.0.0.1:' + dsPort + '/healthcheck', 5000).catch(() => null);
    if (!r || r.status !== 200 || !/true/.test(r.body)) {
      throw new Error('no healthy document server on http://127.0.0.1:' + dsPort);
    }
  } else {
    dsPort = await pickDsPort();
    engine(['rm', '-f', CONTAINER], { stdio: 'ignore' });
    const runArgs = containerArgs(dsPort);
    console.log('→ $ ' + ENGINE + ' ' + runArgs.join(' '));
    const run = engine(runArgs, { stdio: 'inherit' });
    if (run.status !== 0) throw new Error(ENGINE + ' run failed (exit ' + run.status + ')');
    containerStarted = true;
    await waitDsReady(dsPort);
  }

  const proxy = await createProxy({ port: 0, verbose: !!process.env.PROXY_VERBOSE });
  console.log('→ MITM proxy on ws://127.0.0.1:' + proxy.port + '/oo');
  const pages = await startPageServer(dsPort, docs);
  console.log('→ harness page on http://127.0.0.1:' + pages.port + '/');

  const monitorSrc = fs.readFileSync(MONITOR, 'utf8');
  const browser = await chromium.launch({
    headless: HEADLESS,
    args: [
      '--disable-blink-features=AutomationControlled',
      '--no-sandbox',
      '--allow-running-insecure-content',
      '--disable-web-security',
    ],
  });

  const rec = new Recorder(proxy.port);
  await rec.connect();

  const failed = [];
  try {
    if (process.env.OO_LOCK_PROBE) {
      await runLockProbe(browser, monitorSrc, proxy.port, pages.port, process.env.OO_LOCK_PROBE);
      return;
    }
    if (process.env.OO_SCAN) {
      await runScan(browser, monitorSrc, proxy.port, pages.port, process.env.OO_SCAN);
      return;
    }
    if (PROBE_DOC) {
      await runProbe(browser, monitorSrc, proxy.port, pages.port, PROBE_DOC);
      return;
    }

    fs.mkdirSync(OUT_DIR, { recursive: true });
    for (const name of names) {
      const sc = SCENARIOS[name];
      const outFile = path.join(OUT_DIR, name + '.md');
      console.log('\n=== scenario ' + name + ' (' + sc.docType + ') → ' + outFile);
      // clean state for this record: no gate, no leftover holds, no old events
      rec.setIntercept(false);
      await rec.drain().catch(() => {});
      rec.reset();
      const docKey = 'rec' + Date.now().toString(36) + Math.random().toString(36).slice(2, 6);
      const { ctx, page } = await openRecordPage(
        browser, monitorSrc, proxy.port, pages.port, sc.docType, docKey, sc.deferAlice
      );
      CURRENT = { rec, page, docType: sc.docType, docKey };
      try {
        await sc.fn();
        await rec.waitQuiescence();
        fs.writeFileSync(outFile, rec.toMarkdown(scenarioInfoLines(sc, docs)));
        console.log('  ✓ wrote ' + outFile + ' (' + rec.events.length + ' events)');
      } catch (e) {
        const dbg = path.join('/tmp', name + '.md');
        fs.writeFileSync(dbg, rec.toMarkdown(['Scenario: ' + sc.info, 'FAILED: ' + e.message]));
        console.log('  ✖ FAILED: ' + e.message);
        console.log('    record so far dumped to ' + dbg);
        await page.screenshot({ path: '/tmp/' + name + '.png' }).catch(() => {});
        failed.push(name);
      }
      await ctx.close();
      // let the closed sessions drain before the next record starts
      await sleep(1500);
    }
    if (failed.length) {
      console.log('\n✖ failed scenarios: ' + failed.join(', '));
      process.exitCode = 1;
    } else {
      console.log('\n✓ all scenarios recorded');
    }
  } finally {
    if (!STAY_ALIVE) {
      await browser.close().catch(() => {});
      proxy.close();
      pages.server.close();
      cleanup();
    } else {
      console.log('→ OO_STAY_ALIVE: keeping browser + container open (Ctrl-C to exit)');
      await new Promise(() => {});
    }
  }
})()
  .then(() => {
    // Everything is written: exit now. Some handles (the document server's
    // keep-alive connection to the page server, the recorder's /ctl reconnect
    // loop) can keep node's event loop alive for a long time after close() —
    // a batch tool has nothing left to wait for.
    if (!STAY_ALIVE) process.exit(process.exitCode || 0);
  })
  .catch((e) => { console.error('ERR', e); cleanup(); process.exit(1); });
