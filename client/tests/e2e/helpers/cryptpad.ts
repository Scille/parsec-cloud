// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect } from '@playwright/test';
import { MsPage } from '@tests/e2e/helpers/types';

// cspell:disable-next-line
export const CRYPTPAD_SERVER = 'cryptpad-dev.parsec.cloud';

interface MockCryptpadOptions {
  timeout?: boolean;
  httpErrorCode?: number;
  failInit?: boolean;
  failOpen?: boolean;
  customInitFunction?: string;
  customOpenFunction?: string;
}

const FRAME_CONTENT = `
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <script src="./frame.js" defer></script>
</head>
<body>
  <div id="editor-container"></div>
</body>
</html>
<style>
  body, html {
    margin: 0;
    padding: 0;
    border: none;
    height: 100%;
    width: 100%;
    overflow: hidden;
  }
  #editor-container {
    height: 100%;
    width: 100%;
    border: none;
  }
</style>
`;

const DEFAULT_INIT_FUNCTION = `
  sendToParent({ command: 'editics-init-result', success: true });
`;

const DEFAULT_OPEN_FUNCTION = `
  sendToParent({ command: 'editics-open-result', success: true });
  document.getElementById('editor-container').innerText = data.documentName;
  setTimeout(() => {
    sendToParent({ command: 'editics-event', event: 'ready' });
  }, 100);
`;

function generateScriptContent(initFunction: string = DEFAULT_INIT_FUNCTION, openFunction: string = DEFAULT_OPEN_FUNCTION): string {
  return `
  (function () {
    let origin = undefined;
    let lastSavedContent = null;

    function sendToParent(data) {
      if (data.command === 'editics-event' && data.event === 'save') {
        lastSavedContent = data.documentContent;
      }
      window.parent.postMessage(data, origin);
    }

    function initialize() {
      ${initFunction}
    }

    function openFile(data) {
      lastSavedContent = data.documentContent;
      ${openFunction}
    }

    window.addEventListener('message', (event) => {
      if (event.data.command === 'editics-hello') {
        origin = event.origin;
        return;
      }
      switch (event.data.command) {
        case 'editics-init': {
          initialize();
          break;
        }
        case 'editics-open': {
          openFile(event.data);
          break;
        }
        case 'editics-save': {
          if (lastSavedContent) {
            if (!(lastSavedContent instanceof Blob)) {
              lastSavedContent = new Blob([lastSavedContent], { type: 'application/octet-stream' });
            }
            sendToParent({ command: 'editics-event', event: 'save', documentContent: lastSavedContent });
          }
          break;
        }
      }
    });

    // Signaling everyone that we're ready
    window.parent.postMessage({ command: 'editics-ready' }, '*');
  })();
  `;
}

export async function mockCryptpadServer(page: MsPage, opts?: MockCryptpadOptions): Promise<void> {
  await page.route(`https://${CRYPTPAD_SERVER}/**`, async (route) => {
    if (opts?.timeout) {
      await route.abort('timedout');
    } else if (opts?.httpErrorCode) {
      await route.fulfill({ status: opts.httpErrorCode });
    } else if (route.request().url().endsWith('/frame.html')) {
      await route.fulfill({ status: 200, body: FRAME_CONTENT });
    } else if (route.request().url().endsWith('/frame.js')) {
      let initFunction = DEFAULT_INIT_FUNCTION;
      let openFunction = DEFAULT_OPEN_FUNCTION;

      if (opts?.failInit && opts?.customInitFunction) {
        throw new Error('Both `failInit` and `customInitFunction` set, use only one');
      }
      if (opts?.failOpen && opts?.customOpenFunction) {
        throw new Error('Both `failOpen` and `customOpenFunction` set, use only one');
      }

      if (opts?.failInit) {
        initFunction = "sendToParent({ command: 'editics-init-result', success: false, error: 'init-failed', details: 'It failed.' });";
      }
      if (opts?.failOpen) {
        openFunction = "sendToParent({ command: 'editics-open-result', success: false, error: 'open-failed', details: 'It failed.' });";
      }
      if (opts?.customInitFunction) {
        initFunction = opts?.customInitFunction;
      }
      if (opts?.customOpenFunction) {
        openFunction = opts?.customOpenFunction;
      }

      await route.fulfill({ status: 200, body: generateScriptContent(initFunction, openFunction) });
    } else {
      console.error(`Route not mocked: '${route.request().url()}'`);
    }
  });
}

export async function waitUntilSaved(page: MsPage, timeout = 10000): Promise<void> {
  await expect(page.locator('#unsaved-changes')).toBeHidden();
  await expect(page.locator('#saved-changes')).toBeVisible({ timeout: timeout });
}
