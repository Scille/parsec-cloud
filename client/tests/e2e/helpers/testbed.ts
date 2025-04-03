// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';

type TestbedTemplate = 'coolorg' | 'empty';

export async function newTestbed(page: Page, template: TestbedTemplate = 'coolorg'): Promise<void> {
  const TESTBED_SERVER = process.env.TESTBED_SERVER;
  if (TESTBED_SERVER === undefined) {
    throw new Error('Environ variable `TESTBED_SERVER` must be defined to use testbed');
  }

  // `page.evaluate` runs inside the web page, hence why we pass a function with
  // parameters instead of a closure.
  await page.evaluate(
    async ([template, testbedServerUrl]) => {
      const [libparsec, nextStage] = window.nextStageHook();
      const configResult = await libparsec.testNewTestbed(template, testbedServerUrl);
      if (!configResult.ok) {
        throw new Error(`Failed to init the testbed ${JSON.stringify(configResult.error)}`);
      }
      const configPath = configResult.value;
      (window as any).TESTING_CONFIG_PATH = configPath;
      await nextStage(configPath, 'en-US');
      return configPath;
    },
    [template, TESTBED_SERVER],
  );
}

export async function dropTestbed(page: Page): Promise<void> {
  await page.evaluate(async () => {
    if ('TESTING_CONFIG_PATH' in window) {
      await window.libparsec.testDropTestbed(window.TESTING_CONFIG_PATH as string);
    }
  });
}
