// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { createApp } from 'vue';

import App from '@/App.vue';
import { getConnectionHandle, getRouter } from '@/router';
import { Config, StorageManagerKey, ThemeManagerKey, storageManagerInstance } from '@/services/storageManager';
import { IonicVue, isPlatform, modalController, popoverController } from '@ionic/vue';

/* Theme variables */
import '@/theme/global.scss';

import appEnUS from '@/locales/en-US.json';
import appFrFR from '@/locales/fr-FR.json';
import {
  ClientEvent,
  ClientEventGreetingAttemptCancelled,
  ClientEventGreetingAttemptJoined,
  ClientEventGreetingAttemptReady,
  ClientEventInvitationChanged,
  ClientEventTag,
  ConnectionHandle,
  ParsecAccount,
  Platform,
  detectBrowser,
  isElectron,
  listStartedClients,
  logout,
} from '@/parsec';
import { getClientConfig } from '@/parsec/internals';
import { libparsec } from '@/plugins/libparsec';
import { Env } from '@/services/environment';
import { EventDistributor, Events } from '@/services/eventDistributor';
import { HotkeyManager, HotkeyManagerKey } from '@/services/hotkeyManager';
import { Information, InformationDataType, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';
import { handleParsecLink } from '@/services/linkHandler';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { Sentry } from '@/services/sentry';
import { initViewers } from '@/services/viewers';
import { LogLevel, WebLogger } from '@/services/webLogger';
import LongPathsSupportModal from '@/views/about/LongPathsSupportModal.vue';
import IncompatibleEnvironmentModal from '@/views/home/IncompatibleEnvironmentModal.vue';
import { Answer, I18n, Locale, MegaSharkPlugin, Obj, StripeConfig, ThemeManager, askQuestion } from 'megashark-lib';

enum AppState {
  Ready = 'ready',
  Initializing = 'initializing',
}

async function parsecEventCallback(handle: ConnectionHandle, event: ClientEvent): Promise<void> {
  const injections = injectionProvider.getInjections(handle);
  // Event might fire a bit to fast, before we get a chance to create the injections for this handle
  // If we get the default injection, we retry 5s later.
  if (injections.isDefault) {
    setTimeout(() => {
      const inj = injectionProvider.getInjections(handle);
      processEvent(inj.eventDistributor);
    }, 5000);
  } else {
    processEvent(injections.eventDistributor);
  }

  async function processEvent(distributor: EventDistributor): Promise<void> {
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
        distributor.dispatchEvent(Events.ClientRevoked, undefined, { delay: 1000 });
        break;
      case ClientEventTag.ExpiredOrganization:
        distributor.dispatchEvent(Events.ExpiredOrganization, undefined, { delay: 1000 });
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
      case ClientEventTag.WorkspaceOpsOutboundSyncProgress:
        distributor.dispatchEvent(Events.EntrySyncProgress, { workspaceId: event.realmId, entryId: event.entryId, way: 'outbound' });
        break;
      case ClientEventTag.ClientStarted:
        injectionProvider.getDefault().eventDistributor.dispatchEvent(Events.ClientStarted, { handle: handle });
        break;
      case ClientEventTag.ClientStopped:
        injectionProvider.getDefault().eventDistributor.dispatchEvent(Events.ClientStopped, { handle: handle });
        break;
      case ClientEventTag.OrganizationNotFound:
        distributor.dispatchEvent(Events.OrganizationNotFound, undefined, { delay: 1000 });
        break;
      case ClientEventTag.FrozenSelfUser:
        distributor.dispatchEvent(Events.ClientFrozen, undefined, { delay: 1000 });
        break;
      case ClientEventTag.AsyncEnrollmentUpdated:
        distributor.dispatchEvent(Events.AsyncEnrollmentUpdated);
      // Ignore this for now;
      case ClientEventTag.ServerConfigChanged:
        break;
      default:
        window.electronAPI.log('info', `Unhandled event ${event.tag}`);
        break;
    }
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

window.addEventListener('securitypolicyviolation', (e) => {
  window.electronAPI.log('error', `'{e.blockedURI}' blocked because if violates '${e.violatedDirective}' (${e.effectiveDirective})`);
});

const injectionProvider = new InjectionProvider();

async function setupApp(): Promise<void> {
  await WebLogger.init();

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

  if (!isElectron()) {
    setupWebElectronAPI(injectionProvider);
  }
  await ResourcesManager.instance().loadAll();

  await storageManagerInstance.init();
  const storageManager = storageManagerInstance.get();

  const config = await storageManager.retrieveConfig();

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

  const customFrFr = ResourcesManager.instance().get(Resources.TranslationFrFr);
  const customEnUs = ResourcesManager.instance().get(Resources.TranslationEnUs);

  if (customFrFr) {
    Obj.mergeDeep(appFrFR, customFrFr);
  }
  if (customEnUs) {
    Obj.mergeDeep(appEnUS, customEnUs);
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

  if (!Env.isAccountEnabled() || config.skipAccount) {
    ParsecAccount.markSkipped();
  }

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
    if (testbedDiscriminantPath) {
      window.usesTestbed = (): boolean => true;
      window.getConfigDir = (): string => testbedDiscriminantPath;
      window.getDataBaseDir = (): string => testbedDiscriminantPath;
      window.isDev = (): boolean => true;
    } else {
      window.usesTestbed = (): boolean => false;
    }

    await ParsecAccount.init();

    await router.isReady();

    if ((await detectBrowser()) === 'Safari') {
      const modal = await modalController.create({
        component: IncompatibleEnvironmentModal,
        cssClass: 'incompatible-environment-modal',
        componentProps: {
          message: 'globalErrors.incompatibleWithSafari',
        },
        backdropDismiss: false,
        showBackdrop: true,
        canDismiss: false,
      });
      await modal.present();
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
          await cleanBeforeQuitting(injectionProvider, true);
          window.electronAPI.closeApp();
        }
      });
      window.electronAPI.receive('parsec-open-link', async (link: string) => {
        await handleParsecLink(link, getCurrentInformationManager(injectionProvider));
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
        await cleanBeforeQuitting(injectionProvider, true);
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
      window.electronAPI.receive('parsec-long-paths-disabled', async () => {
        if (config.skipLongPathsSupportWarning) {
          window.electronAPI.log('info', 'Support for long paths is disabled but the user chose to ignore the warning.');
          return;
        }
        if ((await modalController.getTop()) !== undefined) {
          window.electronAPI.log(
            'warn',
            "Support for long paths is disabled but we can't warn the user because a modal is already opened.",
          );
          return;
        }
        const modal = await modalController.create({
          component: LongPathsSupportModal,
          canDismiss: true,
          backdropDismiss: false,
          cssClass: 'long-paths-support-modal',
        });
        await modal.present();
        const { data } = await modal.onWillDismiss();
        await modal.dismiss();
        if (data?.skipLongPathsSupportWarning !== undefined) {
          config.skipLongPathsSupportWarning = data.skipLongPathsSupportWarning;
          await storageManager.storeConfig(config);
        }
      });
    }
    window.electronAPI.pageIsInitialized();
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

function setupWebElectronAPI(injectionProvider: InjectionProvider): void {
  window.electronAPI = {
    sendConfig: (_config: Config): void => {},
    closeApp: (): void => {},
    receive: (_channel: string, _f: (...args: any[]) => Promise<void>): void => {},
    openFile: (_path: string): void => {
      console.log('OpenFile: Not available.');
    },
    sendMountpointFolder: (_path: string): void => {
      console.log('SetMountpointFolder: Not available.');
    },
    getUpdateAvailability: (force?: boolean): void => {
      if ((window as any).TESTING_ENABLE_UPDATE_EVENT === true || force) {
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
      if ((window as any).TESTING === true && level === 'debug') {
        return;
      }
      console[level](`[MOCKED-ELECTRON-LOG] ${message}`);
      if ((window as any).TESTING === true) {
        return;
      }
      WebLogger.log(level, message);
    },
    pageIsInitialized: (): void => {
      window.isDev = (): boolean => Boolean(import.meta.env.PARSEC_APP_TESTBED_SERVER);
    },
    initError: (error?: string): void => {
      console.error('Error at initialization', error);
    },
    openConfigDir: (): void => {
      console.log('OpenConfigDir: Not available');
    },
    seeInExplorer: (_path: string): void => {
      console.log('SeeInExplorer: Not available');
    },
    getLogs: (): void => {
      console.warn('Call WebLogger directly in web mode');
    },
    readCustomFile: (_file: string): Promise<ArrayBuffer | undefined> => {
      console.log('readCustomFile: Not available');
      return new Promise((resolve, _reject) => {
        resolve(undefined);
      });
    },
    openPopup: (url: string): boolean => {
      const width = 600;
      const height = 800;
      const left = window.screenX + (window.outerWidth - width) / 2;
      const top = window.screenY + (window.outerHeight - height) / 2;

      const popup = window.open(url, 'Login', `width=${width},height=${height},left=${left},top=${top}`);

      if (!popup) {
        return false;
      }
      return true;
    },
  };
}

async function cleanBeforeQuitting(injectionProvider: InjectionProvider, stopClients = false): Promise<void> {
  await injectionProvider.cleanAll();
  if (stopClients) {
    const started = await listStartedClients();

    for (const [handle, _device] of started) {
      await logout(handle);
    }
  }
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

declare global {
  interface Window {
    nextStageHook: () => [any, (configPath?: string, locale?: string) => Promise<void>];
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
      getUpdateAvailability: (force?: boolean) => void;
      updateApp: () => void;
      prepareUpdate: () => void;
      log: (level: LogLevel, message: string) => void;
      pageIsInitialized: () => void;
      openConfigDir: () => void;
      seeInExplorer: (path: string) => void;
      getLogs: () => void;
      initError: (error?: string) => void;
      readCustomFile: (file: string) => Promise<ArrayBuffer | undefined>;
      openPopup: (url: string) => boolean;
    };
  }
}

await setupApp();
