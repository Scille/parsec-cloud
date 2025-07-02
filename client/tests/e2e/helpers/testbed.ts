// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { MsPage } from '@tests/e2e/helpers/types';

type TestbedTemplate = 'coolorg' | 'empty';

declare global {
  interface Window {
    TESTING_CONFIG_PATH: string;
    TESTBED_SERVER_URL: string;
  }
}

export async function initTestBed(page: Page, reuseConfigPath?: string, template: TestbedTemplate = 'coolorg'): Promise<string> {
  const TESTBED_SERVER = process.env.TESTBED_SERVER;
  if (TESTBED_SERVER === undefined) {
    throw new Error('Environ variable `TESTBED_SERVER` must be defined to use testbed');
  }

  // `page.evaluate` runs inside the web page, hence why we pass a function with
  // parameters instead of a closure.
  return await page.evaluate(
    async ([template, testbedServerUrl, reuseConfigPath]) => {
      const [libparsec, nextStage] = window.nextStageHook();
      let configPath: string | undefined = reuseConfigPath;
      if (configPath === undefined) {
        const configResult = await libparsec.testNewTestbed(template, testbedServerUrl);
        if (!configResult.ok) {
          throw new Error(`Failed to init the testbed ${JSON.stringify(configResult.error)}`);
        }
        configPath = configResult.value;
      }
      window.TESTING_CONFIG_PATH = configPath as string;
      window.TESTBED_SERVER_URL = testbedServerUrl as string;
      await nextStage(configPath as string, 'en-US');
      return configPath as string;
    },
    [template, TESTBED_SERVER, reuseConfigPath],
  );
}

export async function dropTestbed(page: Page): Promise<void> {
  await page.evaluate(async () => {
    if ('TESTING_CONFIG_PATH' in window) {
      await window.libparsec.testDropTestbed(window.TESTING_CONFIG_PATH as string);
    }
  });
}

export async function testbedGetAccountCreationCode(page: MsPage, email: string): Promise<string | undefined> {
  const result = await page.evaluate(async (email) => {
    return await window.libparsec.testCheckMailbox(window.TESTBED_SERVER_URL, email);
  }, email);
  if (!result.ok) {
    throw new Error(`Failed to retrieve code: ${result.error.tag} (${result.error.error})`);
  }
  if (result.value.length === 0) {
    return;
  }
  const mailContent = (result.value.at(-1) as [string, any, string])[2];
  if (!mailContent) {
    return;
  }

  const match = mailContent.match(/<div id="code">([A-Z0-9]{4})<\/div>/);
  if (!match || match.length < 2) {
    return;
  }
  return match[1];
}
