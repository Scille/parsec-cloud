// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { createApp } from 'vue';

import App from '@/App.vue';
import { Routes, currentRouteIs, getRouter, navigateTo } from '@/router';

import { IonicVue } from '@ionic/vue';

/* Core CSS required for Ionic components to work properly */
import '@ionic/vue/css/core.css';

/* Basic CSS for apps built with Ionic */
import '@ionic/vue/css/normalize.css';
import '@ionic/vue/css/structure.css';
import '@ionic/vue/css/typography.css';

/* Optional CSS utils that can be commented out */
import '@ionic/vue/css/display.css';
import '@ionic/vue/css/flex-utils.css';
import '@ionic/vue/css/float-elements.css';
import '@ionic/vue/css/padding.css';
import '@ionic/vue/css/text-alignment.css';
import '@ionic/vue/css/text-transformation.css';

import { Config, StorageManager, StorageManagerKey } from '@/services/storageManager';
import { isPlatform } from '@ionic/vue';

/* Theme variables */
import { Validity, claimLinkValidator, fileLinkValidator } from '@/common/validators';
import { Answer, askQuestion } from '@/components/core';
import { isElectron } from '@/parsec';
import { Platform, libparsec } from '@/plugins/libparsec';
import { ImportManager, ImportManagerKey } from '@/services/importManager';
import { Notification, NotificationKey, NotificationLevel, NotificationManager } from '@/services/notificationManager';
import { initTranslations } from '@/services/translation';
import '@/theme/global.scss';

import Vue3Lottie from 'vue3-lottie';

async function setupApp(): Promise<void> {
  const storageManager = new StorageManager();
  await storageManager.create();

  const config = await storageManager.retrieveConfig();

  const i18n = initTranslations(config.locale);

  const { t } = i18n.global;

  const notificationManager = new NotificationManager();
  const importManager = new ImportManager();
  const router = getRouter();

  const app = createApp(App)
    .use(IonicVue, {
      rippleEffect: false,
    })
    .use(router)
    .use(i18n)
    .use(Vue3Lottie);

  app.provide(StorageManagerKey, storageManager);
  app.provide(NotificationKey, notificationManager);
  app.provide(ImportManagerKey, importManager);

  // We get the app element
  const appElem = document.getElementById('app');
  if (!appElem) {
    throw Error('Cannot retrieve #app');
  }

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

  // We can start the app with different cases :
  // - dev with a testbed Parsec server with the default devices
  // - dev or prod where devices are fetched from the local storage
  // - tests with Cypress where the testbed instantiation is done by Cypress
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
  // Only set the attribute once nextStageHook has been set
  appElem.setAttribute('app-state', 'initializing');

  if (import.meta.env.VITE_APP_TEST_MODE?.toLowerCase() === 'true') {
    const x = async (): Promise<void> => {
      await router.isReady();
      router.push('/test');
    };
    x(); // Fire-and-forget call
  }

  if (isElectron()) {
    window.electronAPI.receive('close-request', async () => {
      const answer = await askQuestion(t('quit.title'), t('quit.subtitle'), {
        yesText: t('quit.yes'),
        noText: t('quit.no'),
      });
      if (answer === Answer.Yes) {
        window.electronAPI.closeApp();
      }
    });
    window.electronAPI.receive('open-link', async (link: string) => {
      if ((await fileLinkValidator(link)).validity === Validity.Valid || (await claimLinkValidator(link)).validity === Validity.Valid) {
        if (currentRouteIs(Routes.Home)) {
          await navigateTo(Routes.Home, { query: { link: link } });
        }
      } else {
        await notificationManager.showModal(
          new Notification({
            title: t('link.invalid.title'),
            message: t('link.invalid.message'),
            level: NotificationLevel.Error,
          }),
        );
      }
    });
    window.electronAPI.receive('open-file-failed', async (path: string, _error: string) => {
      notificationManager.showToast(
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
