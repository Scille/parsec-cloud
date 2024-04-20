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

const electronIsDev = app.isPackaged ? false : true;

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
  private APP_GUID = '2f56a772-db54-4a32-b264-28c42970f684';
  private regedit: any = null;

  constructor(capacitorFileConfig: CapacitorElectronConfig, appMenuBarMenuTemplate?: (MenuItemConstructorOptions | MenuItem)[]) {
    this.CapacitorFileConfig = capacitorFileConfig;

    this.customScheme = this.CapacitorFileConfig.electron?.customUrlScheme ?? 'capacitor-electron';

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

    if (process.platform === 'win32') {
      const reg = require('regedit');
      if (!electronIsDev) {
        const vbsDirectory = join(app.getAppPath(), '../vbs');
        reg.setExternalVBSLocation(vbsDirectory);
      }
      this.regedit = reg.promisified;
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
    await this.removeMountpointFromQuickAccess();
    app.quit();
  }

  // Expose the mainWindow ref for use outside of the class.
  getMainWindow(): BrowserWindow {
    return this.MainWindow;
  }

  getCustomURLScheme(): string {
    return this.customScheme;
  }

  updateConfig(newConfig: object): void {
    this.config = newConfig;

    this.TrayMenuTemplate = [
      new MenuItem({
        label: this.config.hasOwnProperty('locale') && (this.config as any).locale === 'fr-FR' ? 'Afficher Parsec' : 'Show Parsec',
        click: () => {
          this.showMainWindow();
        },
      }),
      new MenuItem({
        label: this.config.hasOwnProperty('locale') && (this.config as any).locale === 'fr-FR' ? 'Quitter' : 'Quit',
        click: () => {
          this.showMainWindow();
          this.MainWindow.webContents.send('close-request');
        },
      }),
    ];
    this.TrayIcon.setContextMenu(Menu.buildFromTemplate(this.TrayMenuTemplate));
  }

  private async addMountpointToQuickAccess(mountpointPath: string): Promise<void> {
    if (process.platform !== 'win32' || !this.regedit) {
      return;
    }
    await this.removeMountpointFromQuickAccess();

    const baseKey1 = `HKCU\\Software\\Classes\\CLSID\\{${this.APP_GUID}}`;
    const baseKey2 = `HKCU\\Software\\Classes\\Wow6432Node\\CLSID\\{${this.APP_GUID}}`;
    const systemRoot = process.env.SYSTEMROOT || 'C:\\Windows';
    const iconPath = this.getIconPaths().tray;

    for (const key of [baseKey1, baseKey2]) {
      await this.regedit.createKey([key]);
      await this.regedit.putValue({
        [key]: {
          AppName: {
            value: 'Parsec',
            type: 'REG_DEFAULT',
          },
          SortOrderIndex: {
            value: 0x42,
            type: 'REG_DWORD',
          },
          'System.IsPinnedToNamespaceTree': {
            value: 0x01,
            type: 'REG_DWORD',
          },
        },
      });
      await this.regedit.createKey([`${key}\\DefaultIcon`]);
      await this.regedit.putValue({
        [`${key}\\DefaultIcon`]: {
          IconPath: {
            value: iconPath,
            type: 'REG_DEFAULT',
          },
        },
      });
      await this.regedit.createKey([`${key}\\InProcServer32`]);
      await this.regedit.putValue({
        [`${key}\\InProcServer32`]: {
          IconPath: {
            value: `${systemRoot}\\system32\\shell32.dll`,
            type: 'REG_DEFAULT',
          },
        },
      });
      await this.regedit.createKey([`${key}\\Instance`]);
      await this.regedit.putValue({
        [`${key}\\Instance`]: {
          CLSID: {
            value: '{0E5AAE11-A475-4c5b-AB00-C66DE400274E}',
            type: 'REG_SZ',
          },
        },
      });
      await this.regedit.createKey([`${key}\\Instance\\InitPropertyBag`]);
      await this.regedit.putValue({
        [`${key}\\Instance\\InitPropertyBag`]: {
          Attributes: {
            value: 0x11,
            type: 'REG_DWORD',
          },
          TargetFolderPath: {
            value: mountpointPath,
            type: 'REG_SZ',
          },
        },
      });

      await this.regedit.createKey([`${key}\\ShellFolder`]);
      await this.regedit.putValue({
        [`${key}\\ShellFolder`]: {
          Attributes: {
            value: 0xf080004d,
            type: 'REG_DWORD',
          },
          FolderValueFlags: {
            value: 0x28,
            type: 'REG_DWORD',
          },
        },
      });
    }

    await this.regedit.createKey(`HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Desktop\\NameSpace\\{${this.APP_GUID}}`);
    await this.regedit.putValue({
      [`HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Desktop\\NameSpace\\{${this.APP_GUID}}`]: {
        AppName: {
          value: 'Parsec',
          type: 'REG_DEFAULT',
        },
      },
    });

    await this.regedit.putValue({
      'HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\HideDesktopIcons\\NewStartPanel': {
        [`{${this.APP_GUID}}`]: {
          value: 0x1,
          type: 'REG_DWORD',
        },
      },
    });
  }

  private async removeMountpointFromQuickAccess(): Promise<void> {
    async function _silentDeleteValues(regedit: any, values: string[]): Promise<void> {
      try {
        await regedit.deleteValue(values);
      } catch (_err) {}
    }

    async function _silentDeleteKeys(regedit: any, keys: string[]): Promise<void> {
      try {
        await regedit.deleteKey(keys);
      } catch (_err) {}
    }

    if (process.platform !== 'win32' || !this.regedit) {
      return;
    }
    const baseKey1 = `HKCU\\Software\\Classes\\CLSID\\{${this.APP_GUID}}`;
    const baseKey2 = `HKCU\\Software\\Classes\\Wow6432Node\\CLSID\\{${this.APP_GUID}}`;

    for (const key of [baseKey1, baseKey2]) {
      await _silentDeleteValues(this.regedit, [
        `${key}\\SortOrderIndex`,
        `${key}\\System.IsPinnedToNamespaceTree`,
        `${key}\\Instance\\CLSID`,
        `${key}\\Instance\\InitPropertyBag\\Attributes`,
        `${key}\\Instance\\InitPropertyBag\\TargetFolderPath`,
        `${key}\\ShellFolder\\Attributes`,
        `${key}\\ShellFolder\\FolderValueFlags`,
        `${key}\\Instance\\InitPropertyBag`,
        `${key}\\Instance`,
      ]);
      await _silentDeleteKeys(this.regedit, [`${key}\\DefaultIcon`, `${key}\\InProcServer32`, `${key}\\ShellFolder`, key]);
    }

    await _silentDeleteValues(this.regedit, [
      `HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Desktop\\NameSpace\\{${this.APP_GUID}}`,
    ]);
    await _silentDeleteKeys(this.regedit, [
      `HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Desktop\\NameSpace\\{${this.APP_GUID}}`,
    ]);
    await _silentDeleteValues(this.regedit, [
      `HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\HideDesktopIcons\\NewStartPanel\\{${this.APP_GUID}}`,
    ]);
  }

  updateMountpoint(path: string): void {
    this.addMountpointToQuickAccess(path);
  }

  isTrayEnabled(): boolean {
    if ('minimizeToTray' in this.config) {
      return (this.config as any).minimizeToTray;
    }
    return false;
  }

  hideMainWindow(): void {
    if (process.platform === 'darwin') {
      this.MainWindow.minimize();
    } else {
      this.MainWindow.hide();
    }
  }

  showMainWindow(): void {
    if (process.platform === 'darwin') {
      this.MainWindow.restore();
    } else {
      this.MainWindow.show();
    }
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
      if (this.forceClose) {
        return;
      }

      const tray = this.isTrayEnabled();
      if (tray) {
        this.hideMainWindow();
      } else {
        this.MainWindow.webContents.send('close-request');
      }
      event.preventDefault();
    });

    // When the tray icon is enabled, setup the options.
    if (this.CapacitorFileConfig.electron?.trayIconAndMenuEnabled) {
      this.TrayIcon = new Tray(trayIcon);

      const trayToggleVisibility = () => {
        this.showMainWindow();
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
            this.MainWindow.webContents.send('open-link', lastArg);
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
