require('./rt/electron-rt');

import { contextBridge } from 'electron';

const libparsec = require('./libparsec');

contextBridge.exposeInMainWorld('libparsec_plugin', {
  submitJob: libparsec.submitJob
});
