#!/usr/bin/env node
/**
 * prove-me-fucker.js — Automated proof that the monitor panel correctly shows
 * the server→client `auth` event for each editor in the OnlyOffice
 * "Collaborate" scenario.
 *
 * It reproduces run-demo.js's setup (start the MITM proxy, launch Chromium,
 * inject the monitor, switch to Collaborate) and then asserts:
 *
 *   1. After John's editor connects, the panel shows a `recv auth` event
 *      whose user is "John Smith".
 *   2. After clicking "Start Kate's editor", the panel shows a `recv auth`
 *      event whose user is "Kate Cage".
 *
 * Both checks are read from the in-page panel state (window.__OO_MON.events)
 * which is fed by the proxy over the /ctl control channel. The script exits
 * non-zero on failure.
 *
 * Usage:
 *   node prove-me-fucker.js            # headed (visible browser)
 *   HEADLESS=1 node prove-me-fucker.js   # headless (no display)
 *   STAY=1 node prove-me-fucker.js        # keep browser open for inspection
 *   NOSCREENSHOT=1 node prove-me-fucker.js # skip saving screenshots
 */
const path = require('path');
const fs = require('fs');
const { chromium } = require('playwright');
const { createProxy } = require('./proxy');

const MONITOR = path.join(__dirname, 'monitor.js');
const URL = 'https://www.onlyoffice.com/see-it-in-action.aspx';
const STAY_MS = parseInt(process.env.STAY || '0', 10); // 0 = run assertions and exit
const HEADLESS = process.env.HEADLESS === '1';          // default: headed (visible)
const SHOTS_DIR = path.join(__dirname, 'proof-shots');
const NOSCREENSHOT = process.env.NOSCREENSHOT === '1';
let shotSeq = 0;
async function screenshot(page, label) {
  if (NOSCREENSHOT) return;
  try { if (!fs.existsSync(SHOTS_DIR)) fs.mkdirSync(SHOTS_DIR, { recursive: true }); } catch (e) {}
  const file = path.join(SHOTS_DIR, String(++shotSeq).padStart(2, '0') + '-' + label + '.png');
  try { await page.screenshot({ path: file, fullPage: false }); console.log('  📸 screenshot: ' + file); }
  catch (e) { console.log('  (screenshot failed: ' + e.message.slice(0, 80) + ')'); }
}

// Locate a usable Chromium. Prefer an explicit override, then Playwright's
// bundled build, then a few system candidates.
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

function fail(proxy, browser, msg) {
  console.error('\n❌ FAIL: ' + msg);
  if (proxy) try { proxy.close(); } catch (e) {}
  if (browser) try { browser.close(); } catch (e) {}
  process.exit(1);
}
function pass() {
  console.log('\n✅ PASS: panel showed the server→client auth event for both John and Kate.');
}

// Read the events AS SHOWN in the panel DOM — i.e. the rows actually rendered
// in #oo-mon-list. This is the source of truth for what the user sees (not the
// in-memory __OO_MON.events, which can grow even if rendering is broken).
// Returns an array of {dir, type, user} parsed from the rendered row heads.
async function readPanelEvents(page) {
  return await page.evaluate(() => {
    var list = document.getElementById('oo-mon-list');
    console.log("!!!!!!!!!!!!!!!!!", list);
    if (!list) return [];
    var rows = Array.from(list.querySelectorAll('.row'));
    return rows.map(function (r) {
      var d = (r.querySelector('.d') || {}).textContent || '';
      var ed = (r.querySelector('.ed') || {}).textContent || '';
      var ty = (r.querySelector('.ty') || {}).textContent || '';
      var sum = (r.querySelector('.sum') || {}).textContent || '';
      // dir from the row class (recv/send/open/engine/held)
      var dir = /recv|send|open|engine|held/.test(r.className)
        ? (r.className.match(/recv|send|open|engine|held/)[0]) : '?';
      // For msg rows, the type is in .ty and the user is in .ed (the author column).
      // .ed holds the resolved username (what the panel displays).
      return { dir: dir, type: ty, user: ed, summary: sum, icon: d };
    });
  });
}

// Find the first recv `auth` row whose displayed user is `username`.
function findRecvAuthFor(events, username) {
  return events.find(e => e.dir === 'recv' && e.type === 'auth' && e.user === username) || null;
}

(async () => {
  // ---- start the local MITM proxy first -------------------------------
  const proxy = await createProxy({ port: 0, verbose: !!process.env.PROXY_VERBOSE });
  const proxyPort = proxy.port;
  console.log('→ MITM proxy listening on ws://127.0.0.1:' + proxyPort + '/oo  (ctl: /ctl)');

  const monitorSrc = fs.readFileSync(MONITOR, 'utf8');
  const initScript = 'window.__OO_PROXY_PORT = ' + proxyPort + ';\n' + monitorSrc;

  const browser = await chromium.launch({
    executablePath,
    headless: HEADLESS,
    args: [
      '--disable-blink-features=AutomationControlled',
      '--no-sandbox',
      '--allow-running-insecure-content',
      '--disable-web-security',
    ],
  });
  console.log('→ browser launched (mode: ' + (HEADLESS ? 'headless' : 'HEADED — you should see the window') + ')');
  const ctx = await browser.newContext({ viewport: null, bypassCSP: true });
  const page = await ctx.newPage();
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

  // Switch to "Collaborate" so John's editor connects.
  console.log('switching to "Collaborate"');
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
  await screenshot(page, 'collaborate-active');

  // Wait for John's editor frame to appear, then give the panel time to render
  // the auth handshake it received from the server.
  console.log('→ waiting for John\'s editor to connect and the panel to show its auth…');
  try {
    await page.waitForFunction(() =>
      Array.from(document.querySelectorAll('iframe')).some(
        fr => /frameEditorId=document-editor/.test(fr.src || '')
      ), { timeout: 30000 });
  } catch (e) {
    console.log('  (editor frame detection timed out, continuing)');
  }
  // Poll the panel for a recv auth for John Smith (up to ~20s). The auth event
  // arrives shortly after the editor socket opens.
  let johnAuth = null;
  const johnDeadline = Date.now() + 20000;
  while (Date.now() < johnDeadline) {
    await page.waitForTimeout(500);
    const events = await readPanelEvents(page);
    johnAuth = findRecvAuthFor(events, 'John Smith');
    if (johnAuth) break;
  }
  if (!johnAuth) {
    const ev = await readPanelEvents(page);
    console.log('  panel DOM rows:', JSON.stringify(ev.slice(0, 12)));
    const mem = await page.evaluate(() => { const M=window.__OO_MON||{}; return { eventsLen:(M.events||[]).length, listExists:!!document.getElementById('oo-mon-list'), statText: document.getElementById('oo-mon-stat')?document.getElementById('oo-mon-stat').textContent:'na', users:Object.keys(M.users||{}).map(u=>u+':'+M.users[u].checked), mode:M.mode, connected:M.connected }; });
    console.log('  in-memory state:', JSON.stringify(mem, null, 2));
    await screenshot(page, 'john-auth-MISSING');
    fail(proxy, browser, 'no recv `auth` event for "John Smith" appeared in the PANEL DOM within 20s (rows=' + ev.length + ', memEvents=' + mem.eventsLen + ')');
  }
  console.log('  ✓ John\'s recv auth seen:', JSON.stringify(johnAuth));
  await screenshot(page, 'john-auth-received');

  // ---- Now start Kate's editor (it is deferred behind the overlay) -----
  console.log('→ clicking "Start Kate\'s editor" …');
  const startedKate = await page.evaluate(() => {
    // The overlay button lives in Kate's slot (#oo-kate-overlay). Prefer that,
    // fall back to any button whose text matches.
    const ov = document.getElementById('oo-kate-overlay');
    let btn = ov && Array.from(ov.querySelectorAll('button')).find(b => /start kate/i.test(b.textContent || ''));
    if (!btn) {
      btn = Array.from(document.querySelectorAll('button')).find(b => /start kate/i.test(b.textContent || ''));
    }
    if (btn) { btn.click(); return true; }
    return false;
  });
  if (!startedKate) {
    fail(proxy, browser, '"Start Kate\'s editor" button not found / not clickable');
  }

  // Wait for Kate's recv auth.
  let kateAuth = null;
  const kateDeadline = Date.now() + 30000;
  while (Date.now() < kateDeadline) {
    await page.waitForTimeout(500);
    const events = await readPanelEvents(page);
    kateAuth = findRecvAuthFor(events, 'Kate Cage');
    if (kateAuth) break;
  }
  if (!kateAuth) {
    const ev = await readPanelEvents(page);
    console.log('  panel DOM rows (last 12):', JSON.stringify(ev.slice(-12)));
    const mem = await page.evaluate(() => { const M=window.__OO_MON||{}; return { eventsLen:(M.events||[]).length, statText: document.getElementById('oo-mon-stat')?document.getElementById('oo-mon-stat').textContent:'na', users:Object.keys(M.users||{}).map(u=>u+':'+M.users[u].checked) }; });
    console.log('  in-memory state:', JSON.stringify(mem, null, 2));
    await screenshot(page, 'kate-auth-MISSING');
    fail(proxy, browser, 'no recv `auth` event for "Kate Cage" appeared in the PANEL DOM within 30s of starting Kate (rows=' + ev.length + ', memEvents=' + mem.eventsLen + ')');
  }
  console.log('  ✓ Kate\'s recv auth seen:', JSON.stringify(kateAuth));
  await screenshot(page, 'kate-auth-received');

  // (Session-distinctness is no longer checked from the DOM rows since we no
  // longer read sid; the two auth events having different users is enough.)

  // Summary of what the panel showed for auth events.
  const finalEvents = await readPanelEvents(page);
  const authEvents = finalEvents.filter(e => e.type === 'auth');
  console.log('\n=== all `auth` events shown in the panel ===');
  authEvents.forEach(e => console.log('  ' + e.dir + '  user=' + e.user + '  type=' + e.type + '  summary="' + e.summary + '"'));

  // Assert there is exactly ONE panel in the DOM and that it's populated.
  const panelState = await page.evaluate(() => {
    const panels = document.querySelectorAll('#oo-mon');
    const list = document.getElementById('oo-mon-list');
    return { panelCount: panels.length, rows: list ? list.childElementCount : 0 };
  });
  console.log('\n=== panel DOM ===');
  console.log('  #oo-mon panels: ' + panelState.panelCount + '  | rows in list: ' + panelState.rows);
  if (panelState.panelCount !== 1) {
    fail(proxy, browser, 'expected exactly one #oo-mon panel, found ' + panelState.panelCount + ' (duplicate-panel bug)');
  }
  if (panelState.rows === 0) {
    fail(proxy, browser, 'the visible #oo-mon-list is empty (0 rows) even though events were expected');
  }

  await screenshot(page, 'final');
  pass();

  if (STAY_MS > 0) {
    console.log('→ staying for ' + STAY_MS + 'ms then closing');
    await page.waitForTimeout(STAY_MS);
  }
  proxy.close();
  await browser.close();
  process.exit(0);
})().catch(e => {
  console.error('ERR', e);
  process.exit(1);
});
