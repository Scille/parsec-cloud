import { CapacitorConfig } from '@capacitor/cli';

const config: CapacitorConfig = {
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
      backgroundColor: '#121212' // YOUR SPLASH SCREEN MAIN COLOR
    }
  }
};

export default config;
