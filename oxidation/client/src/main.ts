// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import { createApp } from 'vue';
import App from './App.vue';
import router from './router';

import { IonicVue } from '@ionic/vue';
import { createI18n, useI18n } from 'vue-i18n';
import frFR from './locales/fr-FR.json';
import enUS from './locales/en-US.json';

/* Core CSS required for Ionic components to work properly */
import '@ionic/vue/css/core.css';

/* Basic CSS for apps built with Ionic */
import '@ionic/vue/css/normalize.css';
import '@ionic/vue/css/structure.css';
import '@ionic/vue/css/typography.css';

/* Optional CSS utils that can be commented out */
import '@ionic/vue/css/padding.css';
import '@ionic/vue/css/float-elements.css';
import '@ionic/vue/css/text-alignment.css';
import '@ionic/vue/css/text-transformation.css';
import '@ionic/vue/css/flex-utils.css';
import '@ionic/vue/css/display.css';

import { formatTimeSince } from '@/common/date';
import { StorageManager } from '@/services/storageManager';
import { DateTime } from 'luxon';

/* Theme variables */
import './theme/variables.css';
import { libparsec } from './plugins/libparsec';

async function setupApp(): Promise<void> {

  const storageManager = new StorageManager();
  await storageManager.create();

  const config = await storageManager.retrieveConfig();

  /* I18n variables */
  // Type-define 'fr-FR' as the master schema for the resource
  type MessageSchema = typeof frFR;
  const supportedLocales:{[key: string]: string} = {
    fr: 'fr-FR',
    en: 'en-US',
    'fr-FR': 'fr-FR',
    'en-US': 'en-US'
  };
  const defaultLocale = 'fr-FR';
  const i18n = createI18n<[MessageSchema], 'fr-FR' | 'en-US'>({
    legacy: false,
    globalInjection: true,
    locale: config.locale || supportedLocales[window.navigator.language] || defaultLocale,
    messages: {
      'fr-FR': frFR,
      'en-US': enUS
    },
    datetimeFormats: {
      'en-US': {
        short: {
          year: 'numeric',
          month: 'short',
          day: 'numeric'
        },
        long: {
          year: 'numeric',
          month: 'long',
          day: 'numeric',
          weekday: 'long',
          hour: 'numeric',
          minute: 'numeric'
        }
      },
      'fr-FR': {
        short: {
          year: 'numeric',
          month: 'short',
          day: 'numeric'
        },
        long: {
          year: 'numeric',
          month: 'long',
          day: 'numeric',
          weekday: 'long',
          hour: 'numeric',
          minute: 'numeric'
        }
      }
    }
  });

  const app = createApp(App)
    .use(IonicVue)
    .use(router)
    .use(i18n);

  app.provide('formatters', {
    'timeSince': (date: DateTime | undefined, defaultValue=''): string => {
      const { t, d } = useI18n();
      return formatTimeSince(date, t, d, defaultValue);
    }
  });
  app.provide('storageManager', storageManager);

  // We can start the app with different cases :
  // - dev with a testbed Parsec server with the default devices
  // - dev or prod where devices are fetched from the local storage
  // - tests with Cypress where the testbed instantation is done by Cypress

  // We get the app element and we set and attribute to indicate that we are waiting for
  // the config path
  const appElem = document.getElementById('app');
  if (!appElem) {
    throw Error('Cannot retrieve #app');
  }
  appElem.setAttribute('app-state', 'waiting-for-config-path');

  // nextStage() finally mounts the app using the configPath provided
  // Note this function cause a deadlock on `router.isReady` if it is awaited
  // from within `setupApp`, so instead it should be called in fire-and-forget
  // and only awaited when it is called from third party code (i.e. when
  // obtained through `window.nextStageHook`, see below)
  const nextStage = async (configPath: string): Promise<void> => {
    await router.isReady();
    // configPath is injected to components
    app.provide('configPath', configPath);
    app.mount('#app');
    appElem.setAttribute('app-state', 'ready');
  };

  if ('Cypress' in window) {
    // Cypress handle the testbed and provides the configPath
    window.nextStageHook = (): any => {
      return [libparsec, nextStage];
    };
  } else if (import.meta.env.VITE_TESTBED_SERVER_URL) {
    // Dev mode, provide a default testbed
    const configPath = await libparsec.testNewTestbed('coolorg', import.meta.env.VITE_TESTBED_SERVER_URL);
    nextStage(configPath);  // Fire-and-forget call
  } else {
    // Prod or using devices in local storage
    nextStage('/');  // Fire-and-forget call
  }
}

declare global {
  interface Window {
    nextStageHook: () => [any, (configPath: string) => Promise<void>]
  }
}

await setupApp();
