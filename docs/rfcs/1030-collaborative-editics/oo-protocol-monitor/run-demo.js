#!/usr/bin/env node

/**
 * run-demo.js — Launch a browser, open the OnlyOffice "see-it-in-action" demo,
 * click "Collaborate", inject the protocol monitor into every frame, and
 * stay open so you can watch the live event log widget in the editor.
 *
 * Usage:
 *   node run-demo.js                 # headless:false, 2 editor instances
 *   HEADLESS=1 node run-demo.js      # headless (UI widget still rendered, no display)
 *   STAY=60000 node run-demo.js       # auto-close after 60s (default: keeps open)
 *
 * Requires: `npm i playwright` and a Chromium (uses the playwright-bundled one).
 */
const path = require('path');
const fs = require('fs');
const { chromium } = require('playwright');

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
  const monitorSrc = fs.readFileSync(MONITOR, 'utf8');

  const browser = await chromium.launch({
    executablePath,
    headless: process.env.HEADLESS === '1',
    args: ['--disable-blink-features=AutomationControlled', '--no-sandbox'],
  });
  // viewport:null => the page uses the real OS window size and resizes
  // live when the window is moved/maximized.
  const ctx = await browser.newContext({
    viewport: null,
  });
  const page = await ctx.newPage();

  // Inject the monitor BEFORE any frame's scripts run (so we beat socket.io).
  await ctx.addInitScript(monitorSrc);

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

  // Report what the monitor captured. Events are forwarded to the TOP frame
  // (the panel lives there); the editor frames only hold socket counts.
  const editors = page.frames().filter(f => /frameEditorId=document-editor/.test(f.url()));
  console.log('\n=== monitor status (top-frame panel) ===');
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
        byType
      };
    });
    console.log('  mode=' + s.mode + '  users=' + JSON.stringify(s.editorUser) + '  chips=' + JSON.stringify(s.users));
    console.log('  events=' + s.events);
    console.log('  ' + JSON.stringify(s.byType, null, 0));
  } catch (e) {
    console.log('  (top-frame eval failed:', e.message.slice(0, 80), ')');
  }
  for (const f of editors) {
    try {
      const sc = await f.evaluate(() => (window.__OO_MON || {}).sockets || 0);
      var who = (f.url().match(/frameEditorId=([^&]*)/) || [, '?'])[1];
      console.log('  [' + who + '] sockets=' + sc);
    } catch (e) {}
  }
  console.log('\nA draggable "OO Protocol Monitor" panel is visible in the top page.');
  console.log('Editors are labelled by username (Anonymous / John Smith / Kate Cage).');
  console.log('ping/pong frames are hidden; the filter auto-follows the active scenario tab.\n');

  if (STAY_MS > 0) {
    console.log('→ staying for ' + STAY_MS + 'ms then closing');
    await page.waitForTimeout(STAY_MS);
    await browser.close();
  } else {
    console.log('→ keeping the browser open. Ctrl-C in this terminal to exit.');
    // keep the node process alive
    await new Promise(() => {});
  }
})().catch(e => { console.error('ERR', e); process.exit(1); });
