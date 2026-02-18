// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { MsPage } from '@tests/e2e/helpers/types';

export interface LibParsecFunction {
  name: string;
  result: { ok: true; value: any } | { ok: false; error: { tag: string; error: string } };
  valueConverter?: Array<[string, 'toMap']>;
}

export async function mockLibParsec(page: MsPage, functions: Array<LibParsecFunction>): Promise<void> {
  await page.evaluate((functions) => {
    for (const func of functions) {
      console.log(`Mocking '${func.name}'`);
      if (func.result.ok && func.valueConverter) {
        // We can't pass complex objects to `evaluate`, so no functions, maps, ...
        // So if we want the Parsec's function to return a map for example, we have to pass
        // a Array<[]> to `evaluate`, and then convert it to a Map inside the `evaluate`.
        for (const [attrName, converter] of func.valueConverter) {
          switch (converter) {
            case 'toMap':
              func.result.value[attrName] = new Map(func.result.value[attrName]);
              break;
            default:
              console.error(`Unknown converter '${func.valueConverter}'`);
              break;
          }
        }
      }

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
