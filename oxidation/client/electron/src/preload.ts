require('./rt/electron-rt');

// User Defined Preload scripts below

import { contextBridge } from 'electron';
import libparsec from './libparsec';

contextBridge.exposeInMainWorld('libparsec_plugin', libparsec);
