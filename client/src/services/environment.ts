// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { EnvironmentType, I18n } from 'megashark-lib';

export const APP_VERSION = __APP_VERSION__;

const STRIPE_ENV_VARIABLE = 'PARSEC_APP_STRIPE_API_KEY';
// cspell:disable-next-line
const DEFAULT_STRIPE_API_KEY = 'pk_test_P4dfuyoLBQtDHKjTiNDH3JH700TT3mCLbE';

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

const BMS_ENV_VARIABLE = 'PARSEC_APP_BMS_API_URL';
const DEFAULT_BMS_URL = 'https://bms-dev.parsec.cloud';

function getBmsUrl(): string {
  if (import.meta.env[BMS_ENV_VARIABLE]) {
    return import.meta.env[BMS_ENV_VARIABLE];
  }
  return DEFAULT_BMS_URL;
}

const SIGN_ENV_VARIABLE = 'PARSEC_APP_SIGN_URL';
const DEFAULT_SIGN_URL = 'https://sign-dev.parsec.cloud';

function getSignUrl(): string {
  if (import.meta.env[SIGN_ENV_VARIABLE]) {
    return import.meta.env[SIGN_ENV_VARIABLE];
  }
  return DEFAULT_SIGN_URL;
}

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

const CLEAN_APP_VERSION = `${APP_VERSION.slice(0, APP_VERSION.indexOf('+') === -1 ? undefined : APP_VERSION.indexOf('+'))}`;
const APP_VERSION_PREFIX = `v${CLEAN_APP_VERSION}`;

async function openDocumentationLink(): Promise<void> {
  window.open(I18n.translate({ key: 'MenuPage.documentationLink', data: { version: APP_VERSION_PREFIX } }), '_blank');
}

async function openContactLink(): Promise<void> {
  window.open(I18n.translate({ key: 'MenuPage.contactLink', data: { signUrl: getSignUrl() } }), '_blank');
}

async function openLicenseLink(): Promise<void> {
  window.open(I18n.translate({ key: 'app.licenseLink', data: { version: APP_VERSION_PREFIX } }), '_blank');
}

async function openChangelogLink(version?: string): Promise<void> {
  window.open(I18n.translate({ key: 'app.history', data: { version: version ?? APP_VERSION_PREFIX } }), '_blank');
}

async function openSourcesLink(): Promise<void> {
  window.open(I18n.translate('app.projectSources'), '_blank');
}

async function openDeveloperLink(): Promise<void> {
  window.open(I18n.translate('app.developerLink'), '_blank');
}

async function openTOS(tosLink: string): Promise<void> {
  window.open(tosLink, '_blank');
}

export const Env = {
  getStripeApiKey,
  getBmsUrl,
  getSignUrl,
  getSaasServers,
  getTrialServers,
  Links: {
    openDocumentationLink,
    openContactLink,
    openLicenseLink,
    openChangelogLink,
    openSourcesLink,
    openDeveloperLink,
    openTOS,
  },
};
