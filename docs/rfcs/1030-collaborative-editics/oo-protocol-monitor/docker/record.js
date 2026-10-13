#!/usr/bin/env node
/**
 * record.js — automated scenario recorder for the OnlyOffice co-editing
 * protocol monitor.
 *
 * Where `run.js` opens the harness page for a *human* to craft scenarios with
 * the monitor panel, this script replays a fixed list of scenarios (auth,
 * cursor moves, chat messages, document edits — serial / concurrent /
 * while-another-client-is-authenticating, across the docx / xlsx / pptx
 * sample documents) with Playwright driving the editors, and writes one
 * record file per scenario to `server/tests/editics/records/new/`.
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

async function openRecordPage(browser, monitorSrc, proxyPort, pageServerPort, docType, docKey) {
  const ctx = await browser.newContext({ viewport: { width: 1920, height: 1080 }, bypassCSP: true });
  await ctx.addInitScript('window.__OO_PROXY_PORT = ' + proxyPort + ';\n' + monitorSrc);
  const page = await ctx.newPage();
  page.on('pageerror', (e) => console.log('  [pageerror]', String(e.message).slice(0, 120)));
  const url = 'http://127.0.0.1:' + pageServerPort + '/?doc=' + docType + '&key=' + docKey;
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

async function startEditor(page, who) {
  await page.evaluate((id) => {
    const MON = window.__OO_MON;
    if (MON && MON.deferred[id]) MON.startDeferred(id);
    else throw new Error('editor ' + id + ' is not deferred (already started?)');
  }, EDITORS[who]);
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
// Scenario table
// ---------------------------------------------------------------------------

const SCENARIOS = {};
function scenario(name, docType, info, fn) {
  SCENARIOS[name] = { name, docType, info, fn };
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
    for (const n of Object.keys(SCENARIOS)) console.log(n + '  [' + SCENARIOS[n].docType + ']  ' + SCENARIOS[n].info);
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
      const { ctx, page } = await openRecordPage(browser, monitorSrc, proxy.port, pages.port, sc.docType, docKey);
      CURRENT = { rec, page, docType: sc.docType, docKey };
      try {
        await sc.fn();
        await rec.waitQuiescence();
        const info = [
          'Document: ' + docs[sc.docType].file + ' (' + sc.docType + ')',
          'Scenario: ' + sc.info,
        ];
        fs.writeFileSync(outFile, rec.toMarkdown(info));
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
