// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import * as Sentry from '@sentry/electron/main';
import type { MenuItemConstructorOptions } from 'electron';
import { BrowserWindow, MenuItem, app, dialog, ipcMain, shell } from 'electron';
import log from 'electron-log/main.js';
import unhandled from 'electron-unhandled';
import { electronIsDev } from './utils.js';

import fs from 'fs';
import path from 'path';
import capacitorConfig from '../capacitor.config.js';
import { PageToWindowChannel, WindowToPageChannel } from './communicationChannels.js';
import { setupContentSecurityPolicy } from './cspRules.js';
import { setupCustomCaFile } from './customCa.js';
import { FEATURE_FLAGS } from './features.js';
import { ParsecApp, setupReloadWatcher } from './setup.js';

const PARSEC_CONFIG_DIR_NAME = 'parsec3';
const ELECTRON_CONFIG_DIR_NAME = electronIsDev ? 'app-dev' : 'app';
const SENTRY_DSN_GUI_ELECTRON = process.env.SENTRY_DSN_GUI_ELECTRON;
const SENTRY_DSN_GUI_ELECTRON_DEFAULT = 'https://f7f91bb7f676a2f1b8451c386f1a8f9a@o155936.ingest.us.sentry.io/4507638897246208';
const INSTALL_GUIDE_URL_FR = 'https://docs.parsec.cloud/fr/latest/userguide/installation.html';
const INSTALL_GUIDE_URL_EN = 'https://docs.parsec.cloud/en/latest/userguide/installation.html';

// Graceful handling of unhandled errors.
unhandled({
  showDialog: false,
  logger: log.error,
});

function initSentry(): void {
  // Sentry is initialized in
  // - dev mode: only if the env var is set
  // - release mode: always, either with the default DSN or with the env var if set
  let sentry_dsn = SENTRY_DSN_GUI_ELECTRON;
  if (!sentry_dsn && !electronIsDev) {
    sentry_dsn = SENTRY_DSN_GUI_ELECTRON_DEFAULT;
  }
  if (sentry_dsn && FEATURE_FLAGS.sentryEnabled()) {
    Sentry.init({
      dsn: sentry_dsn,
      integrations: [Sentry.captureConsoleIntegration({ levels: ['warn', 'error', 'assert'] })],
    });
  } else {
    if (FEATURE_FLAGS.sentryEnabled()) {
      console.info('Sentry not configured ("SENTRY_DSN_GUI_ELECTRON" env variable was not set).');
    } else {
      console.info('Sentry is disabled');
    }
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
  console.log(`Failed to set config path to '${newConfigDir}'`);
}

log.initialize();
Object.assign(console, log.functions);
if (!electronIsDev) {
  log.transports.file.level = 'info';
  log.transports.console.level = 'info';
} else {
  log.transports.file.level = 'debug';
  log.transports.console.level = 'debug';
}

// Configure Sentry
// Must be done after changing userData directory as Sentry uses this path to
// cache scope and events between application restarts.
// https://docs.sentry.io/platforms/javascript/guides/electron/#app-userdata-directory
initSentry();

// Initialize our app. You can pass menu templates into the app here.
const parsecApp = new ParsecApp(capacitorConfig, appMenuBarMenuTemplate);

function openLink(link: string): void {
  if (parsecApp.pageIsInitialized) {
    parsecApp.sendEvent(WindowToPageChannel.OpenLink, link);
  } else {
    parsecApp.storedLink = link;
  }
}

const lock = app.requestSingleInstanceLock();

if (!lock) {
  app.quit();
} else {
  // If deep linking is enabled then we will set it up here.
  if (capacitorConfig.electron?.deepLinkingEnabled) {
    const customProtocol = capacitorConfig.electron.deepLinkingCustomProtocol ?? 'parsec3';
    if (!app.isDefaultProtocolClient(customProtocol)) {
      app.setAsDefaultProtocolClient(customProtocol);
    }
  }

  // If we are in Dev mode, use the file watcher components.
  if (electronIsDev) {
    setupReloadWatcher(parsecApp);
  }

  // Run Application
  (async (): Promise<any> => {
    let sigProcessed = false;
    // Wait for electron app to be ready.
    await app.whenReady();
    // Security - Set Content-Security-Policy based on whether or not we are in dev mode.
    setupContentSecurityPolicy(parsecApp.getCustomURLScheme());
    await setupCustomCaFile(process.env.SSL_CAFILE);
    // Initialize our app, build windows, and load content.
    await parsecApp.init();
    parsecApp.sendEvent(WindowToPageChannel.CustomBrandingFolder, path.join(app.getPath('userData'), 'custom'));

    process.on('SIGINT', () => {
      if (!sigProcessed) {
        console.log('Killed by SIGINT, cleaning up Parsec...');
        parsecApp.sendEvent(WindowToPageChannel.CloseRequest, true);
      }
      sigProcessed = true;
    });
    process.on('SIGTERM', () => {
      if (!sigProcessed) {
        console.log('Killed by SIGTERM, cleaning up Parsec...');
        parsecApp.sendEvent(WindowToPageChannel.CloseRequest, true);
      }
      sigProcessed = true;
    });
  })();

  app.on('second-instance', (_event, commandLine, _workingDirectory) => {
    if (parsecApp.pageIsInitialized) {
      if (!parsecApp.isMainWindowVisible()) {
        parsecApp.showMainWindow();
      } else {
        parsecApp.focusMainWindow();
      }
    }

    if (commandLine.length > 0) {
      const lastArg = commandLine.at(-1);
      // We're only interested in potential Parsec links
      if (lastArg && lastArg.startsWith('parsec3://')) {
        openLink(lastArg);
      }
    }
  });
}

// Protocol handler for osx
app.on('open-url', (_event, url) => {
  if (url.length > 0 && url.startsWith('parsec3://')) {
    openLink(url);
  }
});

// This is triggered only on cmd+Q on MacOS right before the window `close` event.
// This is used to differentiate cmd+Q from red X which also fires `close`.
app.on('before-quit', async () => {
  if (process.platform === 'darwin') {
    parsecApp.macOSForceQuit = true;
  }
});

// Handle when all of our windows are close (platforms have their own expectations).
app.on('window-all-closed', async () => {
  // On OS X it is common for applications and their menu bar
  // to stay active until the user quits explicitly with Cmd + Q
  if (process.platform !== 'darwin') {
    await parsecApp.quitApp();
  }
});

// When the MacOS dock icon is clicked.
app.on('activate', async () => {
  parsecApp.showMainWindow();
});

ipcMain.on(PageToWindowChannel.ConfigUpdate, (_event, data) => {
  parsecApp.updateConfig(data);
});

ipcMain.on(PageToWindowChannel.MountpointUpdate, (_event, path) => {
  parsecApp.updateMountpoint(path);
});

ipcMain.on(PageToWindowChannel.CloseApp, async (_event) => {
  parsecApp.forceClose = true;
  await parsecApp.quitApp();
});

ipcMain.on(PageToWindowChannel.OpenFile, async (_event, path: string) => {
  // Try the recommanded way first
  const error = await shell.openPath(path);

  if (error) {
    try {
      let updatedPath = path;
      // Add a `file://`
      if (!updatedPath.startsWith('file://')) {
        updatedPath = `file://${updatedPath}`;
      }
      // Encode the URI to handle spaces and other characters
      updatedPath = encodeURI(updatedPath);
      await shell.openExternal(updatedPath);
    } catch (e: any) {
      parsecApp.sendEvent(WindowToPageChannel.OpenPathFailed, path, error);
    }
  }
});

ipcMain.on(PageToWindowChannel.SeeInExplorer, async (_event, path: string) => {
  shell.showItemInFolder(path);
});

ipcMain.on(PageToWindowChannel.UpdateApp, async () => {
  parsecApp.updateApp();
});

ipcMain.on(PageToWindowChannel.UpdateAvailabilityRequest, async () => {
  parsecApp.checkForUpdates();
});

ipcMain.on(PageToWindowChannel.PrepareUpdate, async () => {
  if (parsecApp.updater && parsecApp.updater.isUpdateDownloaded()) {
    parsecApp.sendEvent(WindowToPageChannel.CleanUpBeforeUpdate);
  }
});

ipcMain.on(PageToWindowChannel.Log, async (_event, level: 'debug' | 'info' | 'warn' | 'error', message: string) => {
  parsecApp.log(level, message);
});

ipcMain.on(PageToWindowChannel.PageIsInitialized, async () => {
  parsecApp.onPageInitialized();
});

ipcMain.on(PageToWindowChannel.PageInitError, async (_event, error?: string) => {
  parsecApp.onRendererInitError(error);
});

ipcMain.on(PageToWindowChannel.OpenConfigDir, async () => {
  await shell.openExternal(parsecConfigDir.startsWith('file://') ? parsecConfigDir : `file://${parsecConfigDir}`);
});

ipcMain.on(PageToWindowChannel.MacfuseNotInstalled, async (_event) => {
  const isFRLocale: boolean = app.getLocale().startsWith('fr');
  const url: string = isFRLocale ? INSTALL_GUIDE_URL_FR : INSTALL_GUIDE_URL_EN;

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
  parsecApp.forceClose = true;
  await parsecApp.quitApp();
});

ipcMain.on(PageToWindowChannel.CriticalError, async (_event, message: string, error: Error) => {
  dialog.showErrorBox('Critical error', `${message}: ${error.message}`);
  parsecApp.forceClose = true;
  await parsecApp.quitApp();
});

ipcMain.on(PageToWindowChannel.GetLogs, async () => {
  const content = fs.readFileSync(log.transports.file.getFile().path, { encoding: 'utf-8', flag: 'r' });
  parsecApp.sendEvent(WindowToPageChannel.LogRecords, content);
});

ipcMain.on(PageToWindowChannel.OpenPopup, async (_event, url: string) => {
  const popup = new BrowserWindow({
    width: 600,
    height: 800,
    transparent: false,
    frame: true,
    alwaysOnTop: true,
    resizable: false,
    parent: parsecApp.getMainWindow(),
    show: false,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      sandbox: true,
      javascript: true,
    },
  });
  popup.setMenu(null);
  popup.webContents.setWindowOpenHandler(() => {
    return { action: 'deny' };
  });
  popup.webContents.on('will-redirect', (event, url) => {
    // When the redirect URL is called, check that it is the URL callback, and if so,
    // close the popup and send the infos
    const parsed = new URL(url);
    const params = parsed.searchParams;
    // If the hostname changes, don't forget to also change it in the client
    if (parsed.hostname === 'callback.parsec.cloud.invalid') {
      if (params.get('code') && params.get('state')) {
        parsecApp.sendEvent(WindowToPageChannel.SSOComplete, { ok: true, value: { code: params.get('code'), state: params.get('state') } });
      } else {
        parsecApp.sendEvent(WindowToPageChannel.SSOComplete, {
          ok: false,
          error: { error: params.get('error') ?? 'unknown', description: params.get('error_description') },
        });
      }
      event.preventDefault();
      popup.hide();
      popup.destroy();
    }
  });
  popup.show();
  await popup.loadURL(url);
});
