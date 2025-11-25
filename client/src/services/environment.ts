// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { isWeb } from '@/parsec';
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
  return import.meta.env[STRIPE_DISABLE_ENV_VARIABLE] === 'true' || (window as any).TESTING_DISABLE_STRIPE === true;
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
  if ((window as any).TESTING_SAAS_SERVERS) {
    return (window as any).TESTING_SAAS_SERVERS.split(';');
  }
  if (import.meta.env[SAAS_SERVERS_ENV_VARIABLE]) {
    return import.meta.env[SAAS_SERVERS_ENV_VARIABLE].split(';');
  }
  return DEFAULT_SAAS_SERVERS;
}

function getTrialServers(): Array<string> {
  if ((window as any).TESTING_TRIAL_SERVERS) {
    return (window as any).TESTING_TRIAL_SERVERS.split(';');
  }
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
const ENABLE_ACCOUNT_ENV_VARIABLE = 'PARSEC_APP_ENABLE_ACCOUNT';
const ENABLE_ACCOUNT_AUTO_LOGIN_ENV_VARIABLE = 'PARSEC_APP_ENABLE_ACCOUNT_AUTO_LOGIN';

function getAccountServer(): string {
  let addr = ACCOUNT_DEFAULT_SERVER;

  if ((window as any).TESTING_ACCOUNT_SERVER) {
    addr = (window as any).TESTING_ACCOUNT_SERVER;
  } else if (import.meta.env[ACCOUNT_SERVER_ENV_VARIABLE]) {
    addr = import.meta.env[ACCOUNT_SERVER_ENV_VARIABLE];
  } else if (isWeb()) {
    addr = document.location.hostname;
  } else {
    addr = ACCOUNT_DEFAULT_SERVER;
  }
  // replace whatever scheme with parsec3
  addr = addr.replace(/^([a-z][a-z0-9\-.+]*:\/\/)/i, 'parsec3://');
  // if it doesn't start with a scheme, we add parsec3://
  if (!/^[a-z][a-z0-9-.+]*:\/\//i.test(addr)) {
    addr = `parsec3://${addr}`;
  }

  return addr;
}

function isAccountEnabled(): boolean {
  return import.meta.env[ENABLE_ACCOUNT_ENV_VARIABLE] === 'true' || (window as any).TESTING_ENABLE_ACCOUNT === true;
}

function isAccountAutoLoginEnabled(): boolean {
  return (
    isAccountEnabled() &&
    (import.meta.env[ENABLE_ACCOUNT_AUTO_LOGIN_ENV_VARIABLE] === 'true' || (window as any).TESTING_ACCOUNT_AUTO_LOGIN === true)
  );
}

/*
 Custom branding
*/

const CUSTOM_BRANDING_ENV_VARIABLE = 'PARSEC_APP_ENABLE_CUSTOM_BRANDING';

function isCustomBrandingEnabled(): boolean {
  return import.meta.env[CUSTOM_BRANDING_ENV_VARIABLE] === 'true' || (window as any).TESTING_ENABLE_CUSTOM_BRANDING === true;
}

const OPEN_BAO_SERVER_ENV_VARIABLE = 'PARSEC_APP_OPEN_BAO_SERVER';

function getOpenBaoServer(): string | undefined {
  if ((window as any).TESTING_OPEN_BAO_SERVER) {
    return (window as any).TESTING_OPEN_BAO_SERVER;
  }
  return import.meta.env[OPEN_BAO_SERVER_ENV_VARIABLE];
}

/*
 Links
*/

function normalizeAppVersion(version: string): string {
  version = version.trim();
  if (version.startsWith('v')) {
    return version;
  }
  return `v${version}`;
}

const CLEAN_APP_VERSION = normalizeAppVersion(
  `${APP_VERSION.slice(0, APP_VERSION.indexOf('+') === -1 ? undefined : APP_VERSION.indexOf('+'))}`,
);

async function openUrl(url: string): Promise<void> {
  window.electronAPI.log('debug', `Opening ${url}`);
  window.open(url, '_blank');
}

type Pages =
  | 'troubleshooting'
  | 'introduction'
  | 'installation'
  | 'new_organization'
  | 'first_steps'
  | 'manage_organization'
  | 'join_organization'
  | 'parsec_workspaces'
  | 'manage_files'
  | 'manage_devices'
  | 'security';

async function openDocumentationUserGuideLink(page: Pages, anchor = ''): Promise<void> {
  await openUrl(
    I18n.translate({ key: 'MenuPage.documentationGuideLink', data: { version: CLEAN_APP_VERSION, page: page, anchor: anchor } }),
  );
}

async function openDocumentationLink(): Promise<void> {
  await openUrl(I18n.translate({ key: 'MenuPage.documentationLink', data: { version: CLEAN_APP_VERSION } }));
}

async function openSequesterDocumentationLink(): Promise<void> {
  await openUrl(I18n.translate({ key: 'CreateOrganization.sequester.link', data: { version: CLEAN_APP_VERSION } }));
}

async function openDownloadParsecLink(): Promise<void> {
  await openUrl(I18n.translate({ key: 'MenuPage.downloadParsecLink' }));
}

async function openContactLink(): Promise<void> {
  await openUrl(I18n.translate({ key: 'MenuPage.contactLink', data: { signUrl: getSignUrl() } }));
}

async function openLicenseLink(): Promise<void> {
  await openUrl(I18n.translate({ key: 'app.licenseLink', data: { version: CLEAN_APP_VERSION } }));
}

async function openChangelogLink(version?: string): Promise<void> {
  await openUrl(I18n.translate({ key: 'app.history', data: { version: version ? normalizeAppVersion(version) : CLEAN_APP_VERSION } }));
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

/* Editics / Cryptpad variables */

const ENABLE_EDITICS_VARIABLE = 'PARSEC_APP_ENABLE_EDITICS';
const CRYPTPAD_SERVER_VARIABLE = 'PARSEC_APP_DEFAULT_CRYPTPAD_SERVER';
const DEFAULT_CRYPTPAD_SERVER = 'https://cryptpad-dev.parsec.cloud';

function isEditicsEnabled(): boolean {
  return import.meta.env[ENABLE_EDITICS_VARIABLE] === 'true' || (window as any).TESTING_ENABLE_EDITICS === true;
}

function getDefaultCryptpadServer(): string {
  if ((window as any).TESTING_CRYPTPAD_SERVER) {
    return (window as any).TESTING_CRYPTPAD_SERVER;
  }
  if (import.meta.env[CRYPTPAD_SERVER_VARIABLE]) {
    return import.meta.env[CRYPTPAD_SERVER_VARIABLE];
  }
  return DEFAULT_CRYPTPAD_SERVER;
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
  getOpenBaoServer,
  isAccountEnabled,
  isEditicsEnabled,
  getDefaultCryptpadServer,
  isAccountAutoLoginEnabled,
  isCustomBrandingEnabled,
  Links: {
    openDocumentationLink,
    openDocumentationUserGuideLink,
    openDownloadParsecLink,
    openContactLink,
    openLicenseLink,
    openChangelogLink,
    openSourcesLink,
    openDeveloperLink,
    openTOS,
    openUrl,
    openSequesterDocumentationLink,
  },
};
