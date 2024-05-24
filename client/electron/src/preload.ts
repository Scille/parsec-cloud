// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

require('./rt/electron-rt');

// User Defined Preload scripts below

import { contextBridge, ipcRenderer } from 'electron';
import { PageToWindowChannel } from './communicationChannels';
import libparsec from './libparsec';

process.once('loaded', () => {
  contextBridge.exposeInMainWorld('libparsec_plugin', libparsec);
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
  });
});
