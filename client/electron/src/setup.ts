// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import type { CapacitorElectronConfig } from '@capacitor-community/electron';
import { CapElectronEventEmitter, setupCapacitorElectronPlugins } from '@capacitor-community/electron';
import chokidar from 'chokidar';
import type { MenuItemConstructorOptions } from 'electron';
import { BrowserWindow, Menu, MenuItem, Tray, app, nativeImage, session, shell } from 'electron';
import log from 'electron-log/main';
import electronServe from 'electron-serve';
import windowStateKeeper from 'electron-window-state';
import fs from 'fs';
import { join } from 'path';
import { WindowToPageChannel } from './communicationChannels';
import { Env } from './envVariables';
import { SplashScreen } from './splashscreen';
import AppUpdater, { UpdaterState, createAppUpdater } from './updater';
import { electronIsDev } from './utils';
import { WinRegistry } from './winRegistry';

const AUTHORIZED_PROTOCOLS = ['http', 'https', 'parsec3'];
const CHECK_UPDATE_INTERVAL = 1000 * 60 * 60; // 1 hour

// Define components for a watcher to detect when the webapp is changed so we can reload in Dev mode.
const reloadWatcher = {
  debouncer: null,
  ready: false,
  watcher: null,
};
export function setupReloadWatcher(electronCapacitorApp: ElectronCapacitorApp): void {
  reloadWatcher.watcher = chokidar
    .watch(join(app.getAppPath(), 'app'), {
      ignored: /[/\\]\./,
      persistent: true,
    })
    .on('ready', () => {
      reloadWatcher.ready = true;
    })
    .on('all', () => {
      if (reloadWatcher.ready) {
        clearTimeout(reloadWatcher.debouncer);
        reloadWatcher.debouncer = setTimeout(async () => {
          electronCapacitorApp.getMainWindow().webContents.reload();
          reloadWatcher.ready = false;
          clearTimeout(reloadWatcher.debouncer);
          reloadWatcher.debouncer = null;
          reloadWatcher.watcher = null;
          setupReloadWatcher(electronCapacitorApp);
        }, 1500);
      }
    });
}

// Define our class to manage our app.
export class ElectronCapacitorApp {
  private MainWindow: BrowserWindow;
  private TrayIcon: Tray;
  private CapacitorFileConfig: CapacitorElectronConfig;
  private TrayMenuTemplate: (MenuItem | MenuItemConstructorOptions)[] = [];
  private AppMenuBarMenuTemplate: (MenuItem | MenuItemConstructorOptions)[] = [
    { role: process.platform === 'darwin' ? 'appMenu' : 'fileMenu' },
    { role: 'viewMenu' },
  ];
  private mainWindowState;
  private loadWebApp;
  private customScheme: string;
  private config: object;
  public forceClose: boolean;
  public macOSForceQuit: boolean = false;
  private APP_GUID = '2f56a772-db54-4a32-b264-28c42970f684';
  private winRegistry: WinRegistry | null = null;
  public pageIsInitialized = false;
  public storedLink = '';
  updater?: AppUpdater;
  private splash: SplashScreen | null = null;

  constructor(capacitorFileConfig: CapacitorElectronConfig, appMenuBarMenuTemplate?: (MenuItemConstructorOptions | MenuItem)[]) {
    this.CapacitorFileConfig = capacitorFileConfig;

    this.customScheme = this.CapacitorFileConfig.electron?.customUrlScheme ?? 'capacitor-electron';

    this.winRegistry = new WinRegistry(this.APP_GUID, app.getAppPath());

    this.TrayMenuTemplate = [
      new MenuItem({
        label: 'Quit App',
        click: async () => {
          this.forceClose = true;
          await this.quitApp();
        },
      }),
    ];

    log.initialize();
    Object.assign(console, log.functions);
    if (!electronIsDev) {
      log.transports.file.level = 'info';
      log.transports.console.level = 'info';
    } else {
      log.transports.file.level = 'debug';
      log.transports.console.level = 'debug';
    }

    for (let i = 1; i < process.argv.length; i++) {
      const arg = process.argv.at(i);
      // We're only interested in potential Parsec links
      if (arg.startsWith('parsec3://')) {
        if (this.storedLink) {
          log.info('Multiple links were passed as arguments, using only the first.');
        } else {
          this.storedLink = arg;
        }
      }
      if (arg === '--log-debug') {
        log.transports.file.level = 'debug';
        log.transports.console.level = 'debug';
        log.debug('Setting log level to DEBUG');
      }
    }

    if (appMenuBarMenuTemplate) {
      this.AppMenuBarMenuTemplate = appMenuBarMenuTemplate;
    }

    this.config = {};
    this.forceClose = false;

    // Setup our web app loader, this lets us load apps like react, vue, and angular without changing their build chains.
    this.loadWebApp = electronServe({
      directory: join(app.getAppPath(), 'app'),
      scheme: this.customScheme,
    });

    if (Env.DISABLE_UPDATES) {
      log['info']('Updates are disabled');
    } else {
      this.updater = createAppUpdater();
    }

    if (this.updater) {
      this.updater.on(UpdaterState.UpdateDownloaded, (info) => {
        this.sendEvent(WindowToPageChannel.UpdateAvailability, true, info.version);
      });

      // Check periodically if an update is available
      setInterval(async () => {
        await this.updater.checkForUpdates();
      }, CHECK_UPDATE_INTERVAL);
    } else if (!electronIsDev) {
      console.warn("We are in a production build but the updater isn't available.");
    }
  }

  // Helper function to load in the app.
  private async loadMainWindow(thisRef: any): Promise<void> {
    await thisRef.loadWebApp(thisRef.MainWindow);
  }

  async quitApp(): Promise<void> {
    await this.winRegistry.removeMountpointFromQuickAccess();
    app.quit();
  }

  // Expose the mainWindow ref for use outside of the class.
  getMainWindow(): BrowserWindow {
    return this.MainWindow;
  }

  sendEvent(event: WindowToPageChannel, ...args: any[]): void {
    this.MainWindow.webContents.send(event, ...args);
  }

  log(level: 'debug' | 'info' | 'warn' | 'error', message: string): void {
    log[level](message);
    if (electronIsDev) {
      this.sendEvent(WindowToPageChannel.PrintToConsole, level, message);
    }
  }

  getCustomURLScheme(): string {
    return this.customScheme;
  }

  showMainWindow(): void {
    if (this.pageIsInitialized) {
      this.MainWindow.show();
    }
  }

  hideMainWindow(): void {
    if (this.pageIsInitialized) {
      this.MainWindow.hide();
    }
  }

  focusMainWindow(): void {
    if (this.pageIsInitialized) {
      this.MainWindow.focus();
    }
  }

  updateApp(): void {
    if (this.updater && this.updater.isUpdateDownloaded()) {
      this.forceClose = true;
      this.macOSForceQuit = true;
      this.updater.quitAndInstall();
    } else {
      console.warn('Update app has been called but no update has been downloaded');
    }
  }

  /**
   * Ask the updater to check for updates.
   * If an update is available, an event will be sent to the web app.
   */
  async checkForUpdates(): Promise<void> {
    if (electronIsDev) {
      setTimeout(() => {
        this.sendEvent(WindowToPageChannel.UpdateAvailability, true, '42.00');
      }, 10000);
    }

    if (this.updater) {
      await this.updater.checkForUpdates();
    }
  }

  updateConfig(newConfig: object): void {
    this.config = newConfig;

    const locale = this.config.hasOwnProperty('locale') ? (this.config as any).locale : 'en-US';
    this.TrayMenuTemplate = [
      new MenuItem({
        label: locale === 'fr-FR' ? 'Afficher Parsec' : 'Show Parsec',
        click: () => {
          this.MainWindow.show();
        },
      }),
      new MenuItem({
        label: locale === 'fr-FR' ? 'Quitter' : 'Quit',
        click: () => {
          this.MainWindow.show();
          this.sendEvent(WindowToPageChannel.CloseRequest, this.skipConfirmBeforeQuit());
        },
      }),
    ];
    this.TrayIcon.setContextMenu(Menu.buildFromTemplate(this.TrayMenuTemplate));
    this.setAppMenu(locale);
  }

  updateMountpoint(path: string): void {
    fs.mkdirSync(path, { recursive: true });
    this.winRegistry.addMountpointToQuickAccess(path, this.getIconPaths().tray);
  }

  isTrayEnabled(): boolean {
    if ('minimizeToTray' in this.config) {
      return (this.config as any).minimizeToTray;
    }
    return false;
  }

  skipConfirmBeforeQuit(): boolean {
    if (process.platform === 'darwin' && 'confirmBeforeQuit' in this.config) {
      return !(this.config as any).confirmBeforeQuit;
    }
    return false;
  }

  isMainWindowVisible(): boolean {
    if (!this.pageIsInitialized) {
      return false;
    }
    if (process.platform === 'darwin') {
      return !this.MainWindow.isMinimized();
    } else {
      return this.MainWindow.isVisible();
    }
  }

  getIconPaths(): { app: string; tray: string } {
    let appIconName: string;
    let trayIconName: string;
    switch (process.platform) {
      case 'darwin':
        appIconName = 'icon.png';
        trayIconName = 'trayIconTemplate.png';
        break;
      case 'linux':
        appIconName = '512x512.png';
        trayIconName = '256x256.png';
        break;
      default:
        appIconName = 'icon.png';
        trayIconName = 'trayIcon.png';
    }
    return {
      app: join(app.getAppPath(), 'assets', appIconName),
      tray: join(app.getAppPath(), 'assets', trayIconName),
    };
  }

  setAppMenu(locale: any): void {
    if (process.platform === 'darwin') {
      const menu = new Menu();
      menu.append(
        new MenuItem({
          label: app.name,
          submenu: [
            {
              label: locale === 'fr-FR' ? 'Quitter' : 'Quit',
              accelerator: 'Cmd+Q',
              click: (): void => {
                this.sendEvent(WindowToPageChannel.CloseRequest, this.skipConfirmBeforeQuit());
              },
            },
          ],
        }),
      );
      menu.append(
        new MenuItem({
          label: locale === 'fr-FR' ? 'Edition' : 'Edit',
          submenu: [
            {
              label: locale === 'fr-FR' ? 'Annuler' : 'Undo',
              accelerator: 'Cmd+Z',
              role: 'undo',
            },
            {
              label: locale === 'fr-FR' ? 'Rétablir' : 'Redo',
              accelerator: 'Shift+Cmd+Z',
              role: 'redo',
            },
            { type: 'separator' },
            {
              label: locale === 'fr-FR' ? 'Couper' : 'Cut',
              accelerator: 'Cmd+X',
              role: 'cut',
            },
            {
              label: locale === 'fr-FR' ? 'Copier' : 'Copy',
              accelerator: 'Cmd+C',
              role: 'copy',
            },
            {
              label: locale === 'fr-FR' ? 'Coller' : 'Paste',
              accelerator: 'Cmd+V',
              role: 'paste',
            },
            {
              label: locale === 'fr-FR' ? 'Tout sélectionner' : 'Select All',
              accelerator: 'Cmd+A',
              role: 'selectAll',
            },
          ],
        }),
      );
      Menu.setApplicationMenu(menu);
    }
  }

  onPageInitialized(): void {
    this.sendEvent(WindowToPageChannel.IsDevMode, electronIsDev);
    setTimeout(() => {
      this.pageIsInitialized = true;
      this.MainWindow.show();
      if (this.splash) {
        this.splash.destroy();
        this.splash = null;
      }
      if (this.storedLink) {
        this.sendEvent(WindowToPageChannel.OpenLink, this.storedLink);
        this.storedLink = '';
      }

      this.winRegistry.areLongPathsEnabled().then((longPathsEnabled) => {
        if (!longPathsEnabled) {
          this.sendEvent(WindowToPageChannel.LongPathsDisabled);
        }
      });
    }, 1500);
  }

  onRendererInitError(error?: string): void {
    error && log.error(error);
    this.MainWindow.show();
    if (this.splash) {
      this.splash.destroy();
      this.splash = null;
    }
  }

  async init(): Promise<void> {
    const iconPaths = this.getIconPaths();
    const appIcon = nativeImage.createFromPath(iconPaths.app);
    const trayIcon = nativeImage.createFromPath(iconPaths.tray);

    this.mainWindowState = windowStateKeeper({
      defaultWidth: 1000,
      defaultHeight: 800,
    });
    // Setup preload script path and construct our main window.
    const preloadPath = join(app.getAppPath(), 'build', 'src', 'preload.js');

    this.MainWindow = new BrowserWindow({
      icon: appIcon,
      show: false,
      x: this.mainWindowState.x,
      y: this.mainWindowState.y,
      width: this.mainWindowState.width,
      height: this.mainWindowState.height,
      minWidth: 1280,
      minHeight: 720,
      webPreferences: {
        nodeIntegration: true,
        contextIsolation: true,
        // Use preload to inject the electron variant overrides for capacitor plugins.
        // preload: join(app.getAppPath(), "node_modules", "@capacitor-community", "electron", "dist", "runtime", "electron-rt.js"),
        preload: preloadPath,
      },
    });
    this.mainWindowState.manage(this.MainWindow);
    this.MainWindow.setMenu(null);
    this.MainWindow.hide();

    if (this.CapacitorFileConfig.electron.splashScreenEnabled) {
      this.splash = new SplashScreen({ width: 624, height: 424 });
      await this.splash.load(join(app.getAppPath(), 'assets', 'splash-screen.png'));
    }

    if (this.CapacitorFileConfig.backgroundColor) {
      this.MainWindow.setBackgroundColor(this.CapacitorFileConfig.electron.backgroundColor);
    }

    // If we close the main window with the splashscreen enabled we need to destroy the ref.
    this.MainWindow.on('closed', () => {
      if (this.splash) {
        this.splash.destroy();
        this.splash = null;
      }
    });

    this.MainWindow.on('show', () => {
      setTimeout(() => {
        this.MainWindow.focus();
      }, 200);
    });

    this.MainWindow.on('close', (event) => {
      if (process.platform === 'darwin') {
        if (!this.macOSForceQuit) {
          // Red X clicked
          if (this.MainWindow.isFullScreen()) {
            // If fullscreen, exit fullscreen first then hide window with a one-time listener
            this.MainWindow.once('leave-full-screen', () => {
              this.MainWindow.hide();
            });
            this.MainWindow.setFullScreen(false);
          } else {
            this.MainWindow.hide();
          }
          event.preventDefault();
        }
      } else {
        if (this.forceClose) {
          return;
        }

        const tray = this.isTrayEnabled();
        if (tray) {
          this.MainWindow.hide();
        } else {
          this.sendEvent(WindowToPageChannel.CloseRequest);
        }
        event.preventDefault();
      }
    });

    // When the tray icon is enabled, setup the options.
    if (this.CapacitorFileConfig.electron?.trayIconAndMenuEnabled) {
      this.TrayIcon = new Tray(trayIcon);

      const trayToggleVisibility = () => {
        this.MainWindow.show();
      };

      // Does not seem to do anything, at least on Linux
      this.TrayIcon.on('double-click', trayToggleVisibility);
      this.TrayIcon.setToolTip(app.getName());
      this.TrayIcon.setContextMenu(Menu.buildFromTemplate(this.TrayMenuTemplate));
    }

    // Setup the main menu bar at the top of our window.
    this.MainWindow.setMenu(null);
    // Menu.setApplicationMenu(Menu.buildFromTemplate(this.AppMenuBarMenuTemplate));

    this.loadMainWindow(this);

    // Security
    this.MainWindow.webContents.setWindowOpenHandler((details) => {
      function isAuthorizedUrl(url: string): boolean {
        return AUTHORIZED_PROTOCOLS.some((protocol) => url.startsWith(protocol));
      }

      // Open browser on trying to reach an external link, but only if we know about it.
      if (details.url) {
        if (isAuthorizedUrl(details.url)) {
          shell.openExternal(details.url);
        } else {
          this.log('warn', `App tried to open unauthorized URL ${details.url}`);
        }
      }
      if (!details.url.includes(this.customScheme)) {
        return { action: 'deny' };
      } else {
        return { action: 'allow' };
      }
    });

    this.MainWindow.webContents.on('will-navigate', (event) => {
      if (!this.MainWindow.webContents.getURL().includes(this.customScheme)) {
        event.preventDefault();
      }
    });

    // Link electron plugins into the system.
    setupCapacitorElectronPlugins();

    this.MainWindow.webContents.on('dom-ready', () => {
      setTimeout(() => {
        if (electronIsDev || process.env.OPEN_DEV_TOOLS == 'true') {
          this.MainWindow.webContents.openDevTools();
        }
        CapElectronEventEmitter.emit('CAPELECTRON_DeeplinkListenerInitialized', '');
      }, 400);
    });
  }
}

// Set a CSP up for our application based on the custom scheme
export function setupContentSecurityPolicy(customScheme: string): void {
  session.defaultSession.webRequest.onHeadersReceived((details, callback) => {
    callback({
      responseHeaders: {
        ...details.responseHeaders,
        'Content-Security-Policy': [
          electronIsDev
            ? `default-src ${customScheme}://* 'unsafe-inline' devtools://* 'unsafe-eval' https://* 'unsafe-eval' data: blob:`
            : `default-src ${customScheme}://* 'unsafe-inline' 'unsafe-eval' https://* 'unsafe-eval' data: blob:`,
        ],
      },
    });
  });
}
