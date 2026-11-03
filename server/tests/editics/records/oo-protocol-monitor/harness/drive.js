#!/usr/bin/env node
/**
 * drive.js — scripted record generation against an ALREADY-RUNNING document
 * server (unlike run.js, which starts its own container).
 *
 * This exists to capture the protocol scenarios that the interactive harness
 * cannot easily reproduce by hand (e.g. a save big enough to be split into
 * several `saveChanges` chunks), and to make record generation reproducible.
 *
 * It reuses run.js's page server + monitor injection, but skips the container
 * management: point it at a document server already listening on
 * `OO_DS_PORT` (default 8080) and it serves the harness page on
 * `OO_DS_PORT + 1` like run.js does.
 *
 * Usage:
 *   node harness/drive.js <scenario> [--doc=word|cell|slide] [--out=FILE]
 *
 * The scenario is a JS module exporting `async function run(ctx)`; see the
 * `scenarios/` folder. `ctx` exposes the Playwright page and the monitor
 * helpers.
 */
'use strict';

const fs = require('fs');
const http = require('http');
const path = require('path');

const { chromium } = require('playwright');
const { createProxy } = require('./proxy');

const DIR = __dirname;
const MONITOR = path.join(DIR, 'monitor.js');
const PAGE_TEMPLATE = path.join(DIR, 'index.html');
const SAMPLE_DOC_DIR = path.join(
  DIR, '..', '..', '..', '..', '..', '..', 'client', 'tests', 'e2e', 'data', 'imports'
);

const DS_PORT = parseInt(process.env.OO_DS_PORT || '8080', 10);
// run.js uses DS_PORT + 1, which may already be taken by an interactive
// harness left running; allow overriding so both can coexist.
const PAGE_PORT = parseInt(process.env.OO_PAGE_PORT || String(DS_PORT + 1), 10);
const HOST_ALIAS = process.env.OO_HOST_ALIAS || 'host.containers.internal';

const DOC_TYPES = {
  word: { file: 'document.docx', documentType: 'word', fileType: 'docx', label: 'Word (.docx)' },
  cell: { file: 'spreadsheet.xlsx', documentType: 'cell', fileType: 'xlsx', label: 'Spreadsheet (.xlsx)' },
  slide: { file: 'presentation.pptx', documentType: 'slide', fileType: 'pptx', label: 'Presentation (.pptx)' },
};

const MIME = {
  docx: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
  xlsx: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
  pptx: 'application/vnd.openxmlformats-officedocument.presentationml.presentation',
};

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

function httpGet(url, timeoutMs) {
  return new Promise((resolve, reject) => {
    const req = http.get(url, { timeout: timeoutMs }, (res) => {
      let body = '';
      res.setEncoding('utf8');
      res.on('data', (d) => (body += d));
      res.on('end', () => resolve({ status: res.statusCode, body }));
    });
    req.on('timeout', () => req.destroy(new Error('timeout')));
    req.on('error', reject);
  });
}

// ---- page server (same contract as run.js) ---------------------------------

function startPageServer(dsPort, docs, docKeyBase) {
  const server = http.createServer();
  return new Promise((resolve, reject) => {
    server.on('error', reject);
    server.listen(PAGE_PORT, '0.0.0.0', () => {
      const port = PAGE_PORT;
      const template = fs.readFileSync(PAGE_TEMPLATE, 'utf8');
      const contents = {};
      for (const type in docs) contents[docs[type].file] = fs.readFileSync(docs[type].path);

      function docButtons(type) {
        return Object.keys(docs)
          .map(
            (t) =>
              '<button class="doc-button' +
              (t === type ? ' active' : '') +
              '" data-doc="' +
              t +
              '" title="Open ' +
              docs[t].file +
              '">' +
              docs[t].label +
              '</button>'
          )
          .join('');
      }
      function renderPage(type) {
        const doc = docs[type];
        return template
          .replace(/\{\{DS_PORT\}\}/g, String(dsPort))
          .replace(/\{\{PAGE_PORT\}\}/g, String(port))
          .replace(/\{\{DOC_KEY\}\}/g, docKeyBase + '-' + type)
          .replace(/\{\{HOST_ALIAS\}\}/g, HOST_ALIAS)
          .replace(/\{\{IMAGE\}\}/g, 'external document server')
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
          const type = docs[u.searchParams.get('doc')] ? u.searchParams.get('doc') : 'word';
          res.writeHead(200, { 'content-type': 'text/html; charset=utf-8' });
          res.end(renderPage(type));
        } else if (req.method === 'GET' && contents[p.slice(1)]) {
          const doc = Object.values(docs).find((d) => d.file === p.slice(1));
          const body = contents[p.slice(1)];
          res.writeHead(200, { 'content-type': MIME[doc.fileType], 'content-length': body.length });
          res.end(body);
        } else if (req.method === 'POST' && p === '/callback') {
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

// ---- main ------------------------------------------------------------------

function parseArgs(argv) {
  const out = { scenario: null, doc: 'word', outFile: null };
  for (const a of argv) {
    if (a.startsWith('--doc=')) out.doc = a.slice(6);
    else if (a.startsWith('--out=')) out.outFile = a.slice(6);
    else if (!a.startsWith('--')) out.scenario = a;
  }
  return out;
}

(async () => {
  const args = parseArgs(process.argv.slice(2));
  if (!args.scenario) {
    console.error('usage: node harness/drive.js <scenario.js> [--doc=word|cell|slide] [--out=FILE]');
    process.exit(1);
  }
  if (!DOC_TYPES[args.doc]) {
    console.error('unknown --doc=' + args.doc);
    process.exit(1);
  }

  // 1) The document server must already be up.
  try {
    const r = await httpGet('http://127.0.0.1:' + DS_PORT + '/healthcheck', 5000);
    if (r.status !== 200) throw new Error('HTTP ' + r.status);
  } catch (e) {
    console.error('✖ no document server on http://127.0.0.1:' + DS_PORT + ' (' + e.message + ')');
    process.exit(1);
  }
  console.log('→ reusing the document server on http://127.0.0.1:' + DS_PORT);

  // 2) Page server + proxy + browser.
  const docs = {};
  for (const type of Object.keys(DOC_TYPES)) {
    const doc = Object.assign({}, DOC_TYPES[type], {
      path: path.join(SAMPLE_DOC_DIR, DOC_TYPES[type].file),
    });
    if (fs.existsSync(doc.path)) docs[type] = doc;
  }
  if (!docs[args.doc]) {
    console.error('✖ sample document missing: ' + path.join(SAMPLE_DOC_DIR, DOC_TYPES[args.doc].file));
    process.exit(1);
  }

  const docKeyBase = 'drive' + Date.now().toString(36);
  const pages = await startPageServer(DS_PORT, docs, docKeyBase);
  const proxy = await createProxy({ port: 0, verbose: !!process.env.PROXY_VERBOSE });
  console.log('→ proxy ws://127.0.0.1:' + proxy.port + '/oo   page http://127.0.0.1:' + pages.port + '/');

  const monitorSrc = fs.readFileSync(MONITOR, 'utf8');
  const initScript = 'window.__OO_PROXY_PORT = ' + proxy.port + ';\n' + monitorSrc;

  const browser = await chromium.launch({
    headless: process.env.HEADLESS !== '0',
    args: [
      '--disable-blink-features=AutomationControlled',
      '--no-sandbox',
      '--allow-running-insecure-content',
      '--disable-web-security',
    ],
  });
  const ctx = await browser.newContext({ viewport: null, bypassCSP: true });
  const page = await ctx.newPage();
  await ctx.addInitScript(initScript);
  page.on('pageerror', (e) => console.log('[pageerror]', e.message.slice(0, 160)));

  const pageUrl = 'http://127.0.0.1:' + pages.port + '/?doc=' + args.doc;
  console.log('→ opening ' + pageUrl);
  await page.goto(pageUrl, { waitUntil: 'domcontentloaded', timeout: 60000 });

  // Wait for Alice's editor frame to connect.
  try {
    await page.waitForFunction(
      () =>
        Array.from(document.querySelectorAll('iframe')).some((fr) =>
          /frameEditorId=document-editor/.test(fr.src || '')
        ),
      { timeout: 60000 }
    );
  } catch (e) {
    console.log('  (editor frame detection timed out, continuing)');
  }
  await page.waitForTimeout(8000);

  // Report the chunking setting the document server advertises: the
  // multi-chunk save scenario only produces `savePartChanges` when
  // `websocketMaxPayloadSize` is small (run.js lowers it in the
  // document server config).
  try {
    const settings = await page.evaluate(() => {
      const a = window.__OO_MON.events.find(
        (e) => e.meta && e.meta.type === 'auth' && e.meta.settings
      );
      return a ? a.meta.settings.websocketMaxPayloadSize : null;
    });
    console.log('→ document server websocketMaxPayloadSize: ' + settings);
  } catch (e) {
    /* non-fatal */
  }

  // 3) Run the scenario.
  // Accept an absolute path, a path relative to the cwd, or a bare scenario
  // name (resolved against the `scenarios/` folder).
  const scenarioPath = path.isAbsolute(args.scenario)
    ? args.scenario
    : fs.existsSync(path.resolve(args.scenario))
      ? path.resolve(args.scenario)
      : path.join(DIR, '..', 'scenarios', args.scenario);
  const scenario = require(scenarioPath);

  const helpers = {
    page,
    sleep,
    // Editor ids as the harness knows them.
    EDITORS: {
      Alice: 'document-editor-1',
      Bob: 'document-editor-2',
      Mallory: 'document-editor-3',
    },
    // Start a deferred editor (Bob/Mallory are deferred behind an overlay).
    async startEditor(who, { view = false } = {}) {
      const id = helpers.EDITORS[who];
      await page.evaluate(
        ([id, view]) => {
          if (window.__OO_MON && window.__OO_MON.startDeferred) {
            window.__OO_MON.startDeferred(id, view);
          }
        },
        [id, view]
      );
      await page.waitForTimeout(6000);
    },
    // Frame of a given editor (to drive the editor's own API).
    editorFrame(who) {
      const id = helpers.EDITORS[who];
      return page.frames().find((f) => f.url().includes('frameEditorId=' + id));
    },
    async waitQuiescence(ms = 1500) {
      await page.waitForTimeout(ms);
    },
    // Export the monitor's record text exactly like the `copy` button (the
    // monitor exposes its own `exportText`, so the formatting cannot drift).
    async exportRecord() {
      return await page.evaluate(() => {
        const mon = window.__OO_MON;
        if (!mon) throw new Error('no monitor on the page');
        if (typeof mon.exportText !== 'function') {
          throw new Error('monitor has no exportText (stale monitor.js?)');
        }
        return mon.exportText();
      });
    },
  };

  let exitCode = 0;
  try {
    await scenario.run(helpers);
    const text = await helpers.exportRecord();
    if (args.outFile) {
      fs.writeFileSync(args.outFile, text);
      console.log('→ record written to ' + args.outFile + ' (' + text.length + ' bytes)');
    } else {
      process.stdout.write('\n===== RECORD START =====\n' + text + '\n===== RECORD END =====\n');
    }
  } catch (e) {
    console.error('✖ scenario failed: ' + (e && e.stack ? e.stack : e));
    exitCode = 1;
  } finally {
    await browser.close().catch(() => {});
    proxy.close();
    pages.server.close();
  }
  process.exit(exitCode);
})();
