// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { MsPage, ProtectionMethod } from '@tests/e2e/helpers/types';

export async function mockAllowedProtectionMethods(page: MsPage, allowedMethods: Array<ProtectionMethod>): Promise<void> {
  await page.evaluate((methods) => {
    (window as any).TESTING_MOCK_ALLOWED_PROTECTION_METHODS = methods;
  }, allowedMethods);
}
