// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { CapacitorElectronConfig } from '@capacitor-community/electron';

const config: CapacitorElectronConfig = {
  appId: 'com.scille.parsec',
  appName: 'Parsec',
  webDir: 'dist',
  bundledWebRuntime: false,
  electron: {
    trayIconAndMenuEnabled: true,
    splashScreenEnabled: false, // sadly we are forced to disable the splash screen because of capacitor-community/electron v5.0.1
  }, // see: https://github.com/capacitor-community/electron/issues/280
};

export default config;
