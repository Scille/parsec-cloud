// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { BrowserWindow, NativeImage } from 'electron';
import log from 'electron-log/main';
import * as fs from 'fs';

export class SplashScreen {
  splash: BrowserWindow;
  loaded: boolean = false;

  constructor(opts?: { width?: number; height?: number; x?: number; y?: number; mainWindow?: BrowserWindow; icon: NativeImage }) {
    // Arbitrary default size
    let width = 800;
    let height = 600;
    let x: number | undefined = undefined;
    let y: number | undefined = undefined;

    if (opts) {
      if (opts.width) {
        // Width is given, use that
        width = opts.width;
      } else if (opts.mainWindow) {
        // otherwise use the width of the main window / 2
        width = Math.floor(opts.mainWindow.getSize()[0] / 2);
      }
      if (opts.height) {
        // Height is given, use that
        height = opts.height;
      } else if (opts.mainWindow) {
        // otherwise use the height of the main window / 2
        height = Math.floor(opts.mainWindow.getSize()[1] / 2);
      }
      if (opts.x) {
        // X position is given, use that
        x = opts.x;
      } else if (opts.mainWindow) {
        // otherwise place it at the center of the main window
        x = opts.mainWindow.getPosition()[0] + Math.floor(opts.mainWindow.getSize()[0] / 2 - width / 2);
      }
      if (opts.y) {
        // Y position is given, use that
        y = opts.y;
      } else if (opts.mainWindow) {
        // otherwise place it at the center of the main window
        y = opts.mainWindow.getPosition()[1] + Math.floor(opts.mainWindow.getSize()[1] / 2 - height / 2);
      }
    }

    this.splash = new BrowserWindow({
      icon: opts.icon,
      width: width,
      height: height,
      transparent: true,
      show: false,
      frame: false,
      alwaysOnTop: true,
      hasShadow: false,
      resizable: false,
      x: x,
      y: y,
    });
  }

  async load(imagePath: string): Promise<void> {
    try {
      const image = fs.readFileSync(imagePath).toString('base64');
      let format = '';

      const lowerCase = imagePath.toLocaleLowerCase();
      if (lowerCase.endsWith('.svg')) {
        format = 'image/svg+xml';
      } else if (lowerCase.endsWith('.png')) {
        format = 'image/png';
      } else {
        console.log(`Unknown image format for splashscreen: ${imagePath}`);
      }
      const imageData = `data:${format};base64,${image}`;

      const html = `
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <style>
    body {
      margin: 0;
      width: 100vw;
      height: 100vh;
      overflow: hidden;
      background: url('${imageData}') no-repeat center center fixed;
      background-size: cover;
    }
  </style>
</head>
<body>
</body>
</html>`;
      await this.splash.loadURL(`data:text/html;charset=UTF-8,${encodeURIComponent(html)}`);
      this.splash.show();
    } catch (err: any) {
      log.error(`Could not load splashscreen: ${err.toString()}`);
    }
  }

  hide(): void {
    this.splash.hide();
  }

  destroy(): void {
    try {
      this.splash.close();
      this.splash.destroy();
    } catch (err: any) {
      log.error(`Could not destroy splashscreen: ${err.toString()}`);
    }
  }
}
