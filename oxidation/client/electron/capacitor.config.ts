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
      backgroundColor: '#121212'
    }
  },
  electron: {
    trayIconAndMenuEnabled: true,
    splashScreenEnabled: true
  }
};

export default config;
