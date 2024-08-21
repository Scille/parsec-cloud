// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { EnvironmentType } from 'megashark-lib';

const STRIPE_ENV_VARIABLE = 'VITE_STRIPE_API_KEY';
// cspell:disable-next-line
const DEFAULT_STRIPE_API_KEY = 'pk_live_5hF9sn4DLUTYWx9uWPDJv51s00Q8ktUVfI';

function getStripeApiKey(): { key: string; mode: EnvironmentType } {
  if (import.meta.env[STRIPE_ENV_VARIABLE]) {
    return { key: import.meta.env[STRIPE_ENV_VARIABLE], mode: EnvironmentType.Development };
  }
  return { key: DEFAULT_STRIPE_API_KEY, mode: EnvironmentType.Production };
}

const BMS_ENV_VARIABLE = 'VITE_BMS_API_URL';
const DEFAULT_BMS_URL = 'https://bms.parsec.cloud';

function getBmsUrl(): string {
  if (import.meta.env[BMS_ENV_VARIABLE]) {
    return import.meta.env[BMS_ENV_VARIABLE];
  }
  return DEFAULT_BMS_URL;
}

const SIGN_ENV_VARIABLE = 'VITE_SIGN_URL';
const DEFAULT_SIGN_URL = 'https://sign.parsec.cloud';

function getSignUrl(): string {
  if (import.meta.env[SIGN_ENV_VARIABLE]) {
    return import.meta.env[SIGN_ENV_VARIABLE];
  }
  return DEFAULT_SIGN_URL;
}

function getBindingsLogConfig(): string | undefined {
  return import.meta.env['RUST_LOG'];
}

export const Env = {
  getStripeApiKey,
  getBmsUrl,
  getSignUrl,
  getBindingsLogConfig,
};
