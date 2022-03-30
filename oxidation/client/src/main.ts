// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS
import { createApp } from 'vue';
import App from './App.vue';
import router from './router';

import { Storage } from '@ionic/storage';
import { IonicVue } from '@ionic/vue';
import { createI18n } from 'vue-i18n';
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

/* Theme variables */
import './theme/variables.css';

async function setupApp(): Promise<void> {
  const store = new Storage();
  await store.create();

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
    locale: await store.get('userLocale') || supportedLocales[window.navigator.language] || defaultLocale,
    messages: {
      'fr-FR': frFR,
      'en-US': enUS
    }
  });

  const app = createApp(App)
    .use(IonicVue)
    .use(router)
    .use(i18n);

  router.isReady().then(() => {
    app.mount('#app');
  });
}

setupApp();
