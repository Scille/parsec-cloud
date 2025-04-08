// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

require('./rt/electron-rt');
import log from 'electron-log/main';

// User Defined Preload scripts below

import { contextBridge, ipcRenderer } from 'electron';
import { PageToWindowChannel } from './communicationChannels';

process.once('loaded', async () => {
  contextBridge.exposeInMainWorld('electronAPI', {
    sendConfig: (config: any) => {
      ipcRenderer.send(PageToWindowChannel.ConfigUpdate, config);
    },
    receive: (channel: string, f: (...args: any[]) => Promise<void>) => {
      ipcRenderer.on(channel, async (event, ...args) => await f(...args));
    },
    closeApp: () => {
      ipcRenderer.send(PageToWindowChannel.CloseApp);
    },
    openFile: (path: string) => {
      ipcRenderer.send(PageToWindowChannel.OpenFile, path);
    },
    sendMountpointFolder: (path: string) => {
      ipcRenderer.send(PageToWindowChannel.MountpointUpdate, path);
    },
    getUpdateAvailability: () => {
      ipcRenderer.send(PageToWindowChannel.UpdateAvailabilityRequest);
    },
    updateApp: () => {
      ipcRenderer.send(PageToWindowChannel.UpdateApp);
    },
    prepareUpdate: () => {
      ipcRenderer.send(PageToWindowChannel.PrepareUpdate);
    },
    log: (level: 'debug' | 'info' | 'warn' | 'error', message: string) => {
      ipcRenderer.send(PageToWindowChannel.Log, level, message);
    },
    pageIsInitialized: () => {
      ipcRenderer.send(PageToWindowChannel.PageIsInitialized);
    },
    openConfigDir: () => {
      ipcRenderer.send(PageToWindowChannel.OpenConfigDir);
    },
    authorizeURL: (url: string) => {
      ipcRenderer.send(PageToWindowChannel.AuthorizeURL, url);
    },
    seeInExplorer: (path: string) => {
      ipcRenderer.send(PageToWindowChannel.SeeInExplorer, path);
    },
    getLogs: (): Promise<Array<string>> => {
      return new Promise((resolve, _reject) => {
        resolve(log.transports.file.readAllLogs().flatMap((v) => v.lines));
      });
    },
  });

  try {
    const libparsec = await import('./libparsec');
    contextBridge.exposeInMainWorld('libparsec_plugin', libparsec);
  } catch (error: any) {
    console.error(error);
    ipcRenderer.send(PageToWindowChannel.Log, 'error', JSON.stringify(error));
    if (process.platform === 'darwin' && error.toString().includes("Reason: tried: '/usr/local/lib/libfuse.2.dylib' (no such file)")) {
      ipcRenderer.send(PageToWindowChannel.MacfuseNotInstalled);
    } else {
      ipcRenderer.send(PageToWindowChannel.CriticalError, 'Failed to initialize Parsec', error);
    }
  }
});
