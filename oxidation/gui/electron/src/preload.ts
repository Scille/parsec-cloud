require('./rt/electron-rt');
const { contextBridge } = require('electron');
const libparsec = require('../../libparsec');

contextBridge.exposeInMainWorld('libparsec', {
  submitJob: libparsec.submitJob
});
