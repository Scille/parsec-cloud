// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Taken from electron-is-dev` to avoid another useless dependency

import electron from 'electron';

if (typeof electron === 'string') {
  throw new TypeError('Not running in an Electron environment!');
}

const isEnvSet = 'ELECTRON_IS_DEV' in process.env;
const getFromEnv = Number.parseInt(process.env.ELECTRON_IS_DEV, 10) === 1;

const isDev = isEnvSet ? getFromEnv : !electron.app.isPackaged;

export { isDev as electronIsDev };
