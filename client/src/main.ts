// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { createApp } from 'vue';

import App from '@/App.vue';
import {
  RouteBackup,
  Routes,
  backupCurrentOrganization,
  currentRouteIs,
  getConnectionHandle,
  getRouter,
  navigateTo,
  switchOrganization,
} from '@/router';

import { IonicVue, modalController } from '@ionic/vue';

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
import { getLoggedInDevices, getOrganizationHandle, isElectron, listAvailableDevices, logout, parseFileLink } from '@/parsec';
import { Platform, libparsec } from '@/plugins/libparsec';
import { HotkeyManager, HotkeyManagerKey } from '@/services/hotkeyManager';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';
import { TranslationPlugin, initTranslations } from '@/services/translation';
import '@/theme/global.scss';

import { Base64 } from '@/common/base64';
import Vue3Lottie from 'vue3-lottie';

async function setupApp(): Promise<void> {
  const storageManager = new StorageManager();
  await storageManager.create();

  const config = await storageManager.retrieveConfig();

  const i18n = initTranslations(config.locale);

  const injectionProvider = new InjectionProvider();
  const hotkeyManager = new HotkeyManager();
  const router = getRouter();

  const injections = injectionProvider.getDefault();
  const informationManager = injections.informationManager;

  const app = createApp(App)
    .use(IonicVue, {
      rippleEffect: false,
    })
    .use(router)
    .use(TranslationPlugin)
    .use(Vue3Lottie);

  app.provide(StorageManagerKey, storageManager);
  app.provide(InjectionProviderKey, injectionProvider);
  app.provide(HotkeyManagerKey, hotkeyManager);

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
    const configResult = await libparsec.testNewTestbed('coolorg', import.meta.env.VITE_TESTBED_SERVER_URL);
    if (configResult.ok) {
      nextStage(configResult.value); // Fire-and-forget call
    } else {
      // eslint-disable-next-line no-alert
      alert(
        `Failed to initialize using the testbed.\nTESTBED_SERVER_URL is set to '${import.meta.env.VITE_TESTBED_SERVER_URL}'\n${
          configResult.error.tag
        }: ${configResult.error.error}`,
      );
      if (isElectron()) {
        (window as any).electronAPI.closeApp();
      }
    }
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
    if ((await libparsec.getPlatform()) === Platform.Windows) {
      const mountpoint = await libparsec.getDefaultMountpointBaseDir();
      window.electronAPI.sendMountpointFolder(mountpoint);
    }

    window.electronAPI.receive('close-request', async () => {
      const answer = await askQuestion('quit.title', 'quit.subtitle', {
        yesText: 'quit.yes',
        noText: 'quit.no',
      });
      if (answer === Answer.Yes) {
        const devices = await getLoggedInDevices();
        await injectionProvider.cleanAll();
        for (const device of devices) {
          await logout(device.handle);
        }
        window.electronAPI.closeApp();
      }
    });
    window.electronAPI.receive('open-link', async (link: string) => {
      if (await modalController.getTop()) {
        informationManager.present(
          new Information({
            message: 'link.appIsBusy',
            level: InformationLevel.Error,
          }),
          PresentationMode.Toast,
        );
        return;
      }
      if ((await claimLinkValidator(link)).validity === Validity.Valid) {
        await handleJoinLink(link);
      } else if ((await fileLinkValidator(link)).validity === Validity.Valid) {
        await handleFileLink(link, informationManager);
      } else {
        await informationManager.present(
          new Information({
            message: 'link.invalid',
            level: InformationLevel.Error,
          }),
          PresentationMode.Modal,
        );
      }
    });
    window.electronAPI.receive('open-file-failed', async (path: string, _error: string) => {
      informationManager.present(
        new Information({
          message: { key: 'globalErrors.openFileFailed', data: { path: path } },
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
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
      // eslint-disable-next-line @typescript-eslint/no-empty-function
      sendMountpointFolder: (_path: string): void => {},
    };
  }
}

async function handleJoinLink(link: string): Promise<void> {
  if (!currentRouteIs(Routes.Home)) {
    await switchOrganization(null, true);
  }
  await navigateTo(Routes.Home, { query: { claimLink: link } });
}

async function handleFileLink(link: string, informationManager: InformationManager): Promise<void> {
  const result = await parseFileLink(link);
  if (!result.ok) {
    informationManager.present(
      new Information({
        message: 'link.invalidFileLink',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return;
  }
  const linkData = result.value;
  // Check if the org we want is already logged in
  const handle = await getOrganizationHandle({ id: linkData.organizationId, server: { hostname: linkData.hostname, port: linkData.port } });

  // We have a matching organization already opened
  if (handle) {
    if (getConnectionHandle() && handle !== getConnectionHandle()) {
      await switchOrganization(handle, true);
    }

    const routeData: RouteBackup = {
      handle: handle,
      data: {
        route: Routes.Workspaces,
        params: { handle: handle },
        query: { fileLink: link },
      },
    };
    await navigateTo(Routes.Loading, { skipHandle: true, replace: true, query: { loginInfo: Base64.fromObject(routeData) } });
  } else {
    // Check if we have a device with the org
    const devices = await listAvailableDevices();
    // Always matching the first one, nothing else we can do.
    const matchingDevice = devices.find((d) => d.organizationId === linkData.organizationId);
    if (!matchingDevice) {
      await informationManager.present(
        new Information({
          message: { key: 'link.orgNotFound', data: { organization: linkData.organizationId } },
          level: InformationLevel.Error,
        }),
        PresentationMode.Modal,
      );
      return;
    }
    if (!currentRouteIs(Routes.Home)) {
      await backupCurrentOrganization();
    }
    await navigateTo(Routes.Home, { replace: true, skipHandle: true, query: { deviceId: matchingDevice.deviceId, fileLink: link } });
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
      sendMountpointFolder: (path: string) => void;
    };
  }
}

await setupApp();
