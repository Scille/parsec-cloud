// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { CapacitorElectronConfig } from '@capacitor-community/electron';

const config: CapacitorElectronConfig = {
  appId: 'com.scille.parsec',
  appName: 'Parsec',
  webDir: 'dist',
  bundledWebRuntime: false,
  plugins: {
    SplashScreen: {
      launchAutoHide: false,
      androidScaleType: 'CENTER_CROP',
      splashFullScreen: true,
      splashImmersive: false,
      backgroundColor: '#121212',
    },
  },
  electron: {
    trayIconAndMenuEnabled: true,
    splashScreenEnabled: false, // sadly we are forced to disable the splash screen because of capacitor-community/electron v5.0.1
    // see: https://github.com/capacitor-community/electron/issues/280
    deepLinkingEnabled: true,
    deepLinkingCustomProtocol: 'parsec3',
  },
};

export default config;
