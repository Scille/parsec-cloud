// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { EnvironmentType, I18n } from 'megashark-lib';

export const APP_VERSION = __APP_VERSION__;

/*
 STRIPE
*/

const STRIPE_ENV_VARIABLE = 'PARSEC_APP_STRIPE_API_KEY';
// cspell:disable-next-line
const DEFAULT_STRIPE_API_KEY = 'pk_test_P4dfuyoLBQtDHKjTiNDH3JH700TT3mCLbE';
const STRIPE_DISABLE_ENV_VARIABLE = 'PARSEC_APP_DISABLE_STRIPE';

function getStripeApiKey(): { key: string; mode: EnvironmentType } {
  if (import.meta.env[STRIPE_ENV_VARIABLE]) {
    return {
      key: import.meta.env[STRIPE_ENV_VARIABLE],
      mode: (import.meta.env[STRIPE_ENV_VARIABLE] as string).startsWith('pk_live_')
        ? EnvironmentType.Production
        : EnvironmentType.Development,
    };
  }
  return { key: DEFAULT_STRIPE_API_KEY, mode: EnvironmentType.Development };
}

function isStripeDisabled(): boolean {
  return import.meta.env[STRIPE_DISABLE_ENV_VARIABLE] === 'true';
}

/*
 SENTRY
*/

const SENTRY_DISABLE_ENV_VARIABLE = 'PARSEC_APP_DISABLE_SENTRY';

function isSentryDisabled(): boolean {
  return import.meta.env[SENTRY_DISABLE_ENV_VARIABLE] === 'true';
}

/*
 BMS
*/

const BMS_ENV_VARIABLE = 'PARSEC_APP_BMS_API_URL';
const DEFAULT_BMS_URL = 'https://bms-dev.parsec.cloud';

function getBmsUrl(): string {
  if (import.meta.env[BMS_ENV_VARIABLE]) {
    return import.meta.env[BMS_ENV_VARIABLE];
  }
  return DEFAULT_BMS_URL;
}

/*
 PARSEC SIGN
*/

const SIGN_ENV_VARIABLE = 'PARSEC_APP_SIGN_URL';
const DEFAULT_SIGN_URL = 'https://sign-dev.parsec.cloud';

function getSignUrl(): string {
  if (import.meta.env[SIGN_ENV_VARIABLE]) {
    return import.meta.env[SIGN_ENV_VARIABLE];
  }
  return DEFAULT_SIGN_URL;
}

/*
 DEFAULT SERVERS
*/

const SAAS_SERVERS_ENV_VARIABLE = 'PARSEC_APP_SAAS_SERVERS';
const TRIAL_SERVERS_ENV_VARIABLE = 'PARSEC_APP_TRIAL_SERVERS';

const DEFAULT_SAAS_SERVERS = ['saas-v3.parsec.cloud', 'saas-demo-v3-mightyfairy.parsec.cloud'];
const DEFAULT_TRIAL_SERVERS = ['trial.parsec.cloud'];

function getSaasServers(): Array<string> {
  if (import.meta.env[SAAS_SERVERS_ENV_VARIABLE]) {
    return import.meta.env[SAAS_SERVERS_ENV_VARIABLE].split(';');
  }
  return DEFAULT_SAAS_SERVERS;
}

function getTrialServers(): Array<string> {
  if (import.meta.env[TRIAL_SERVERS_ENV_VARIABLE]) {
    return import.meta.env[TRIAL_SERVERS_ENV_VARIABLE].split(';');
  }
  return DEFAULT_TRIAL_SERVERS;
}

/*
 Parsec Auth
*/

const ACCOUNT_DEFAULT_SERVER = 'saas-v3.parsec.cloud';
const ACCOUNT_SERVER_ENV_VARIABLE = 'PARSEC_APP_ACCOUNT_SERVER';
const MOCK_ACCOUNT_ENV_VARIABLE = 'PARSEC_APP_MOCK_ACCOUNT';
const ENABLE_ACCOUNT_ENV_VARIABLE = 'PARSEC_APP_ENABLE_ACCOUNT';
const ENABLE_ACCOUNT_AUTO_LOGIN_ENV_VARIABLE = 'PARSEC_APP_ENABLE_ACCOUNT_AUTO_LOGIN';

function getAccountServer(): string {
  if (import.meta.env[ACCOUNT_SERVER_ENV_VARIABLE]) {
    return import.meta.env[ACCOUNT_SERVER_ENV_VARIABLE];
  }
  return ACCOUNT_DEFAULT_SERVER;
}

function isAccountMocked(): boolean {
  return import.meta.env[MOCK_ACCOUNT_ENV_VARIABLE] === 'true';
}

function isAccountEnabled(): boolean {
  return import.meta.env[ENABLE_ACCOUNT_ENV_VARIABLE] === 'true' || (window as any).TESTING_ENABLE_ACCOUNT === true;
}

function isAccountAutoLoginEnabled(): boolean {
  return isAccountEnabled() && isAccountMocked() && import.meta.env[ENABLE_ACCOUNT_AUTO_LOGIN_ENV_VARIABLE] === 'true';
}

/*
 Links
*/

const CLEAN_APP_VERSION = `${APP_VERSION.slice(0, APP_VERSION.indexOf('+') === -1 ? undefined : APP_VERSION.indexOf('+'))}`;
const APP_VERSION_PREFIX = `v${CLEAN_APP_VERSION}`;

async function openUrl(url: string): Promise<void> {
  window.electronAPI.log('debug', `Opening ${url}`);
  window.open(url, '_blank');
}

async function openDocumentationLink(): Promise<void> {
  await openUrl(I18n.translate({ key: 'MenuPage.documentationLink', data: { version: APP_VERSION_PREFIX } }));
}

async function openContactLink(): Promise<void> {
  await openUrl(I18n.translate({ key: 'MenuPage.contactLink', data: { signUrl: getSignUrl() } }));
}

async function openLicenseLink(): Promise<void> {
  await openUrl(I18n.translate({ key: 'app.licenseLink', data: { version: APP_VERSION_PREFIX } }));
}

async function openChangelogLink(version?: string): Promise<void> {
  await openUrl(I18n.translate({ key: 'app.history', data: { version: version ?? APP_VERSION_PREFIX } }));
}

async function openSourcesLink(): Promise<void> {
  await openUrl(I18n.translate('app.projectSources'));
}

async function openDeveloperLink(): Promise<void> {
  await openUrl(I18n.translate('app.developerLink'));
}

async function openTOS(tosLink: string): Promise<void> {
  await openUrl(tosLink);
}

async function openLogs(): Promise<void> {
  const logs = await window.electronAPI.getLogs();

  const blob = new Blob([logs.join('\n')], { type: 'text/plain' });
  const url = URL.createObjectURL(blob);
  window.open(url, '_blank');
}

export const Env = {
  getStripeApiKey,
  getBmsUrl,
  getSignUrl,
  getSaasServers,
  getTrialServers,
  isStripeDisabled,
  isSentryDisabled,
  getAccountServer,
  isAccountMocked,
  isAccountEnabled,
  isAccountAutoLoginEnabled,
  Links: {
    openDocumentationLink,
    openContactLink,
    openLicenseLink,
    openChangelogLink,
    openSourcesLink,
    openDeveloperLink,
    openTOS,
    openUrl,
    openLogs,
  },
};
