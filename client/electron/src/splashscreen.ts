// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { BrowserWindow } from 'electron';
import log from 'electron-log/main';

export class SplashScreen {
  splash: BrowserWindow;
  loaded: boolean = false;

  constructor(from: BrowserWindow) {
    const x = from.getPosition()[0] + (from.getSize()[0] / 2 - 624 / 2);
    const y = from.getPosition()[1] + (from.getSize()[1] / 2 - 424 / 2);

    this.splash = new BrowserWindow({
      width: 624,
      height: 424,
      transparent: true,
      frame: false,
      alwaysOnTop: true,
      hasShadow: true,
      x: x,
      y: y,
    });
  }

  async load(): Promise<boolean> {
    try {
      await this.splash.loadFile('assets/splash.html');
      this.loaded = true;
      return true;
    } catch (err: any) {
      log.error(`Could not load splashscreen: ${err}`);
      this.loaded = false;
    }
    return false;
  }

  isLoaded(): boolean {
    return this.loaded;
  }

  show(): void {
    if (this.loaded) {
      this.splash.show();
    }
  }

  hide(): void {
    let opacity = 1;
    const interval = setInterval(() => {
      this.splash.setOpacity(opacity);
      opacity -= 0.1;
      if (opacity === 0) {
        clearInterval(interval);
        this.splash.hide();
      }
    }, 100);
  }

  close(): void {
    this.splash.close();
    this.splash.destroy();
    this.loaded = false;
  }
}
