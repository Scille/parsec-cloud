// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { createApp } from 'vue';

// eslint-disable-next-line no-relative-import-paths/no-relative-import-paths
import App from './App.vue';
import router, { routerNavigateTo } from '@/router';

import { IonicVue } from '@ionic/vue';
import { createI18n, useI18n } from 'vue-i18n';
import frFR from '@/locales/fr-FR.json';
import enUS from '@/locales/en-US.json';

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
import { formatFileSize } from '@/common/filesize';
import { Config, StorageManager } from '@/services/storageManager';
import { DateTime } from 'luxon';
import { FormattersKey, StorageManagerKey, NotificationKey } from '@/common/injectionKeys';
import { isPlatform } from '@ionic/vue';

/* Theme variables */
import '@/theme/global.scss';
import { Platform, libparsec } from '@/plugins/libparsec';
import { Notification, NotificationCenter, NotificationLevel } from '@/services/notificationCenter';
import { Answer, askQuestion } from '@/components/core/ms-modal/MsQuestionModal.vue';
import { isElectron, isHomeRoute } from '@/parsec';
import { fileLinkValidator, claimLinkValidator, Validity } from '@/common/validators';
import { ImportManager, ImportManagerKey } from '@/services/importManager';

async function setupApp(): Promise<void> {
  const storageManager = new StorageManager();
  await storageManager.create();

  const config = await storageManager.retrieveConfig();

  /* I18n variables */
  // Type-define 'fr-FR' as the master schema for the resource
  type MessageSchema = typeof frFR;
  const supportedLocales: { [key: string]: string } = {
    fr: 'fr-FR',
    en: 'en-US',
    'fr-FR': 'fr-FR',
    'en-US': 'en-US',
  };
  const defaultLocale = 'fr-FR';
  const i18n = createI18n<[MessageSchema], 'fr-FR' | 'en-US'>({
    legacy: false,
    globalInjection: true,
    locale: config.locale || supportedLocales[window.navigator.language] || defaultLocale,
    messages: {
      'fr-FR': frFR,
      'en-US': enUS,
    },
    datetimeFormats: {
      'en-US': {
        short: {
          year: 'numeric',
          month: 'short',
          day: 'numeric',
        },
        long: {
          year: 'numeric',
          month: 'long',
          day: 'numeric',
          weekday: 'long',
          hour: 'numeric',
          minute: 'numeric',
        },
      },
      'fr-FR': {
        short: {
          year: 'numeric',
          month: 'short',
          day: 'numeric',
        },
        long: {
          year: 'numeric',
          month: 'long',
          day: 'numeric',
          weekday: 'long',
          hour: 'numeric',
          minute: 'numeric',
        },
      },
    },
  });

  const { t } = i18n.global;
  const notificationCenter = new NotificationCenter(t);
  const importManager = new ImportManager();

  const app = createApp(App)
    .use(IonicVue, {
      rippleEffect: false,
    })
    .use(router)
    .use(i18n);

  app.provide(FormattersKey, {
    timeSince: (date: DateTime | undefined, defaultValue = '', format = 'long'): string => {
      const { t, d } = useI18n();
      return formatTimeSince(date, t, d, defaultValue, format);
    },
    fileSize: (bytes: number): string => {
      const { t } = useI18n();
      return formatFileSize(bytes, t);
    },
  });
  app.provide(StorageManagerKey, storageManager);
  app.provide(NotificationKey, notificationCenter);
  app.provide(ImportManagerKey, importManager);

  // We can start the app with different cases :
  // - dev with a testbed Parsec server with the default devices
  // - dev or prod where devices are fetched from the local storage
  // - tests with Cypress where the testbed instantiation is done by Cypress

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
  const nextStage = async (configPath?: string, locale?: string): Promise<void> => {
    await router.isReady();

    const configDir = await libparsec.getDefaultConfigDir();
    const dataBaseDir = await libparsec.getDefaultDataBaseDir();
    const mountpointBaseDir = await libparsec.getDefaultMountpointBaseDir();
    const isDesktop = !('Cypress' in window) && isPlatform('electron');
    const platform = await libparsec.getPlatform();
    const isLinux = isDesktop && platform === Platform.Linux;

    window.getConfigDir = (): string => configDir;
    window.getDataBaseDir = (): string => dataBaseDir;
    window.getMountpointBaseDir = (): string => mountpointBaseDir;
    window.getPlatform = (): Platform => platform;
    window.isDesktop = (): boolean => isDesktop;
    window.isLinux = (): boolean => isLinux;

    if (configPath) {
      window.getConfigDir = (): string => configPath;
    }

    if (locale) {
      (i18n.global.locale as any).value = locale;
    }
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
    nextStage(configPath); // Fire-and-forget call
  } else {
    // Prod or using devices in local storage
    nextStage(); // Fire-and-forget call
  }
  if (import.meta.env.VITE_APP_TEST_MODE?.toLowerCase() === 'true') {
    const x = async (): Promise<void> => {
      await router.isReady();
      router.push('/test');
    };
    x(); // Fire-and-forget call
  }

  if (isElectron()) {
    window.electronAPI.receive('close-request', async () => {
      const answer = await askQuestion(t('quit.title'), t('quit.subtitle'));
      if (answer === Answer.Yes) {
        window.electronAPI.closeApp();
      }
    });
    window.electronAPI.receive('open-link', async (link: string) => {
      if ((await fileLinkValidator(link)) === Validity.Valid || (await claimLinkValidator(link)) === Validity.Valid) {
        if (isHomeRoute()) {
          routerNavigateTo('home', {}, { link: link });
        }
      } else {
        await notificationCenter.showModal(
          new Notification({
            message: t('link.invalid'),
            level: NotificationLevel.Error,
          }),
        );
      }
    });
    window.electronAPI.receive('open-file-failed', async (path: string, _error: string) => {
      notificationCenter.showToast(
        new Notification({
          title: t('openFile.failedTitle'),
          message: t('openFile.failedSubtitle', { path: path }),
          level: NotificationLevel.Error,
        }),
      );
    });
  } else {
    window.electronAPI = {
      // eslint-disable-next-line @typescript-eslint/no-empty-function
      sendConfig: (_config: Config): void => {},
      // eslint-disable-next-line @typescript-eslint/no-empty-function
      closeApp: (): void => {},
      // eslint-disable-next-line @typescript-eslint/no-empty-function
      receive: (_channel: string, _f: (...args: any[]) => Promise<void>): void => {},
      // eslint-disable-next-line @typescript-eslint/no-empty-function
      openFile: (_path: string): void => {},
    };
  }
}

declare global {
  interface Window {
    nextStageHook: () => [any, (configPath: string, locale?: string) => Promise<void>];
    testbedPath: string | null;
    getConfigDir: () => string;
    getDataBaseDir: () => string;
    getMountpointBaseDir: () => string;
    getPlatform: () => Platform;
    isDesktop: () => boolean;
    isLinux: () => boolean;
    electronAPI: {
      sendConfig: (config: Config) => void;
      closeApp: () => void;
      receive: (channel: string, f: (...args: any[]) => Promise<void>) => void;
      openFile: (path: string) => void;
    };
  }
}

await setupApp();
