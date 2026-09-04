// Standalone Playwright script driving two browser tabs through the editics
// co-editing scenarios against the running testbed server + Parsec web client.
//
// Prereqs (already running in this environment):
//   - testbed server: `python make.py rts` (port 6770)
//   - Parsec web client: PARSEC_APP_ENABLE_EDITICS=true TESTBED_SERVER=... npm run dev --port 8080
//
// Run:
//   node tests/e2e/editics_two_tabs.mjs
//
// It captures the editics debug panel entries + console for each tab and prints
// a summary, surfacing any `getLock`/`saveChanges`/`unSaveLock` that looks stuck.

import { chromium } from 'playwright';
import path from 'path';

const BASE = 'http://127.0.0.1:8080';
const TESTBED_SERVER = 'parsec3://127.0.0.1:6770?no_ssl=true';
const DOCX = path.resolve(import.meta.dirname, 'data/imports/document.docx');

// Capture console + editics panel entries per page (across all frames).
function attachCapture(page, label, sink) {
  page.on('console', (msg) => {
    const t = msg.text();
    sink.console.push(`[${label}] ${t}`);
  });
  page.on('pageerror', (e) => sink.console.push(`[${label}] PAGEERROR: ${e.message}`));
}

// Init the testbed (creates a coolorg testbed org) and boot the app, shared
// across tabs via the same config path (same SharedWorker => same org).
async function initApp(page, configPath) {
  await page.addInitScript((cfg) => {
    window.TESTING = true;
    window.TESTING_ENABLE_EDITICS = true;
    window.TESTING_EDITICS_SAVE_TIMEOUT = 1;
    window.TESTING_CRYPTPAD_SERVER = null;
    window.TESTING_CONFIG_PATH = cfg || null;
  }, configPath);
  await page.goto(BASE + '/');
  await page.waitForFunction(() => window.libparsec && window.nextStageHook, null, { timeout: 30000 });
  if (!configPath) {
    configPath = await page.evaluate(async (srv) => {
      const [libparsec, nextStage] = window.nextStageHook();
      const r = await libparsec.testNewTestbed('coolorg', srv);
      if (!r.ok) throw new Error('testbed init failed: ' + JSON.stringify(r.error));
      window.TESTING_CONFIG_PATH = r.value;
      await nextStage(r.value, 'en-US');
      return r.value;
    }, TESTBED_SERVER);
  } else {
    await page.evaluate(async (cfg) => {
      const [, nextStage] = window.nextStageHook();
      await nextStage(cfg, 'en-US');
    }, configPath);
  }
  return configPath;
}

async function loginAndGoToWksp1(page) {
  await page.waitForSelector('.organization-card', { timeout: 30000 });
  const cards = page.locator('.organization-card');
  const count = await cards.count();
  const texts = [];
  for (let i = 0; i < count; i++) texts.push((await cards.nth(i).innerText().catch(() => '')).replace(/\s+/g, ' ').slice(0, 80));
  console.log('org cards:', count, JSON.stringify(texts));
  // Click the Alice card (the testbed org owner).
  let aliceIdx = 0;
  for (let i = 0; i < count; i++) if (texts[i].toLowerCase().includes('alice')) { aliceIdx = i; break; }
  await cards.nth(aliceIdx).click();
  // If already logged in (card shows "Logged in"), it goes straight to workspaces;
  // otherwise the password screen appears.
  const pw = page.locator('#password-input');
  const connected = page.locator('#connected-header');
  await Promise.race([pw.waitFor({ state: 'visible', timeout: 10000 }).then(() => 'pw').catch(() => 'no'), connected.waitFor({ state: 'visible', timeout: 10000 }).then(() => 'connected').catch(() => 'no')]);
  if (await pw.isVisible().catch(() => false)) {
    await pw.locator('input').fill('P@ssw0rd.');
    await page.locator('.login-button').click();
    await connected.waitFor({ state: 'visible', timeout: 30000 });
  }
  await page.locator('.workspaces-container-grid .workspace-card-item').nth(0).click();
  await page.waitForSelector('.folder-container', { timeout: 15000 });
}

async function importDocx(page) {
  const dropZone = page.locator('.folder-container .drop-zone').nth(0);
  await dropZone.waitFor({ state: 'visible', timeout: 10000 });
  const fs = await import('fs');
  const content = fs.readFileSync(DOCX).toString('base64');
  const dt = await page.evaluateHandle(async (filesInfo) => {
    const dt = new DataTransfer();
    for (const f of filesInfo) {
      const blob = await fetch(`data:application/octet-stream;base64,${f.content}`).then((r) => r.blob());
      dt.items.add(new File([blob], f.name, { type: 'application/octet-stream' }));
    }
    return dt;
  }, [{ content, name: 'document.docx' }]);
  await dropZone.dispatchEvent('dragenter');
  await dropZone.dispatchEvent('drop', { dataTransfer: dt });
  await page.waitForSelector('.upload-menu', { timeout: 15000 });
  await page.waitForTimeout(600);
  const closeBtn = page.locator('.upload-menu .menu-header-icons ion-icon').nth(1);
  if (await closeBtn.isVisible().catch(() => false)) await closeBtn.click();
  await page.waitForSelector('.file-list-item, .file-card-item', { timeout: 15000 });
}

async function openFirstDocForEdit(page) {
  const entry = page.locator('.folder-container .file-list-item').nth(0);
  await entry.waitFor({ state: 'visible', timeout: 15000 });
  await entry.click({ button: 'right' });
  const menu = page.locator('.file-context-menu');
  await menu.waitFor({ state: 'visible', timeout: 10000 });
  // The "Edit" action opens the OnlyOffice editor (editics). It's the item
  // whose text contains "Edit"; fall back to the 3rd listitem (see
  // editics_enable.spec).
  const editItem = menu.getByRole('listitem', { name: /Edit/ }).first();
  if (await editItem.isVisible().catch(() => false)) {
    await editItem.click();
  } else {
    await menu.getByRole('listitem').nth(2).click();
  }
  await page.waitForSelector('.file-editor', { timeout: 30000 });
}

// Read the editics debug panel log entries from the OnlyOffice host iframe.
async function readPanel(page) {
  const hostFrame = page.frames().find((f) => f !== page.mainFrame() && (f.url().includes('editics/index') || f.url().includes('host')));
  const frame = hostFrame || page.frames().find((f) => f !== page.mainFrame());
  if (!frame) return [];
  return await frame.evaluate(() => {
    const items = [];
    document.querySelectorAll('#ed-panel .ed-entry').forEach((el) => {
      const flow = el.querySelector('.ed-flow')?.textContent || '';
      const type = el.querySelector('.ed-type')?.textContent || '';
      const note = el.querySelector('.ed-dir')?.textContent || '';
      const oo = el.querySelector('.ed-oo pre')?.textContent || '';
      const net = el.querySelector('.ed-net pre')?.textContent || '';
      items.push({ flow, type, note, oo, net });
    });
    return items;
  }).catch(() => []);
}

async function main() {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext();
  const sink = { console: [] };

  const SINGLE_TAB = process.env.SINGLE_TAB === '1';

  const tabA = await context.newPage();
  attachCapture(tabA, 'A', sink);
  const configPath = await initApp(tabA, null);
  await loginAndGoToWksp1(tabA);
  await importDocx(tabA);
  await openFirstDocForEdit(tabA);
  await tabA.waitForTimeout(8000);

  // The editics client + panel live in the OnlyOffice host iframe
  // (editics/index.html), not the top page. Find it and probe inside.
  const hostFrame = tabA.frames().find((f) => f.url().includes('editics/index') || f.url().includes('host'));
  const anyChild = tabA.frames().filter((f) => f !== tabA.mainFrame());
  console.log('child frames:', anyChild.map((f) => f.url().slice(-70)));
  const probeFrame = hostFrame || anyChild[0];
  let diagA = {};
  if (probeFrame) {
    try {
      diagA = await probeFrame.evaluate(() => ({
        hasMockServer: !!window.__ooMockServer,
        mockType: window.__ooMockServer ? window.__ooMockServer.constructor.name : null,
        indexUser: window.__ooMockServer ? window.__ooMockServer.indexUser : null,
        panelPresent: !!document.querySelector('#ed-panel'),
        editorIframe: !!document.querySelector('iframe[name=frameEditor]'),
      }));
    } catch (e) { diagA = { error: e.message }; }
  }
  console.log('=== Diag A (host iframe) ===', JSON.stringify(diagA));

  let tabB = null;
  if (!SINGLE_TAB) {
    tabB = await context.newPage();
    attachCapture(tabB, 'B', sink);
    await initApp(tabB, configPath);
    await tabB.waitForTimeout(3000);
    const diagBpre = await tabB.evaluate(() => ({
      url: location.href,
      hasOrgCard: !!document.querySelector('.organization-card'),
      hasPw: !!document.querySelector('#password-input'),
      appState: document.querySelector('#app')?.getAttribute('app-state'),
      bodyText: document.body.innerText.slice(0, 200),
    }));
    console.log('=== Diag B pre-login ===', JSON.stringify(diagBpre));
    await loginAndGoToWksp1(tabB);
    await openFirstDocForEdit(tabB);
    await tabB.waitForTimeout(5000);
  }

  // Drive the editor's mock-server `onMessage` directly with synthetic OO
  // events to exercise the editics translation + server round-trip (more
  // reliable than fighting the editor's focus for keyboard input).
  const mockA = await (tabA.frames().find((f) => f !== tabA.mainFrame() && (f.url().includes('editics/index') || f.url().includes('host')))).evaluateHandle(() => window.__ooMockServer).catch(() => null);
  const mockB = tabB ? await (tabB.frames().find((f) => f !== tabB.mainFrame() && (f.url().includes('editics/index') || f.url().includes('host')))).evaluateHandle(() => window.__ooMockServer).catch(() => null) : null;

  // Tab A: insert a line = getLock + saveChanges(releaseLocks), then type = saveChanges.
  if (mockA) {
    await mockA.evaluate((m) => m.onMessage({ type: 'getLock', block: ['P1'] }));
    await tabA.waitForTimeout(500);
    await mockA.evaluate((m) => m.onMessage({ type: 'isSaveLock', syncChangesIndex: 0 }));
    await tabA.waitForTimeout(300);
    await mockA.evaluate((m) => m.onMessage({ type: 'saveChanges', changes: JSON.stringify(['66;frag1']), startSaveChanges: true, endSaveChanges: true, deleteIndex: null, excelAdditionalInfo: null, releaseLocks: true }));
    await tabA.waitForTimeout(1500);
    // Another save (typing).
    await mockA.evaluate((m) => m.onMessage({ type: 'isSaveLock', syncChangesIndex: 1 }));
    await mockA.evaluate((m) => m.onMessage({ type: 'saveChanges', changes: JSON.stringify(['66;frag2','37;frag3']), startSaveChanges: true, endSaveChanges: true, deleteIndex: null, excelAdditionalInfo: null, releaseLocks: false }));
    await tabA.waitForTimeout(1500);
  }
  await tabA.waitForTimeout(2000);
  if (tabB) {
    const editorFrameB = tabB.frame({ name: 'frameEditor' });
    if (editorFrameB) {
      const body = editorFrameB.locator('body').first();
      await body.click({ position: { x: 200, y: 300 } }).catch(() => {});
      await tabB.keyboard.type('Hello from B', { delay: 40 });
    }
    await tabB.waitForTimeout(3000);
  }

  const panelA = await readPanel(tabA);
  const panelB = tabB ? await readPanel(tabB) : [];

  console.log('=== Panel A entries:', panelA.length, '===');
  for (const it of panelA) console.log(`A ${it.flow} ${it.type} ${it.note} | oo=${it.oo.slice(0, 120)} | net=${it.net.slice(0, 240)}`);
  if (tabB) {
    console.log('=== Panel B entries:', panelB.length, '===');
    for (const it of panelB) console.log(`B ${it.flow} ${it.type} ${it.note} | oo=${it.oo.slice(0, 120)} | net=${it.net.slice(0, 240)}`);
  }

  console.log('=== Console ===');
  for (const l of sink.console) console.log(l);

  await browser.close();
}

main().catch(async (e) => {
  console.error('FAILED:', e.message);
  process.exit(1);
});
