// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { MsPage } from '@tests/e2e/helpers/types';

export interface LibParsecFunction {
  name: string;
  result: { ok: true; value: any } | { ok: false; error: { tag: string; error: string } };
}

export async function mockLibParsec(page: MsPage, functions: Array<LibParsecFunction>): Promise<void> {
  await page.evaluate((functions) => {
    for (const func of functions) {
      console.log(`Mocking '${func.name}'`);
      (window.libparsec as any)[func.name] = async (..._args: Array<any>) => {
        return func.result;
      };
    }
  }, functions);
}

export async function clearLibParsecMocks(page: MsPage): Promise<void> {
  await page.evaluate(() => {
    (window as any).clearLibParsecMocks();
  });
}
