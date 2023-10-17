// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

require('./rt/electron-rt');

// User Defined Preload scripts below

import { contextBridge, ipcRenderer } from 'electron';
import libparsec from './libparsec';

process.once('loaded', () => {
  contextBridge.exposeInMainWorld('libparsec_plugin', libparsec);
  contextBridge.exposeInMainWorld('electronAPI', {
    sendConfig: (config: any) => {
      ipcRenderer.send('config-update', config);
    },
    receive: (channel: string, f: (data: any) => Promise<void>) => {
      ipcRenderer.on(channel, async (event, data) => await f(data));
    },
    closeApp: () => {
      ipcRenderer.send('close-app');
    }
  });
});
