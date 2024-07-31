// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import * as SentryAPI from '@sentry/vue';
import { App as VueApp } from 'vue';
import { Router } from 'vue-router';

const SENTRY_DSN = 'https://f7f91bb7f676a2f1b8451c386f1a8f9a@o155936.ingest.us.sentry.io/4507638897246208';

async function init(app: VueApp, router: Router): Promise<void> {
  console.log('Configuring Sentry...');
  SentryAPI.init({
    app,
    dsn: SENTRY_DSN,
    integrations: [SentryAPI.browserTracingIntegration({ router }), SentryAPI.replayIntegration()],

    // Set tracesSampleRate to 1.0 to capture 100%
    // of transactions for tracing.
    // We recommend adjusting this value in production
    tracesSampleRate: 1.0,

    // Set `tracePropagationTargets` to control for which URLs trace propagation should be enabled
    tracePropagationTargets: ['localhost'],

    // Capture Replay for 10% of all sessions,
    // plus for 100% of sessions with an error
    replaysSessionSampleRate: 0.1,
    replaysOnErrorSampleRate: 1.0,
    beforeSend(event) {
      if (!isEnabled()) {
        return null;
      }
      return event;
    },
  });
}

function disable(): void {
  const client = SentryAPI.getClient();
  if (client) {
    enabled = false;
    console.log('Sentry disabled');
  } else {
    console.log('Sentry is not initialized, cannot disable');
  }
}

function enable(): void {
  const client = SentryAPI.getClient();
  if (client) {
    enabled = true;
    console.log('Sentry enabled');
  } else {
    console.log('Sentry is not initialized, cannot enable');
  }
}

function isEnabled(): boolean {
  return enabled;
}

let enabled = false;

export const Sentry = {
  init,
  disable,
  enable,
};
