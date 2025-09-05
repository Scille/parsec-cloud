// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { MsPage } from '@tests/e2e/helpers/types';

// cspell:disable-next-line
export const CRYPTPAD_SERVER = 'centakina.ddns.net';

interface MockCryptpadOptions {
  timeout?: boolean;
  httpErrorCode?: number;
}

export async function mockCryptpadServer(page: MsPage, opts?: MockCryptpadOptions): Promise<void> {
  await page.route(`https://${CRYPTPAD_SERVER}/**`, async (route) => {
    if (opts?.timeout) {
      await route.abort('timedout');
    } else if (opts?.httpErrorCode) {
      await route.fulfill({ status: opts.httpErrorCode });
    } else {
      await route.continue();
    }
  });
}
