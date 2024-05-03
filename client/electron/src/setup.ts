// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import type { CapacitorElectronConfig } from '@capacitor-community/electron';
import { CapElectronEventEmitter, CapacitorSplashScreen, setupCapacitorElectronPlugins } from '@capacitor-community/electron';
import chokidar from 'chokidar';
import type { MenuItemConstructorOptions } from 'electron';
import { BrowserWindow, Menu, MenuItem, Tray, app, nativeImage, session, shell } from 'electron';
import log from 'electron-log/main';
import electronServe from 'electron-serve';
import windowStateKeeper from 'electron-window-state';
import { join } from 'path';
import { WindowToPageChannel } from './communicationChannels';
import { electronIsDev } from './utils';
import { WinRegistry } from './winRegistry';

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
  private SplashScreen: CapacitorSplashScreen | null = null;
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
    if (!electronIsDev) {
      Object.assign(console, log.functions);
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

  getCustomURLScheme(): string {
    return this.customScheme;
  }

  updateApp(): void {
    console.log('In electron update app. Sadly does nothing for now :(');
  }

  // Will evolve with auto-update
  async getUpdateInfo(): Promise<{ updateAvailable: boolean; version?: string }> {
    const rand = Math.floor(Math.random() * 2);
    // const result = await appUpdater.checkForUpdates();
    // return result.updateInfo....
    return rand === 1 ? { updateAvailable: true, version: '3.1.0' } : { updateAvailable: false };
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
      Menu.setApplicationMenu(menu);
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

    if (this.CapacitorFileConfig.backgroundColor) {
      this.MainWindow.setBackgroundColor(this.CapacitorFileConfig.electron.backgroundColor);
    }

    // If we close the main window with the splashscreen enabled we need to destroy the ref.
    this.MainWindow.on('closed', () => {
      if (this.SplashScreen?.getSplashWindow() && !this.SplashScreen.getSplashWindow().isDestroyed()) {
        this.SplashScreen.getSplashWindow().close();
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

    // Setup the main manu bar at the top of our window.
    this.MainWindow.setMenu(null);
    // Menu.setApplicationMenu(Menu.buildFromTemplate(this.AppMenuBarMenuTemplate));

    /*
     ** If the splash screen is enabled, show it first while the main window loads, then dismiss it to display the main window.
     ** Alternatively, you can just load the main window from the start without showing the splash screen.
     */
    if (this.CapacitorFileConfig.electron?.splashScreenEnabled) {
      this.SplashScreen = new CapacitorSplashScreen({
        imageFilePath: join(app.getAppPath(), 'assets', this.CapacitorFileConfig.electron?.splashScreenImageName ?? 'splash-screen.png'),
        windowWidth: 600,
        windowHeight: 400,
      });
      this.SplashScreen.init(this.loadMainWindow, this);
    } else {
      this.loadMainWindow(this);
    }

    // Security
    this.MainWindow.webContents.setWindowOpenHandler((details) => {
      function isAuthorizedUrl(url: string): boolean {
        return [
          'https://my.parsec.cloud/',
          'https://parsec.cloud/',
          'https://github.com/Scille/',
          'https://spdx.org/licenses/',
          'https://docs.parsec.cloud/',
        ].some((prefix) => url.startsWith(prefix));
      }

      // Open browser on trying to reach an external link, but only if we know about it.
      if (details.url && isAuthorizedUrl(details.url)) {
        shell.openExternal(details.url);
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

    // When the web app is loaded we hide the splashscreen if needed and show the main window.
    this.MainWindow.webContents.on('dom-ready', () => {
      if (this.CapacitorFileConfig.electron?.splashScreenEnabled) {
        this.SplashScreen.getSplashWindow().hide();
      }
      if (!this.CapacitorFileConfig.electron?.hideMainWindowOnLaunch) {
        this.MainWindow.show();
      }
      setTimeout(() => {
        if (electronIsDev) {
          this.MainWindow.webContents.openDevTools();
        }
        CapElectronEventEmitter.emit('CAPELECTRON_DeeplinkListenerInitialized', '');
        // Send process arguments to renderer
        if (process.argv.length > 0) {
          const lastArg = process.argv.at(-1);
          // We're only interested in potential Parsec links
          if (lastArg.startsWith('parsec3://')) {
            this.sendEvent(WindowToPageChannel.OpenLink, lastArg);
          }
        }
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
            ? `default-src ${customScheme}://* 'unsafe-inline' devtools://* 'unsafe-eval' data:`
            : `default-src ${customScheme}://* 'unsafe-inline' 'unsafe-eval' data:`,
        ],
      },
    });
  });
}
