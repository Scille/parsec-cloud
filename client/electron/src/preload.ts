// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

require('./rt/electron-rt');
import * as fs from 'fs';

// User Defined Preload scripts below

import { contextBridge, ipcRenderer } from 'electron';
import path from 'path';
import { PageToWindowChannel } from './communicationChannels';

import unhandled from 'electron-unhandled';

// Graceful handling of unhandled errors.
unhandled({
  showDialog: false,
  logger: (error: Error) => ipcRenderer.send(PageToWindowChannel.Log, 'error', `Unhandled error in renderer: ${String(error)}`),
});

let CUSTOM_BRANDING_FOLDER = './';

ipcRenderer.on('parsec-custom-branding-folder', (_event, ...args: any[]) => {
  CUSTOM_BRANDING_FOLDER = args[0];
});

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
    initError: (error?: string) => {
      ipcRenderer.send(PageToWindowChannel.PageInitError, error);
    },
    openConfigDir: () => {
      ipcRenderer.send(PageToWindowChannel.OpenConfigDir);
    },
    seeInExplorer: (path: string) => {
      ipcRenderer.send(PageToWindowChannel.SeeInExplorer, path);
    },
    getLogs: () => {
      ipcRenderer.send(PageToWindowChannel.GetLogs);
    },
    openPopup: (url: string) => {
      ipcRenderer.send(PageToWindowChannel.OpenPopup, url);
      return true;
    },
    readCustomFile: (file: string): Promise<ArrayBuffer> => {
      return new Promise((resolve, _reject) => {
        // make sure that it's only a file name to avoid path injections
        file = path.basename(file);
        const fullPath = path.join(CUSTOM_BRANDING_FOLDER, file);
        try {
          const data = fs.readFileSync(fullPath);
          const buffer = data.buffer;
          resolve(buffer.slice(data.byteOffset, data.byteOffset + data.byteLength) as ArrayBuffer);
        } catch (e) {
          resolve(undefined);
        }
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
