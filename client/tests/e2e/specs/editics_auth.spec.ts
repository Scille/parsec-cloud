// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { TestInfo } from '@playwright/test';
import { expect, importDefaultFiles, ImportDocuments, msTest } from '@tests/e2e/helpers';

// Minimal client-side smoke test for the editics step-0 wiring (RFC 1030,
// `todo/step_0.md` §9.3). It opens a document in edit mode and asserts the
// OnlyOffice editor host frame is mounted, which validates that the editics
// config building (server addr, workspace id, vlob id/version, device id) and
// the `editics` option plumbing through `onlyoffice.ts` / `editics/index.html`
// do not break the open flow. The vendored OnlyOffice editor itself is not
// required for this test; the server-side transport (SSE + RPC, `auth` and
// `connectState`) is covered by `server/tests/test_editics.py`.

msTest.describe(() => {
  msTest.use({
    documentsOptions: {
      empty: true,
    },
  });

  msTest('Editics editor host frame mounts on document open', async ({ parsecEditics }, testInfo: TestInfo) => {
    await importDefaultFiles(parsecEditics, testInfo, ImportDocuments.Docx, false);

    const entry = parsecEditics.locator('.folder-container').locator('.file-list-item').nth(0);
    await entry.click({ button: 'right' });
    await parsecEditics.locator('.file-context-menu').getByRole('listitem', { name: 'Edit' }).click();

    // The FileEditor component mounts the OnlyOffice host iframe.
    await expect(parsecEditics.locator('.file-editor')).toBeVisible({ timeout: 15000 });
  });
});
