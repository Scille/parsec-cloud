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
import { Config, StorageManagerKey, ThemeManagerKey, storageManagerInstance } from '@/services/storageManager';
import { IonicVue, isPlatform, modalController, popoverController } from '@ionic/vue';

/* Theme variables */
import '@/theme/global.scss';

import { availableDeviceMatchesServer } from '@/common/device';
import { bootstrapLinkValidator, claimLinkValidator, fileLinkValidator } from '@/common/validators';
import appEnUS from '@/locales/en-US.json';
import appFrFR from '@/locales/fr-FR.json';
import {
  AvailableDevice,
  ClientEvent,
  ClientEventGreetingAttemptCancelled,
  ClientEventGreetingAttemptJoined,
  ClientEventGreetingAttemptReady,
  ClientEventInvitationChanged,
  ClientEventTag,
  ConnectionHandle,
  Platform,
  getOrganizationHandle,
  isElectron,
  listAvailableDevices,
  parseFileLink,
} from '@/parsec';
import { getClientConfig } from '@/parsec/internals';
import { libparsec } from '@/plugins/libparsec';
import { Env } from '@/services/environment';
import { Events } from '@/services/eventDistributor';
import { HotkeyManager, HotkeyManagerKey } from '@/services/hotkeyManager';
import { Information, InformationDataType, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';
import { Sentry } from '@/services/sentry';
import { initViewers } from '@/services/viewers';
import { LogLevel, WebLogger } from '@/services/webLogger';
import { Answer, Base64, I18n, Locale, MegaSharkPlugin, StripeConfig, ThemeManager, Validity, askQuestion } from 'megashark-lib';

enum AppState {
  Ready = 'ready',
  Initializing = 'initializing',
}

async function parsecEventCallback(handle: ConnectionHandle, event: ClientEvent): Promise<void> {
  const distributor = injectionProvider.getInjections(handle).eventDistributor;
  switch (event.tag) {
    case ClientEventTag.Online:
      distributor.dispatchEvent(Events.Online);
      break;
    case ClientEventTag.Offline:
      distributor.dispatchEvent(Events.Offline);
      break;
    case ClientEventTag.MustAcceptTos:
      distributor.dispatchEvent(Events.TOSAcceptRequired, undefined, { delay: 2000 });
      break;
    case ClientEventTag.InvitationChanged:
      distributor.dispatchEvent(Events.InvitationUpdated, {
        token: (event as ClientEventInvitationChanged).token,
        status: (event as ClientEventInvitationChanged).status,
      });
      break;
    case ClientEventTag.GreetingAttemptReady:
      distributor.dispatchEvent(Events.GreetingAttemptReady, {
        token: (event as ClientEventGreetingAttemptReady).token,
        greetingAttempt: (event as ClientEventGreetingAttemptReady).greetingAttempt,
      });
      break;
    case ClientEventTag.GreetingAttemptCancelled:
      distributor.dispatchEvent(Events.GreetingAttemptCancelled, {
        token: (event as ClientEventGreetingAttemptCancelled).token,
        greetingAttempt: (event as ClientEventGreetingAttemptCancelled).greetingAttempt,
      });
      break;
    case ClientEventTag.GreetingAttemptJoined:
      distributor.dispatchEvent(Events.GreetingAttemptJoined, {
        token: (event as ClientEventGreetingAttemptJoined).token,
        greetingAttempt: (event as ClientEventGreetingAttemptJoined).greetingAttempt,
      });
      break;
    case ClientEventTag.IncompatibleServer:
      window.electronAPI.log('warn', `IncompatibleServerEvent: ${JSON.stringify(event)}`);
      distributor.dispatchEvent(
        Events.IncompatibleServer,
        { version: event.apiVersion, supportedVersions: event.supportedApiVersion },
        { delay: 5000 },
      );
      break;
    case ClientEventTag.RevokedSelfUser:
      distributor.dispatchEvent(Events.ClientRevoked);
      break;
    case ClientEventTag.ExpiredOrganization:
      distributor.dispatchEvent(Events.ExpiredOrganization);
      break;
    case ClientEventTag.WorkspaceLocallyCreated:
      distributor.dispatchEvent(Events.WorkspaceCreated);
      break;
    case ClientEventTag.WorkspacesSelfListChanged:
      distributor.dispatchEvent(Events.WorkspaceUpdated);
      break;
    case ClientEventTag.WorkspaceWatchedEntryChanged:
      distributor.dispatchEvent(Events.EntryUpdated, undefined, { aggregateTime: 1000 });
      break;
    case ClientEventTag.WorkspaceOpsInboundSyncDone:
      distributor.dispatchEvent(Events.EntrySynced, { workspaceId: event.realmId, entryId: event.entryId, way: 'inbound' });
      break;
    case ClientEventTag.WorkspaceOpsOutboundSyncDone:
      distributor.dispatchEvent(Events.EntrySynced, { workspaceId: event.realmId, entryId: event.entryId, way: 'outbound' });
      break;
    case ClientEventTag.WorkspaceOpsOutboundSyncStarted:
      distributor.dispatchEvent(Events.EntrySyncStarted, { workspaceId: event.realmId, entryId: event.entryId, way: 'outbound' });
      break;
    // Ignore those events for now
    case ClientEventTag.WorkspaceOpsOutboundSyncProgress:
    case ClientEventTag.ServerConfigChanged:
      break;
    default:
      window.electronAPI.log('debug', `Unhandled event ${event.tag}`);
      break;
  }
}

function preventRightClick(): void {
  window.document.addEventListener('contextmenu', async (event) => {
    if (window.isDesktop() && !window.isDev()) {
      event.preventDefault();
      const top = await popoverController.getTop();
      if (top) {
        await top.dismiss();
      }
    }
  });
}

function warnRefresh(): void {
  window.addEventListener('beforeunload', async (event) => {
    event.preventDefault();
    event.returnValue = true;
  });
}

const injectionProvider = new InjectionProvider();

async function setupApp(): Promise<void> {
  await storageManagerInstance.init();
  const storageManager = storageManagerInstance.get();

  const config = await storageManager.retrieveConfig();

  await WebLogger.init();

  const injectionProvider = new InjectionProvider();
  const hotkeyManager = new HotkeyManager();
  const router = getRouter();

  let stripeConfig: StripeConfig | undefined = undefined;
  if (Env.isStripeDisabled()) {
    console.log('Stripe is disabled');
  } else {
    stripeConfig = {
      publishableKey: Env.getStripeApiKey().key,
      environment: Env.getStripeApiKey().mode,
      locale: config.locale,
    };
  }

  const megasharkPlugin = new MegaSharkPlugin({
    i18n: {
      defaultLocale: config.locale,
      customAssets: {
        'fr-FR': appFrFR,
        'en-US': appEnUS,
      },
      currencies: {
        'fr-FR': 'EUR',
        'en-US': 'EUR',
      },
    },
    stripeConfig: stripeConfig,
  });
  await megasharkPlugin.init();

  const app = createApp(App)
    .use(IonicVue, {
      rippleEffect: false,
    })
    .use(router)
    .use(megasharkPlugin);

  if (!Env.isSentryDisabled()) {
    await Sentry.init(app, router);
  }

  app.provide(StorageManagerKey, storageManager);
  app.provide(InjectionProviderKey, injectionProvider);
  app.provide(HotkeyManagerKey, hotkeyManager);

  if (!isElectron()) {
    setupMockElectronAPI(injectionProvider);
  }

  await initViewers();

  // We get the app element
  const appElem = window.document.getElementById('app');
  if (!appElem) {
    throw Error('Cannot retrieve #app');
  }

  // nextStage() finally mounts the app using the configPath provided
  // Note this function cause a deadlock on `router.isReady` if it is awaited
  // from within `setupApp`, so instead it should be called in fire-and-forget
  // and only awaited when it is called from third party code (i.e. when
  // obtained through `window.nextStageHook`, see below)
  const nextStage = async (testbedDiscriminantPath?: string, locale?: Locale): Promise<void> => {
    await router.isReady();

    const configDir = await libparsec.getDefaultConfigDir();
    const dataBaseDir = await libparsec.getDefaultDataBaseDir();
    const mountpointBaseDir = await libparsec.getDefaultMountpointBaseDir();
    const isDesktop = !('TESTING' in window) && isPlatform('electron');
    const platform = await libparsec.getPlatform();
    const isLinux = isDesktop && platform === Platform.Linux;

    window.getConfigDir = (): string => configDir;
    window.getDataBaseDir = (): string => dataBaseDir;
    window.getMountpointBaseDir = (): string => mountpointBaseDir;
    window.getPlatform = (): Platform => platform;
    window.isDesktop = (): boolean => isDesktop;
    window.isDev = (): boolean => false;
    window.isLinux = (): boolean => isLinux;
    window.isTesting = (): boolean => 'TESTING' in window && (window.TESTING as boolean);

    if (testbedDiscriminantPath) {
      window.usesTestbed = (): boolean => true;
      window.getConfigDir = (): string => testbedDiscriminantPath;
      window.getDataBaseDir = (): string => testbedDiscriminantPath;
      window.isDev = (): boolean => true;
    } else {
      window.usesTestbed = (): boolean => false;
    }

    // Libparsec initialization

    await libparsec.libparsecInitSetOnEventCallback(parsecEventCallback);
    if (platform !== Platform.Web) {
      await libparsec.libparsecInitNativeOnlyInit(getClientConfig());
    }

    if (locale) {
      I18n.changeLocale(locale);
    }
    app.mount('#app');
    appElem.setAttribute('app-state', AppState.Ready);

    const themeManager = new ThemeManager(config.theme);
    app.provide(ThemeManagerKey, themeManager);

    if ((await libparsec.getPlatform()) === Platform.Web) {
      window.addEventListener('unload', async (_event: Event) => {
        // Stop the imports and properly logout on close.
        await cleanBeforeQuitting(injectionProvider);
      });
    }

    if (isElectron()) {
      if ((await libparsec.getPlatform()) === Platform.Windows) {
        const mountpoint = await libparsec.getDefaultMountpointBaseDir();
        window.electronAPI.sendMountpointFolder(mountpoint);
      }

      window.electronAPI.log('info', `BMS Url: ${Env.getBmsUrl()}`);
      window.electronAPI.log('info', `Parsec Sign Url: ${Env.getSignUrl()}`);
      window.electronAPI.log('info', `Stripe API Key: ${Env.getStripeApiKey().key}`);

      let isQuitDialogOpen = false;

      window.electronAPI.receive('parsec-close-request', async (force: boolean = false) => {
        let quit = true;
        if (force === false) {
          if (isQuitDialogOpen) {
            return;
          }
          isQuitDialogOpen = true;
          const answer = await askQuestion('quit.title', 'quit.subtitle', {
            yesText: 'quit.yes',
            noText: 'quit.no',
          });
          isQuitDialogOpen = false;
          quit = answer === Answer.Yes;
        }
        if (quit) {
          await cleanBeforeQuitting(injectionProvider);
          window.electronAPI.closeApp();
        }
      });
      window.electronAPI.receive('parsec-open-link', async (link: string) => {
        const currentInformationManager = getCurrentInformationManager(injectionProvider);
        if (await modalController.getTop()) {
          currentInformationManager.present(
            new Information({
              message: 'link.appIsBusy',
              level: InformationLevel.Error,
            }),
            PresentationMode.Toast,
          );
          return;
        }
        if (await popoverController.getTop()) {
          await popoverController.dismiss();
        }
        if ((await claimLinkValidator(link)).validity === Validity.Valid) {
          await handleJoinLink(link);
        } else if ((await fileLinkValidator(link)).validity === Validity.Valid) {
          await handleFileLink(link, currentInformationManager);
        } else if ((await bootstrapLinkValidator(link)).validity === Validity.Valid) {
          await handleBootstrapLink(link);
        } else {
          await currentInformationManager.present(
            new Information({
              message: 'link.invalid',
              level: InformationLevel.Error,
            }),
            PresentationMode.Modal,
          );
        }
      });
      window.electronAPI.receive('parsec-open-path-failed', async (path: string, _error: string) => {
        getCurrentInformationManager(injectionProvider).present(
          new Information({
            message: { key: 'globalErrors.openFileFailed', data: { path: path } },
            level: InformationLevel.Error,
          }),
          PresentationMode.Toast,
        );
      });
      window.electronAPI.receive('parsec-update-availability', async (updateAvailable: boolean, version?: string) => {
        injectionProvider.distributeEventToAll(Events.UpdateAvailability, { updateAvailable: updateAvailable, version: version });
        if (updateAvailable && version) {
          injectionProvider.notifyAll(
            new Information({
              message: '',
              level: InformationLevel.Info,
              unique: true,
              data: { type: InformationDataType.NewVersionAvailable, newVersion: version },
            }),
            PresentationMode.Notification,
          );
        }
      });
      window.electronAPI.receive('parsec-clean-up-before-update', async () => {
        await cleanBeforeQuitting(injectionProvider);
        window.electronAPI.updateApp();
      });
      window.electronAPI.receive('parsec-is-dev-mode', async (devMode) => {
        window.isDev = (): boolean => devMode;
        if (devMode) {
          Sentry.disable();
        } else {
          config.enableTelemetry ? Sentry.enable() : Sentry.disable();
        }
      });
      window.electronAPI.receive('parsec-print-to-console', async (level: LogLevel, message: string) => {
        console[level](message);
      });

      window.electronAPI.pageIsInitialized();
    }
    preventRightClick();
  };

  // We can start the app with different cases :
  // - dev with a testbed Parsec server with the default devices
  // - dev or prod where devices are fetched from the local storage
  // - tests with Playwright where the testbed instantiation is done by Playwright
  if ('TESTING' in window && window.TESTING === true) {
    Sentry.disable();
    //  handle the testbed and provides the configPath
    window.nextStageHook = (): any => {
      return [libparsec, nextStage];
    };
  } else if (import.meta.env.PARSEC_APP_TESTBED_SERVER) {
    const msg = `\`TESTBED_SERVER\` environ variable detected, creating a new coolorg testbed organization with server ${
      import.meta.env.PARSEC_APP_TESTBED_SERVER
    }`;
    window.electronAPI.log('debug', msg);

    // Dev mode, provide a default testbed
    const configResult = await libparsec.testNewTestbed('coolorg', import.meta.env.PARSEC_APP_TESTBED_SERVER);
    if (configResult.ok) {
      nextStage(configResult.value); // Fire-and-forget call
    } else {
      const message = `Failed to initialize the testbed. TESTBED_SERVER is set to '${import.meta.env.PARSEC_APP_TESTBED_SERVER}': ${
        configResult.error.tag
      } (${configResult.error.error})`;
      window.electronAPI.initError(message);
      setTimeout(() => {
        // eslint-disable-next-line no-alert
        alert(message);
        window.electronAPI.closeApp();
      }, 1500);
    }
  } else {
    // Prod or using devices in local storage
    nextStage(); // Fire-and-forget call
  }
  // Only set the attribute once nextStageHook has been set
  appElem.setAttribute('app-state', AppState.Initializing);

  if (import.meta.env.PARSEC_APP_APP_TEST_MODE?.toLowerCase() === 'true') {
    const x = async (): Promise<void> => {
      await router.isReady();
      router.push('/test');
    };
    x(); // Fire-and-forget call
  }
}

function setupMockElectronAPI(injectionProvider: InjectionProvider): void {
  window.electronAPI = {
    // eslint-disable-next-line @typescript-eslint/no-empty-function
    sendConfig: (_config: Config): void => {},
    // eslint-disable-next-line @typescript-eslint/no-empty-function
    closeApp: (): void => {},
    // eslint-disable-next-line @typescript-eslint/no-empty-function
    receive: (_channel: string, _f: (...args: any[]) => Promise<void>): void => {},
    // eslint-disable-next-line @typescript-eslint/no-empty-function
    openFile: (_path: string): void => {
      console.log('OpenFile: Not available.');
    },
    // eslint-disable-next-line @typescript-eslint/no-empty-function
    sendMountpointFolder: (_path: string): void => {
      console.log('SetMountpointFolder: Not available.');
    },
    getUpdateAvailability: (): void => {
      if (window.usesTestbed()) {
        injectionProvider.distributeEventToAll(Events.UpdateAvailability, { updateAvailable: true, version: '13.37' });
        injectionProvider.notifyAll(
          new Information({
            message: '',
            level: InformationLevel.Info,
            unique: true,
            data: { type: InformationDataType.NewVersionAvailable, newVersion: '13.37' },
          }),
          PresentationMode.Notification,
        );
        console.log('GetUpdateAvailability: MOCKED');
      }
    },
    updateApp: (): void => {
      console.log('UpdateApp: Not available.');
    },
    prepareUpdate: (): void => {
      console.log('PrepareUpdate: Not available');
    },
    log: (level: LogLevel, message: string): void => {
      WebLogger.log(level, message);
      console[level](`[MOCKED-ELECTRON-LOG] ${message}`);
    },
    pageIsInitialized: (): void => {
      window.isDev = (): boolean => Boolean(import.meta.env.PARSEC_APP_TESTBED_SERVER);
      if (!window.isDev()) {
        warnRefresh();
      }
    },
    initError: (error?: string): void => {
      console.error('Error at initialization', error);
    },
    openConfigDir: (): void => {
      console.log('OpenConfigDir: Not available');
    },
    authorizeURL: (_url: string): void => {
      console.log('AuthorizeURL: Not available');
    },
    seeInExplorer: (_path: string): void => {
      console.log('SeeInExplorer: Not available');
    },
    getLogs: (): Promise<Array<string>> => {
      return new Promise((resolve, _reject) => {
        WebLogger.getEntries().then((entries) => {
          resolve(entries.map((entry) => `[${entry.timestamp}] [${entry.level}] ${entry.message}`));
        });
      });
    },
  };
}

async function cleanBeforeQuitting(injectionProvider: InjectionProvider): Promise<void> {
  await injectionProvider.cleanAll();
}

function getCurrentInformationManager(injectionProvider: InjectionProvider): InformationManager {
  const currentHandle = getConnectionHandle();

  // If we're logged in, we try to get the organization information manager
  if (currentHandle) {
    const injections = injectionProvider.getInjections(currentHandle);
    return injections.informationManager;
    // Otherwise, we use the default one
  } else {
    return injectionProvider.getDefault().informationManager;
  }
}

async function handleJoinLink(link: string): Promise<void> {
  if (!currentRouteIs(Routes.Home)) {
    await switchOrganization(null, true);
  }
  await navigateTo(Routes.Home, { query: { claimLink: link } });
}

async function handleBootstrapLink(link: string): Promise<void> {
  if (!currentRouteIs(Routes.Home)) {
    await switchOrganization(null, true);
  }
  await navigateTo(Routes.Home, { query: { bootstrapLink: link } });
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
    let matchingDevice: AvailableDevice | undefined;
    for (const device of devices) {
      // Pre-matching on org id only, in case we don't find a server match. This could happen
      // with different DNS, IP addresses instead of hostname, ...
      if (device.organizationId === linkData.organizationId && !matchingDevice) {
        matchingDevice = device;
      }
      if (
        (await availableDeviceMatchesServer(device, { hostname: linkData.hostname, port: linkData.port })) &&
        device.organizationId === linkData.organizationId
      ) {
        matchingDevice = device;
        // Full match takes priority, we can break out the loop
        break;
      }
    }
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
    isDev: () => boolean;
    isTesting: () => boolean;
    usesTestbed: () => boolean;
    electronAPI: {
      sendConfig: (config: Config) => void;
      closeApp: () => void;
      receive: (channel: string, f: (...args: any[]) => Promise<void>) => void;
      openFile: (path: string) => void;
      sendMountpointFolder: (path: string) => void;
      getUpdateAvailability: () => void;
      updateApp: () => void;
      prepareUpdate: () => void;
      log: (level: LogLevel, message: string) => void;
      pageIsInitialized: () => void;
      openConfigDir: () => void;
      authorizeURL: (url: string) => void;
      seeInExplorer: (path: string) => void;
      getLogs: () => Promise<Array<string>>;
      initError: (error?: string) => void;
    };
  }
}

await setupApp();
