// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Platform } from '@/parsec/types';
import { isPlatform } from '@ionic/vue';

// Vue templates cannot access `window`

export function isDesktop(): boolean {
  return window.isDesktop();
}

export function isWeb(): boolean {
  return window.getPlatform() === Platform.Web;
}

export function isMobile(): boolean {
  return window.getPlatform() === Platform.Android;
}

export function isLinux(): boolean {
  return window.getPlatform() === Platform.Linux;
}

export function isWindows(): boolean {
  return window.getPlatform() === Platform.Windows;
}

export function isMacOS(): boolean {
  return window.getPlatform() === Platform.MacOS;
}

export function usesTestbed(): boolean {
  return window.usesTestbed();
}

export function isElectron(): boolean {
  return isPlatform('electron');
}

type Browser = 'Chrome' | 'Firefox' | 'Safari' | 'Edge' | 'Brave' | 'Chromium';

export async function detectBrowser(): Promise<Browser | undefined> {
  if (!isWeb()) {
    return;
  }
  if ((window as any).TESTING_MOCK_BROWSER !== undefined) {
    return (window as any).TESTING_MOCK_BROWSER as Browser;
  }
  // Inspired from https://github.com/Joe12387/detectIncognito, but lighter.
  // Not really reliable.
  let errorLength = 0;
  try {
    (-1).toFixed(-1);
  } catch (err: any) {
    errorLength = err.message.length;
  }
  switch (errorLength) {
    // JavaScriptCore
    case 43:
    case 44:
      return 'Safari';
    // SpiderMonkey
    case 25:
      return 'Firefox';
    // V8
    case 51: {
      const ua = navigator.userAgent;
      if (!ua.match(/Chrome/)) {
        return 'Chromium';
      }
      if ((navigator as any).brave !== undefined) {
        return 'Brave';
      }
      return ua.match(/Edg/) ? 'Edge' : 'Chrome';
    }
    default:
      return undefined;
  }
}
