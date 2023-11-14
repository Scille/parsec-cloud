// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Handle, isElectron } from '@/parsec';
import { IonicVue, isPlatform } from '@ionic/vue';
import { createApp, App as VueApp } from 'vue';
import App from '@/App.vue';
import { createRouter, createWebHistory } from '@ionic/vue-router';
import { LOGIN_ROUTES, LOGGED_IN_ROUTES } from '@/router/routes';
import { createI18n } from 'vue-i18n';
import frFR from '@/locales/fr-FR.json';
import enUS from '@/locales/en-US.json';
import { Config, StorageManager } from '@/services/storageManager';
import { NotificationCenter } from '@/services/notificationCenter';
import { ImportManager } from '@/common/importManager';
import { FormattersKey, StorageManagerKey, ImportManagerKey, NotificationKey } from '@/common/injectionKeys';
import { DateTime } from 'luxon';
import { formatTimeSince } from '@/common/date';
import { formatFileSize } from '@/common/filesize';
import { Router } from 'vue-router';
import { libparsec, Platform } from '@/plugins/libparsec';
import { Answer, askQuestion } from '@/components/core/ms-modal/MsQuestionModal.vue';
import { claimLinkValidator, fileLinkValidator, Validity } from '@/common/validators';

async function instanciateTranslations(config: Config): Promise<any> {
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
  return i18n;
}

interface ParsecApp {
  mount(elem: HTMLElement): Promise<void>;
  unmount(): Promise<void>;
  getHandle(): Handle | null;
  getRouter(): Router;
  navigateTo(page: string): Promise<void>
}

class LoginApp implements ParsecApp {
  private app: VueApp<Element>;
  private router: Router;
  private notificationCenter: NotificationCenter;

  public constructor(i18n: any, storageManager: StorageManager) {
    this.router = createRouter({
      history: createWebHistory(import.meta.env.BASE_URL),
      routes: LOGIN_ROUTES,
    });

    this.app = createApp(App)
      .use(IonicVue, {
        rippleEffect: false,
      })
      .use(this.router)
      .use(i18n);

    const { t, d } = i18n.global;
    this.notificationCenter = new NotificationCenter(t);
    this.app.provide(FormattersKey, {
      timeSince: (date: DateTime | undefined, defaultValue = '', format = 'long'): string => {
        return formatTimeSince(date, t, d, defaultValue, format);
      },
      fileSize: (bytes: number): string => {
        return formatFileSize(bytes, t);
      },
    });
    this.app.provide(StorageManagerKey, storageManager);
    this.app.provide(NotificationKey, this.notificationCenter);
  }

  public getRouter(): Router {
    return this.router;
  }

  public getHandle(): null {
    return null;
  }

  public async mount(elem: HTMLElement): Promise<void> {
    this.app.mount(elem);
  }

  public async unmount(): Promise<void> {
    this.app.unmount();
  }

  public async navigateTo(page: string): Promise<void> {
    this.router.push({ name: page });
  }
}

class LoggedInApp implements ParsecApp {
  private app: VueApp<Element>;
  private router: Router;
  private _handle: Handle;
  private notificationCenter: NotificationCenter;

  public constructor(i18n: any, handle: Handle, storageManager: StorageManager) {
    this._handle = handle;
    this.router = createRouter({
      history: createWebHistory(import.meta.env.BASE_URL),
      routes: LOGGED_IN_ROUTES,
    });

    this.app = createApp(App)
      .use(IonicVue, {
        rippleEffect: false,
      })
      .use(this.router)
      .use(i18n);

    const { t, d } = i18n.global;
    this.notificationCenter = new NotificationCenter(t);
    const importManager = new ImportManager();
    this.app.provide(FormattersKey, {
      timeSince: (date: DateTime | undefined, defaultValue = '', format = 'long'): string => {
        return formatTimeSince(date, t, d, defaultValue, format);
      },
      fileSize: (bytes: number): string => {
        return formatFileSize(bytes, t);
      },
    });
    this.app.provide(StorageManagerKey, storageManager);
    this.app.provide(NotificationKey, this.notificationCenter);
    this.app.provide(ImportManagerKey, importManager);
  }

  public getRouter(): Router {
    return this.router;
  }

  public getHandle(): Handle {
    return this._handle;
  }

  public async mount(elem: HTMLElement): Promise<void> {
    this.app.mount(elem);
  }

  public async unmount(): Promise<void> {
    this.app.unmount();
  }

  public async navigateTo(page: string): Promise<void> {
    this.router.push({ name: page, params: { handle: this._handle } });
  }
}

export default class AppManager {
  private static instance: AppManager;

  private loginApp: LoginApp | null;
  private loggedInApps: LoggedInApp[];
  private storageManager: StorageManager;
  private i18n: any;
  private currentApp: LoginApp | LoggedInApp | null;
  private isInit: boolean;
  private rootElement: HTMLElement;

  private constructor() {
    this.storageManager = new StorageManager();
    this.loggedInApps = [];
    this.loginApp = null;
    this.currentApp = null;
    this.isInit = false;
    const elem = document.getElementById('app');
    if (!elem) {
      throw Error('Cannot retrieve #app');
    } else {
      this.rootElement = elem;
    }
  }

  public static get(): AppManager {
    if (!AppManager.instance) {
      AppManager.instance = new AppManager();
    }
    return AppManager.instance;
  }

  public async init(): Promise<void> {
    if (this.isInit) {
      return;
    }
    await this.storageManager.create();
    this.i18n = await instanciateTranslations(await this.storageManager.retrieveConfig());
    this.loginApp = new LoginApp(this.i18n, this.storageManager);
    this.isInit = true;

    const nextStage = async (configPath?: string, locale?: string): Promise<void> => {
      await this.loginApp?.getRouter().isReady();

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
        (this.i18n.global.locale as any).value = locale;
      }
      this.loginApp?.mount(this.getMountElement(this.loginApp));
      this.rootElement.setAttribute('app-state', 'ready');
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
        await this.loginApp?.getRouter().isReady();
        this.loginApp?.getRouter().push('/test');
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
      const { t } = this.i18n.global;
      window.electronAPI.receive('open-link', async (link: string) => {
        // if ((await fileLinkValidator(link)) === Validity.Valid || (await claimLinkValidator(link)) === Validity.Valid) {
        //   if (isHomeRoute()) {
        //     routerNavigateTo('home', {}, { link: link });
        //   }
        // } else {
        //   await notificationCenter.showModal(
        //     new Notification({
        //       message: t('link.invalid'),
        //       level: NotificationLevel.Error,
        //     }),
        //   );
        // }
      });
      window.electronAPI.receive('open-file-failed', async (path: string, _error: string) => {
        // notificationCenter.showToast(
        //   new Notification({
        //     title: t('openFile.failedTitle'),
        //     message: t('openFile.failedSubtitle', { path: path }),
        //     level: NotificationLevel.Error,
        //   }),
        // );
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

  private getMountElement(app: LoginApp | LoggedInApp): HTMLElement {
    const elementName = app.getHandle() ? `app-${app.getHandle()}` : 'app-login';

    console.log(`Element ${elementName}`);

    let elem = document.getElementById(elementName);
    if (!elem) {
      elem = document.createElement('div');
      elem.setAttribute('id', elementName);
      this.rootElement.appendChild(elem);
    }
    return elem;
  }

  private findApp(handle: Handle): LoggedInApp | null {
    const elem = this.loggedInApps.find((item) => item.getHandle() === handle);
    return elem ? elem : null;
  }

  public async logout(handle: Handle): Promise<void> {
    const app = this.findApp(handle);

    if (!app) {
      console.error(`App ${handle} not found when loggin out, should not happen`);
    } else {
      this.hideApp(app);
      this.showApp(this.loginApp as LoginApp);
      const index = this.loggedInApps.findIndex((item) => item.getHandle() === handle);

      if (index !== -1) {
        this.loggedInApps.splice(index, 1);
      }
    }
  }

  // public routerNavigateTo(routeName: string, params: any | null = null, query: any | null = null): void {
  //   params = params || {};
  //   params.handle = router.currentRoute.value.params.handle;

  //   router.push({
  //     name: routeName,
  //     params: params,
  //     query: query,
  //   });
  // }

  // export function routerNavigateToWorkspace(workspaceId: WorkspaceID, path = '/'): void {
  //   startWorkspace(workspaceId).then((result) => {
  //     if (result.ok) {
  //       routerNavigateTo('folder', { workspaceHandle: result.value }, { path: path, workspaceId: workspaceId });
  //     } else {
  //       console.log(`Failed to navigate to workspace: ${result.error.tag}`);
  //     }
  //   });

  private async hideApp(app: LoginApp | LoggedInApp | null): Promise<void> {
    if (!app) {
      return;
    }
    const elem = this.getMountElement(app);
    elem.style.visibility = 'hidden';
    this.currentApp = null;
  }

  private async showApp(app: LoginApp | LoggedInApp): Promise<void> {
    const elem = this.getMountElement(app);
    elem.style.visibility = 'visible';
    this.currentApp = app;
  }

  public async login(handle: Handle): Promise<void> {
    const app = this.findApp(handle);
    console.log('Hiding login app');
    this.hideApp(this.loginApp as LoginApp);

    if (app) {
      console.error(`App ${handle} already exists when loggin in, should not happen`);
      this.showApp(app);
    } else {
      const app = new LoggedInApp(this.i18n, handle, this.storageManager);
      app.mount(this.getMountElement(app));
      this.showApp(app);
      this.loggedInApps.push(app);
    }
    console.log('Navigating to workspaces');
    this.currentApp?.navigateTo('workspaces');
  }

  public async goToLogin(): Promise<void> {
    this.showApp(this.loginApp as LoginApp);
  }

  public async switchTo(handle: Handle): Promise<void> {
    if (handle === this.currentApp?.getHandle()) {
      console.error(`Asking to switch to current app handle ${handle}`);
      return;
    }

    const app = this.findApp(handle);

    // Default to login page
    if (!app) {
      console.log(`Could not find app linked to ${handle}, defaulting to login page`);
      this.showApp(this.loginApp as LoginApp);
    } else {
      this.hideApp(this.currentApp);
      this.showApp(app);
    }
  }
}
