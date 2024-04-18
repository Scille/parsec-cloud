// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import type { CapacitorElectronConfig } from '@capacitor-community/electron';
import { getCapacitorElectronConfig, setupElectronDeepLinking } from '@capacitor-community/electron';
import type { MenuItemConstructorOptions } from 'electron';
import { MenuItem, app, ipcMain, shell } from 'electron';
import electronIsDev from 'electron-is-dev';
import unhandled from 'electron-unhandled';
// import { autoUpdater } from 'electron-updater';

import { PageToWindowChannel, WindowToPageChannel } from './communicationChannels';
import { ElectronCapacitorApp, setupContentSecurityPolicy, setupReloadWatcher } from './setup';

// Graceful handling of unhandled errors.
unhandled();

// Define our menu templates (these are optional)
const appMenuBarMenuTemplate: (MenuItemConstructorOptions | MenuItem)[] = [
  { role: process.platform === 'darwin' ? 'appMenu' : 'fileMenu' },
  { role: 'viewMenu' },
];

// Get Config options from capacitor.config
const capacitorFileConfig: CapacitorElectronConfig = getCapacitorElectronConfig();

// Initialize our app. You can pass menu templates into the app here.
// const myCapacitorApp = new ElectronCapacitorApp(capacitorFileConfig);
const myCapacitorApp = new ElectronCapacitorApp(capacitorFileConfig, appMenuBarMenuTemplate);

const lock = app.requestSingleInstanceLock();

if (!lock) {
  app.quit();
} else {
  // If deep linking is enabled then we will set it up here.
  if (capacitorFileConfig.electron?.deepLinkingEnabled) {
    setupElectronDeepLinking(myCapacitorApp, {
      customProtocol: capacitorFileConfig.electron.deepLinkingCustomProtocol ?? 'parsec3',
    });
  }

  // If we are in Dev mode, use the file watcher components.
  if (electronIsDev) {
    setupReloadWatcher(myCapacitorApp);
  }

  // Run Application
  (async (): Promise<any> => {
    // Wait for electron app to be ready.
    await app.whenReady();
    // Security - Set Content-Security-Policy based on whether or not we are in dev mode.
    setupContentSecurityPolicy(myCapacitorApp.getCustomURLScheme());
    // Initialize our app, build windows, and load content.
    await myCapacitorApp.init();
    // Check for updates if we are in a packaged app.
    // autoUpdater.checkForUpdatesAndNotify();
  })();

  app.on('second-instance', (_event, commandLine, _workingDirectory) => {
    if (!myCapacitorApp.isMainWindowVisible()) {
      myCapacitorApp.showMainWindow();
    } else {
      myCapacitorApp.getMainWindow().focus();
    }

    if (commandLine.length > 0) {
      const lastArg = commandLine.at(-1);
      // We're only interested in potential Parsec links
      if (lastArg.startsWith('parsec3://')) {
        myCapacitorApp.sendEvent(WindowToPageChannel.OpenLink, lastArg);
      }
    }
  });
}

// Protocol handler for osx
app.on('open-url', (_event, url) => {
  if (url.length > 0 && url.startsWith('parsec3://')) {
    // Timeout is set if the app is opened with a link, in which case we
    // wait for it to load
    setTimeout(
      () => {
        myCapacitorApp.sendEvent(WindowToPageChannel.OpenLink, url);
      },
      app.isReady() ? 0 : 1000,
    );
  }
});

// Handle when all of our windows are close (platforms have their own expectations).
app.on('window-all-closed', async () => {
  // On OS X it is common for applications and their menu bar
  // to stay active until the user quits explicitly with Cmd + Q
  if (process.platform !== 'darwin') {
    await myCapacitorApp.quitApp();
  }
});

// When the dock icon is clicked.
app.on('activate', async () => {
  // On OS X it's common to re-create a window in the app when the
  // dock icon is clicked and there are no other windows open.
  if (myCapacitorApp.getMainWindow().isDestroyed()) {
    await myCapacitorApp.init();
  }
});

ipcMain.on(PageToWindowChannel.ConfigUpdate, (_event, data) => {
  myCapacitorApp.updateConfig(data);
});

ipcMain.on(PageToWindowChannel.MountpointUpdate, (_event, path) => {
  myCapacitorApp.updateMountpoint(path);
});

ipcMain.on(PageToWindowChannel.CloseApp, async (_event) => {
  myCapacitorApp.forceClose = true;
  await myCapacitorApp.quitApp();
});

ipcMain.on(PageToWindowChannel.OpenFile, async (_event, path: string) => {
  const result = await shell.openPath(path);
  if (result !== '') {
    myCapacitorApp.sendEvent(WindowToPageChannel.OpenPathFailed, path, result);
  }
});

ipcMain.on(PageToWindowChannel.UpdateApp, async () => {
  myCapacitorApp.updateApp();
});

ipcMain.on(PageToWindowChannel.UpdateAvailabilityRequest, async () => {
  const updateInfo = await myCapacitorApp.getUpdateInfo();
  myCapacitorApp.sendEvent(WindowToPageChannel.UpdateAvailability, updateInfo.updateAvailable, updateInfo.version);
});
