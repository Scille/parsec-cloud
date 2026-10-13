#!/usr/bin/env node
/**
 * run.js — Same protocol monitor as the demo one, but against a *pinned
 * local* OnlyOffice Document Server started on demand with podman/docker,
 * instead of the public onlyoffice.com demo page (which silently moves to
 * whatever OnlyOffice version is current).
 *
 * The image is `onlyoffice/documentserver:9.3.0` (fully qualified
 * `docker.io/onlyoffice/documentserver:9.3.0`): the DocumentServer release
 * of the 9.3.0 train, the one the project's pinned editor client
 * (onlyoffice-editor 9.3.0.140+parsec0) is built from — its web-apps are the
 * exact web-apps submodule of DocumentServer v9.3.0, byte-identical to the
 * one vendored in the client. Docker Hub has no image with editor build 140:
 * `9.3.0`/`9.3.0.1` ship editor 9.3.0 build 138 (the closest published),
 * `9.3.1` ships the 9.3.1 train. The monitor panel shows the exact version
 * the server announces on the wire (license/auth events), e.g.
 * "OO 9.3.0 (build 138)".
 *
 * What it does:
 *   1. `podman|docker run -d --rm docker.io/onlyoffice/documentserver:9.3.0`
 *      (JWT off;
 *      private/loopback IP filtering off, so the container may fetch the
 *      sample document and call the editor callback on this machine), then
 *      waits for the /healthcheck endpoint;
 *   2. serves index.html (the harness page) + the sample document (from
 *      client/tests/e2e/data/imports/) + a stub callbackUrl on a local
 *      HTTP server — three editors (Alice / Bob / Mallory, named and ID'ed
 *      after the Parsec test fixtures so captures are directly usable as
 *      test records) sharing one document key;
 *   3. starts the same WebSocket MITM proxy as the demo mode, injects the
 *      monitor, opens the page, prints the captured status.
 *
 * On exit (Ctrl-C, STAY timeout, error) the container is stopped and — thanks
 * to --rm — destroyed. The image itself stays cached so the next run is fast.
 *
 * Usage:
 *   node run.js                     # headful, keeps open (Ctrl-C to exit)
 *   HEADLESS=1 STAY=60000 node run.js
 *
 * Environment (on top of HEADLESS / STAY / PROXY_VERBOSE):
 *   OO_ENGINE          container engine: podman (default if present), docker
 *   OO_DS_IMAGE        default docker.io/onlyoffice/documentserver:9.3.0
 *   OO_DS_PORT         host port for the document server (default: 8080 if
 *                      free, else any free port)
 *   OO_HOST_ALIAS      container→host alias used in document.url/callbackUrl
 *                      (default: host.containers.internal with podman,
 *                      host.docker.internal with docker)
 *   OO_DS_BOOT_TIMEOUT ms to wait for /healthcheck (default 300000)
 *   OO_DOC             sample document to open, looked up in
 *                      client/tests/e2e/data/imports/ (default: document.docx)
 */
'use strict';

const fs = require('fs');
const http = require('http');
const net = require('net');
const path = require('path');
const { spawnSync } = require('child_process');

const { chromium } = require('playwright');
const { createProxy } = require('./proxy');

const DIR = __dirname;
const MONITOR = path.join(DIR, 'monitor.js');
const PAGE_TEMPLATE = path.join(DIR, 'index.html');
// The harness page opens a Word document; sample documents live with the
// other e2e test data (5 levels up = the repository root).
const SAMPLE_DOC_DIR = path.join(DIR, '..', '..', '..', '..', '..', 'client', 'tests', 'e2e', 'data', 'imports');
const SAMPLE_DOC = path.join(SAMPLE_DOC_DIR, process.env.OO_DOC || 'document.docx');

// Fully qualified on purpose: podman setups without unqualified-search
// registries reject the short form, and docker accepts docker.io/… too.
const IMAGE = process.env.OO_DS_IMAGE || 'docker.io/onlyoffice/documentserver:9.3.0';
const CONTAINER = 'oo-protocol-monitor-ds';
const STAY_MS = parseInt(process.env.STAY || '0', 10); // 0 = keep open
const BOOT_TIMEOUT_MS = parseInt(process.env.OO_DS_BOOT_TIMEOUT || '300000', 10);

// ---- container engine (podman first: that's what the team uses) -----------

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
// Alias for URLs the container must fetch on this machine (document.url +
// callbackUrl). podman maps host.containers.internal out of the box; docker
// (Linux) gets host.docker.internal via --add-host=host.docker.internal:host-gateway.
const HOST_ALIAS = process.env.OO_HOST_ALIAS ||
  (ENGINE === 'podman' ? 'host.containers.internal' : 'host.docker.internal');

function engine(args, opts) {
  return spawnSync(ENGINE, args, opts || {});
}

// ---- small helpers ---------------------------------------------------------

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

function httpGet(url, timeoutMs) {
  return new Promise((resolve, reject) => {
    const req = http.get(url, { timeout: timeoutMs }, (res) => {
      let body = '';
      res.setEncoding('utf8');
      res.on('data', (d) => { body += d; });
      res.on('end', () => resolve({ status: res.statusCode, body: body }));
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

// ---- state + cleanup (stops & destroys the container on any exit) ----------

let containerStarted = false;
let browser = null;
let proxy = null;
let pageServer = null;
let cleaned = false;

function cleanup() {
  if (cleaned) return;
  cleaned = true;
  try { if (browser) browser.close().catch(function () {}); } catch (e) {}
  try { if (proxy) proxy.close(); } catch (e) {}
  try { if (pageServer) pageServer.close(); } catch (e) {}
  if (containerStarted) {
    containerStarted = false;
    console.log('→ stopping the document server container (removed automatically thanks to --rm)…');
    engine(['stop', '-t', '10', CONTAINER], { stdio: 'ignore' });
  }
}
process.on('SIGINT', function () { console.log('\n→ SIGINT'); cleanup(); process.exit(130); });
process.on('SIGTERM', function () { cleanup(); process.exit(143); });

function die(msg) {
  console.error('✖ ' + msg);
  cleanup();
  process.exit(1);
}

// ---- document server container ----------------------------------------------

function containerArgs(dsPort) {
  const args = [
    'run', '-d', '--rm',
    '--name', CONTAINER,
    '-p', '127.0.0.1:' + dsPort + ':80',
    // The harness embeds editors from a plain local page without signing
    // configs — fine for a throwaway local test server.
    '-e', 'JWT_ENABLED=false',
    // The document server itself downloads document.url and posts
    // callbackUrl, which point back at this machine through a private /
    // loopback host alias — filtered out by DS's request filtering since
    // 8.0 unless allowed here.
    '-e', 'ALLOW_META_IP_ADDRESS=true',
    '-e', 'ALLOW_PRIVATE_IP_ADDRESS=true',
  ];
  // docker (Linux) has no host.containers.internal: map host.docker.internal
  // to the bridge gateway. Docker Desktop already defines it; the extra
  // --add-host is harmless there.
  if (ENGINE === 'docker') args.push('--add-host=host.docker.internal:host-gateway');
  args.push(IMAGE);
  return args;
}

async function waitDsReady(dsPort) {
  const t0 = Date.now();
  console.log('→ waiting for the document server healthcheck (first boot takes ~30-60s)…');
  for (;;) {
    const elapsed = Date.now() - t0;
    if (elapsed > BOOT_TIMEOUT_MS) {
      // Dump the logs BEFORE cleanup destroys the container (--rm), so a
      // failed boot is diagnosable without re-running by hand.
      console.log('\n--- last document server logs (podman/docker logs ' + CONTAINER + ') ---');
      engine(['logs', '--tail', '40', CONTAINER], { stdio: 'inherit' });
      console.log('--- end of logs ---\n');
      die('document server healthcheck timed out after ' + Math.round(elapsed / 1000) + 's');
    }
    // --rm removes the container as soon as it exits: a failed inspect or a
    // non-running state means it died during startup.
    const st = engine(['inspect', '-f', '{{.State.Running}}', CONTAINER], { encoding: 'utf8' });
    if (st.status !== 0 || String(st.stdout || '').trim() !== 'true') {
      die('the document server container exited during startup (most common cause: not enough RAM — DS wants ~4 GB).\n' +
          '  to debug: ' + ENGINE + ' run --rm -it --name oo-ds-debug -p 127.0.0.1:' + dsPort + ':80 -e JWT_ENABLED=false ' + IMAGE);
    }
    try {
      const r = await httpGet('http://127.0.0.1:' + dsPort + '/healthcheck', 5000);
      if (r.status === 200 && /true/.test(r.body)) {
        console.log('  document server is up after ' + Math.round(elapsed / 1000) + 's  →  http://127.0.0.1:' + dsPort);
        return;
      }
    } catch (e) { /* not up yet */ }
    await sleep(1500);
  }
}

// ---- harness page server (page + sample document + stub callback) --------

function startPageServer(dsPort, docKey, sampleDoc) {
  const server = http.createServer();
  return new Promise((resolve, reject) => {
    server.on('error', reject);
    // 0.0.0.0: the container fetches /document.docx and /callback through the
    // host alias — docker's bridge gateway cannot reach a 127.0.0.1-bound
    // server (podman's slirp could; 0.0.0.0 covers both engines).
    server.listen(0, '0.0.0.0', () => {
      const port = server.address().port;
      const pageSrc = fs.readFileSync(PAGE_TEMPLATE, 'utf8')
        .replace(/\{\{DS_PORT\}\}/g, String(dsPort))
        .replace(/\{\{PAGE_PORT\}\}/g, String(port))
        .replace(/\{\{DOC_KEY\}\}/g, docKey)
        .replace(/\{\{HOST_ALIAS\}\}/g, HOST_ALIAS)
        .replace(/\{\{IMAGE\}\}/g, IMAGE);
      const docx = fs.readFileSync(sampleDoc);
      server.on('request', (req, res) => {
        const u = (req.url || '').split('?')[0];
        if (req.method === 'GET' && u === '/') {
          res.writeHead(200, { 'content-type': 'text/html; charset=utf-8' });
          res.end(pageSrc);
        } else if (req.method === 'GET' && u === '/document.docx') {
          res.writeHead(200, {
            'content-type': 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
            'content-length': docx.length,
          });
          res.end(docx);
        } else if (req.method === 'POST' && u === '/callback') {
          // Stub for editorConfig.callbackUrl: acknowledge every save/discard
          // notification so the document server logs stay clean.
          req.on('data', function () {});
          req.on('end', function () {
            res.writeHead(200, { 'content-type': 'application/json' });
            res.end('{"error":0}');
          });
        } else {
          res.writeHead(404);
          res.end('not found');
        }
      });
      resolve({ server, port });
    });
  });
}

// ---- main --------------------------------------------------------------------

(async () => {
  console.log('→ container engine: ' + ENGINE + '    image: ' + IMAGE);

  // Fail early if the sample document is missing (e.g. bad OO_DOC), before
  // starting the container.
  if (!fs.existsSync(SAMPLE_DOC)) die('sample document not found: ' + SAMPLE_DOC);

  const dsPort = await pickDsPort();

  // Remove a stale container left by a previous crashed run, then start.
  engine(['rm', '-f', CONTAINER], { stdio: 'ignore' });
  const runArgs = containerArgs(dsPort);
  console.log('→ $ ' + ENGINE + ' ' + runArgs.join(' '));
  const run = engine(runArgs, { stdio: 'inherit' }); // stream pull progress too
  if (run.status !== 0) die(ENGINE + ' run failed (exit ' + run.status + ')');
  containerStarted = true;

  await waitDsReady(dsPort);

  // Sanity: the editor API script must be reachable through the published port.
  try {
    const r = await httpGet('http://127.0.0.1:' + dsPort + '/web-apps/apps/api/documents/api.js', 10000);
    if (r.status !== 200) console.log('  (warning: api.js returned HTTP ' + r.status + ')');
  } catch (e) {
    console.log('  (warning: api.js unreachable: ' + e.message + ')');
  }

  // ---- same MITM proxy + monitor injection as run-demo.js ----------------
  proxy = await createProxy({ port: 0, verbose: !!process.env.PROXY_VERBOSE });
  console.log('→ MITM proxy listening on ws://127.0.0.1:' + proxy.port + '/oo  (ctl: /ctl)');

  // Fresh document key per run → fresh co-editing session on the fresh server.
  const docKey = 'oom' + Date.now().toString(36);
  const pages = await startPageServer(dsPort, docKey, SAMPLE_DOC);
  pageServer = pages.server;
  console.log('→ harness page on http://127.0.0.1:' + pages.port + '/  (doc key ' + docKey + ', server-side URLs via ' + HOST_ALIAS + ')');

  const monitorSrc = fs.readFileSync(MONITOR, 'utf8');
  // Set the proxy port BEFORE the monitor script runs so its WebSocket patch
  // can redirect OO sockets to the proxy.
  const initScript = 'window.__OO_PROXY_PORT = ' + proxy.port + ';\n' + monitorSrc;

  browser = await chromium.launch({
    // Use Playwright's bundled Chromium, this requires `npx playwright install`
    executablePath: undefined,
    headless: process.env.HEADLESS === '1',
    args: [
      '--disable-blink-features=AutomationControlled',
      '--no-sandbox',
      // Keep the same relaxed flags as run-demo.js (the monitor probes the
      // proxy cross-origin with a sync XHR; bypassCSP covers CSP only).
      '--allow-running-insecure-content',
      '--disable-web-security',
    ],
  });
  const ctx = await browser.newContext({
    viewport: null,
    bypassCSP: true,
  });
  const page = await ctx.newPage();

  // Inject the monitor BEFORE any frame's scripts run (so we beat socket.io).
  await ctx.addInitScript(initScript);

  page.on('console', (m) => {
    const t = m.text();
    if (/OO Monitor|onlyoffice protocol/i.test(t)) console.log('[page]', t);
  });
  page.on('pageerror', (e) => console.log('[pageerror]', e.message.slice(0, 160)));

  const pageUrl = 'http://127.0.0.1:' + pages.port + '/';
  console.log('→ opening ' + pageUrl);
  await page.goto(pageUrl, { waitUntil: 'domcontentloaded', timeout: 60000 });

  // wait for Alice's editor iframe to load its app (Bob's and Mallory's are
  // deferred behind the panel's "Start …'s editor" overlays, like the demo
  // page's Collaborate tab)
  console.log('→ waiting for the editor frames to connect…');
  try {
    await page.waitForFunction(() =>
      Array.from(document.querySelectorAll('iframe')).some(
        (fr) => /frameEditorId=document-editor/.test(fr.src || '')
      ), { timeout: 60000 });
  } catch (e) { console.log('  (editor frame detection timed out, continuing)'); }
  await page.waitForTimeout(8000);

  // Report what the monitor captured. The proxy is the source of truth now;
  // the top-frame panel just renders what the proxy pushes over /ctl.
  console.log('\n=== monitor status (proxy + top-frame panel) ===');
  try {
    const s = await page.evaluate(() => {
      var M = window.__OO_MON || {};
      var byType = {}, byUser = {};
      (M.events || []).forEach(function (e) {
        if (e.kind === 'msg') {
          var u = (e.editor && M.editorUser[e.editor]) || e.editor || '?';
          var k = u + ' ' + e.dir + ':' + e.meta.type;
          byType[k] = (byType[k] || 0) + 1;
        }
      });
      return {
        events: (M.events || []).length,
        ooVersion: M.ooVersion
          ? M.ooVersion + (M.ooBuild != null ? ' (build ' + M.ooBuild + ')' : '')
          : null,
        editorUser: M.editorUser,
        users: Object.keys(M.users || {}),
        manualFlow: M.intercept,
        connected: M.connected,
        byType
      };
    });
    console.log('  manualFlow=' + s.manualFlow + '  proxy=' + (s.connected ? 'connected' : 'disconnected'));
    console.log('  OnlyOffice version=' + (s.ooVersion || 'unknown (no license/auth event seen yet)'));
    console.log('  users=' + JSON.stringify(s.editorUser) + '  chips=' + JSON.stringify(s.users));
    console.log('  events=' + s.events);
    console.log('  ' + JSON.stringify(s.byType, null, 0));
  } catch (e) {
    console.log('  (top-frame eval failed:', e.message.slice(0, 80), ')');
  }
  console.log('  proxy: sessions=' + proxy.state.sessions.size + '  holds=' + proxy.state.holds.size + '  manualFlow=' + proxy.state.intercept);

  console.log('\nA draggable "OO Protocol Monitor" panel is visible in the page.');
  console.log("Alice's editor starts connected; Bob's and Mallory's are held behind their");
  console.log('"Start …\'s editor" overlays (click them to watch their late-join).');
  console.log('Click "auto-flow" to switch to manual-flow: frames are held and each row shows a');
  console.log('"send" button to release it. Use "Re-start Alice\'s editor in manual-flow" to reload the');
  console.log('page with manual-flow armed and step through Alice\'s handshake frame by frame.\n');
  console.log('Document server: ' + IMAGE + ' on http://127.0.0.1:' + dsPort + ' — stopped & removed when this command exits.');

  if (STAY_MS > 0) {
    console.log('→ staying for ' + STAY_MS + 'ms then closing');
    await page.waitForTimeout(STAY_MS);
    cleanup();
  } else {
    console.log('→ keeping the browser open. Ctrl-C in this terminal to exit.');
    // keep the node process alive
    await new Promise(() => {});
  }
})().catch((e) => { console.error('ERR', e); cleanup(); process.exit(1); });
