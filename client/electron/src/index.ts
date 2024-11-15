// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import type { CapacitorElectronConfig } from '@capacitor-community/electron';
import { getCapacitorElectronConfig, setupElectronDeepLinking } from '@capacitor-community/electron';
import * as Sentry from '@sentry/electron/main';
import type { MenuItemConstructorOptions } from 'electron';
import { MenuItem, app, dialog, ipcMain, shell } from 'electron';
import unhandled from 'electron-unhandled';
import { electronIsDev } from './utils';

import fs from 'fs';
import path from 'path';
import { PageToWindowChannel, WindowToPageChannel } from './communicationChannels';
import { ElectronCapacitorApp, setupContentSecurityPolicy, setupReloadWatcher } from './setup';

const PARSEC_CONFIG_DIR_NAME = 'parsec3';
const ELECTRON_CONFIG_DIR_NAME = electronIsDev ? 'app-dev' : 'app';
const SENTRY_DSN_GUI_ELECTRON = process.env.SENTRY_DSN_GUI_ELECTRON;
const SENTRY_DSN_GUI_ELECTRON_DEFAULT = 'https://f7f91bb7f676a2f1b8451c386f1a8f9a@o155936.ingest.us.sentry.io/4507638897246208';
const INSTALL_GUIDE_URL_FR = 'https://docs.parsec.cloud/fr/latest/userguide/installation.html';
const INSTALL_GUIDE_URL_EN = 'https://docs.parsec.cloud/en/latest/userguide/installation.html';

// Graceful handling of unhandled errors.
unhandled();

function initSentry(): void {
  // Sentry is initialized in
  // - dev mode: only if the env var is set
  // - release mode: always, either with the default DSN or with the env var if set
  let sentry_dsn = SENTRY_DSN_GUI_ELECTRON;
  if (!sentry_dsn && !electronIsDev) {
    sentry_dsn = SENTRY_DSN_GUI_ELECTRON_DEFAULT;
  }
  if (sentry_dsn) {
    console.log('Configuring Sentry...');
    Sentry.init({
      dsn: sentry_dsn,
      integrations: [Sentry.captureConsoleIntegration({ levels: ['warn', 'error', 'assert'] })],
    });
  } else {
    console.log('Sentry not configured ("SENTRY_DSN_GUI_ELECTRON" env variable was not set).');
  }
}

// Define our menu templates (these are optional)
const appMenuBarMenuTemplate: (MenuItemConstructorOptions | MenuItem)[] = [
  { role: process.platform === 'darwin' ? 'appMenu' : 'fileMenu' },
  { role: 'viewMenu' },
];

const configDir = app.getPath('appData');
const parsecConfigDir = path.join(configDir, PARSEC_CONFIG_DIR_NAME);
const newConfigDir = path.join(parsecConfigDir, ELECTRON_CONFIG_DIR_NAME);
try {
  fs.mkdirSync(newConfigDir, { recursive: true });
  app.setPath('userData', newConfigDir);
} catch (e: any) {
  console.log(`Failed to set config path to '{$newConfigDir}'`);
}

// Configure Sentry
// Must be done after changing userData directory as Sentry uses this path to
// cache scope and events between application restarts.
// https://docs.sentry.io/platforms/javascript/guides/electron/#app-userdata-directory
initSentry();

// Get Config options from capacitor.config
const capacitorFileConfig: CapacitorElectronConfig = getCapacitorElectronConfig();

// Initialize our app. You can pass menu templates into the app here.
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
    let sigProcessed = false;
    // Wait for electron app to be ready.
    await app.whenReady();
    // Security - Set Content-Security-Policy based on whether or not we are in dev mode.
    setupContentSecurityPolicy(myCapacitorApp.getCustomURLScheme());
    // Initialize our app, build windows, and load content.
    await myCapacitorApp.init();

    // Check for updates if we are in a packaged app.
    // autoUpdater.checkForUpdatesAndNotify();
    process.on('SIGINT', () => {
      if (!sigProcessed) {
        console.log('Killed by SIGINT, cleaning up Parsec...');
        myCapacitorApp.sendEvent(WindowToPageChannel.CloseRequest, true);
      }
      sigProcessed = true;
    });
    process.on('SIGTERM', () => {
      if (!sigProcessed) {
        console.log('Killed by SIGTERM, cleaning up Parsec...');
        myCapacitorApp.sendEvent(WindowToPageChannel.CloseRequest, true);
      }
      sigProcessed = true;
    });
  })();

  app.on('second-instance', (_event, commandLine, _workingDirectory) => {
    if (!myCapacitorApp.isMainWindowVisible()) {
      myCapacitorApp.getMainWindow().show();
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

// This is triggered only on cmd+Q on MacOS right before the window `close` event.
// This is used to differentiate cmd+Q from red X which also fires `close`.
app.on('before-quit', async () => {
  if (process.platform === 'darwin') {
    myCapacitorApp.macOSForceQuit = true;
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

// When the MacOS dock icon is clicked.
app.on('activate', async () => {
  myCapacitorApp.getMainWindow().show();
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
  myCapacitorApp.checkForUpdates();
});

ipcMain.on(PageToWindowChannel.PrepareUpdate, async () => {
  if (myCapacitorApp.updater && myCapacitorApp.updater.isUpdateDownloaded()) {
    myCapacitorApp.sendEvent(WindowToPageChannel.CleanUpBeforeUpdate);
  }
});

ipcMain.on(PageToWindowChannel.Log, async (_event, level: 'debug' | 'info' | 'warn' | 'error', message: string) => {
  myCapacitorApp.log(level, message);
});

ipcMain.on(PageToWindowChannel.PageIsInitialized, async () => {
  myCapacitorApp.sendEvent(WindowToPageChannel.IsDevMode, electronIsDev);
});

ipcMain.on(PageToWindowChannel.OpenConfigDir, async () => {
  await shell.openPath(parsecConfigDir);
});

ipcMain.on(PageToWindowChannel.AuthorizeURL, async (_event, url: string) => {
  myCapacitorApp.addAuthorizedURL(url);
});

ipcMain.on(PageToWindowChannel.MacfuseNotInstalled, async (_event) => {
  const isFRLocale: boolean = app.getLocale().startsWith('fr');
  const url: string = isFRLocale ? INSTALL_GUIDE_URL_FR : INSTALL_GUIDE_URL_EN;

  console.log(_event, url);
  shell.openExternal(url);

  dialog.showErrorBox(
    isFRLocale ? "macFUSE n'est pas installé" : 'macFUSE is not installed',
    isFRLocale
      ? `Pour utiliser Parsec, macFUSE doit être installé sur votre Mac.\n\nUn guide d'installation devrait s'être ouvert dans votre \
navigateur, dans lequel vous trouverez les étapes pour installer macFUSE dans l'onglet "macOS".\n\nSi le lien ne s'est pas ouvert \
automatiquement, le guide peut être trouvé à cette adresse : ${INSTALL_GUIDE_URL_FR}`
      : `In order to use Parsec, macFUSE must be installed on your Mac.\n\nAn installation guide should have opened in your browser, \
in which you can find steps to install macFUSE in the "macOS" tab.\n\nIf the link did not automatically open, the guide can be found \
at this address: ${INSTALL_GUIDE_URL_EN}`,
  );
  myCapacitorApp.forceClose = true;
  await myCapacitorApp.quitApp();
});

ipcMain.on(PageToWindowChannel.CriticalError, async (_event, message: string, error: Error) => {
  dialog.showErrorBox('Critical error', `${message}: ${error.message}`);
  myCapacitorApp.forceClose = true;
  await myCapacitorApp.quitApp();
});
