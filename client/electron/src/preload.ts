// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

require('./rt/electron-rt');

// User Defined Preload scripts below

import { contextBridge } from 'electron';
import libparsec from './libparsec';

contextBridge.exposeInMainWorld('libparsec_plugin', libparsec);
