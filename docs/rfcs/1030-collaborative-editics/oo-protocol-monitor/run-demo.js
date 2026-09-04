#!/usr/bin/env node
/**
 * run-demo.js — Launch a browser, open the OnlyOffice "see-it-in-action" demo,
 * click "Collaborate", inject the protocol monitor into every frame, and
 * stay open so you can watch the live event log widget in the editor.
 *
 * This variant runs a LOCAL WebSocket man-in-the-middle (proxy.js) and tells
 * the in-page monitor (via window.__OO_PROXY_PORT) to redirect OnlyOffice's
 * co-editing WebSockets through it. The proxy decodes every frame in Node and
 * can HOLD any non-noise frame until you release it from the panel — the only
 * way to gate RECV (the bytes can't reach socket.io until the proxy forwards).
 *
 * Usage:
 *   node run-demo.js                 # headless:false, 2 editor instances
 *   HEADLESS=1 node run-demo.js      # headless (UI widget still rendered, no display)
 *   STAY=60000 node run-demo.js       # auto-close after 60s (default: keeps open)
 *
 * Requires: `npm i playwright ws` and a Chromium (uses the playwright-bundled one).
 */
const path = require('path');
const fs = require('fs');
const { chromium } = require('playwright');
const { createProxy } = require('./proxy');

const MONITOR = path.join(__dirname, 'monitor.js');
const URL = 'https://www.onlyoffice.com/see-it-in-action.aspx';
const STAY_MS = parseInt(process.env.STAY || '0', 10); // 0 = keep open

// Locate a usable Chromium. Try Playwright's bundled build first, then a
// system Chromium so we don't require `npx playwright install`.
function findChromium() {
  if (process.env.CHROMIUM_PATH) return process.env.CHROMIUM_PATH;
  const cands = [
    '/snap/chromium/current/usr/lib/chromium-browser/chrome',
    '/usr/bin/chromium',
    '/usr/bin/chromium-browser',
    '/usr/bin/google-chrome',
    '/usr/bin/google-chrome-stable',
    '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
  ];
  for (const p of cands) {
    try { if (fs.existsSync(p) && fs.statSync(p).isFile()) return p; } catch (e) {}
  }
  return undefined; // fall back to Playwright's bundled browser
}
const executablePath = findChromium();

(async () => {
  // ---- start the local MITM proxy first -------------------------------
  const proxy = await createProxy({ port: 0, verbose: !!process.env.PROXY_VERBOSE });
  const proxyPort = proxy.port;
  console.log('→ MITM proxy listening on ws://127.0.0.1:' + proxyPort + '/oo  (ctl: /ctl)');

  const monitorSrc = fs.readFileSync(MONITOR, 'utf8');
  // Set the proxy port BEFORE the monitor script runs so its WebSocket patch
  // can redirect OO sockets to the proxy.
  const initScript = 'window.__OO_PROXY_PORT = ' + proxyPort + ';\n' + monitorSrc;

  const browser = await chromium.launch({
    executablePath,
    headless: process.env.HEADLESS === '1',
    args: [
      '--disable-blink-features=AutomationControlled',
      '--no-sandbox',
      // Allow ws://127.0.0.1 and http://127.0.0.1 from the https OnlyOffice
      // page (mixed content). bypassCSP covers Content-Security-Policy but NOT
      // mixed-content blocking, which Chrome enforces separately.
      '--allow-running-insecure-content',
      '--disable-web-security',
    ],
  });
  // viewport:null => the page uses the real OS window size and resizes
  // live when the window is moved/maximized.
  const ctx = await browser.newContext({
    viewport: null,
    bypassCSP: true, // let ws://127.0.0.1 control+traffic sockets connect from an https page
  });
  const page = await ctx.newPage();

  // Inject the monitor BEFORE any frame's scripts run (so we beat socket.io).
  await ctx.addInitScript(initScript);

  page.on('console', m => {
    const t = m.text();
    if (/OO Monitor|onlyoffice protocol/i.test(t)) console.log('[page]', t);
  });
  page.on('pageerror', e => console.log('[pageerror]', e.message.slice(0, 160)));

  console.log('→ opening', URL);
  await page.goto(URL, { waitUntil: 'domcontentloaded', timeout: 60000 });

  // Dismiss any cookie/consent banner so it does not intercept tab clicks.
  try {
    const accept = page.locator('button', { hasText: /^(Accept all|Accept All|Agree|I agree|Accept|OK)$/i }).first();
    await accept.click({ timeout: 6000 });
    await page.waitForTimeout(500);
  } catch (e) { /* no banner or not dismissible */ }

  // The page opens on "Edit DOCX" (a solo "Anonymous" scenario). Switch to
  // "Collaborate" so the two John & Kate editors connect. Use a normal click
  // first (React needs the real event to activate the tab), then retry with
  // force if the tab still isn't active.
  console.log('switching to "Collaborate"');
  // Dispatch the click via page.evaluate(btn.click()) rather than Playwright's
  // locator.click(): the monitor panel (bottom-right) would otherwise intercept
  // the hit-tested click coordinates; a direct JS .click() activates the React
  // tab reliably regardless of overlays.
  const switched = await page.evaluate(() => {
    const btn = Array.from(document.querySelectorAll('button[class*="actions-tab-button"]'))
      .find(b => /^collaborate$/i.test((b.textContent || '').trim()));
    if (btn) btn.click();
    return !!btn;
  });
  if (!switched) console.log('  (Collaborate tab button not found)');
  try {
    await page.waitForFunction(() => {
      const btn = Array.from(document.querySelectorAll('button[class*="actions-tab-button"]'))
        .find(b => /active/i.test(b.className));
      return btn && /collaborate/i.test(btn.textContent || '');
    }, { timeout: 10000 });
    console.log('  Collaborate is active');
  } catch (e) {
    console.log('  (Collaborate tab did not become active; continuing)');
  }

  // wait for the two editor iframes to load their app
  console.log('→ waiting for editor frames to connect…');
  try {
    await page.waitForFunction(() =>
      Array.from(document.querySelectorAll('iframe')).some(
        fr => /frameEditorId=document-editor/.test(fr.src || '')
      ), { timeout: 30000 });
  } catch (e) {
    console.log('  (editor frame detection timed out, continuing)');
  }
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
        editorUser: M.editorUser,
        users: Object.keys(M.users || {}),
        mode: M.mode,
        manualFlow: M.intercept,
        connected: M.connected,
        byType
      };
    });
    console.log('  mode=' + s.mode + '  manualFlow=' + s.manualFlow + '  proxy=' + (s.connected ? 'connected' : 'disconnected'));
    console.log('  users=' + JSON.stringify(s.editorUser) + '  chips=' + JSON.stringify(s.users));
    console.log('  events=' + s.events);
    console.log('  ' + JSON.stringify(s.byType, null, 0));
  } catch (e) {
    console.log('  (top-frame eval failed:', e.message.slice(0, 80), ')');
  }
  console.log('  proxy: sessions=' + proxy.state.sessions.size + '  holds=' + proxy.state.holds.size + '  manualFlow=' + proxy.state.intercept);

  console.log('\nA draggable "OO Protocol Monitor" panel is visible in the top page.');
  console.log('Editors are labelled by username (Anonymous / John Smith / Kate Cage).');
  console.log('ping/pong frames are hidden; the filter auto-follows the active scenario tab.');
  console.log('Click "auto-flow" to switch to manual-flow: frames are held and each row shows a');
  console.log('"send" button to release it. Use "Re-start John\'s editor in manual-flow" (under');
  console.log('"Start Kate\'s editor") to step through John\'s handshake frame by frame.\n');

  if (STAY_MS > 0) {
    console.log('→ staying for ' + STAY_MS + 'ms then closing');
    await page.waitForTimeout(STAY_MS);
    proxy.close();
    await browser.close();
  } else {
    console.log('→ keeping the browser open. Ctrl-C in this terminal to exit.');
    // keep the node process alive
    await new Promise(() => {});
  }
})().catch(e => { console.error('ERR', e); process.exit(1); });
