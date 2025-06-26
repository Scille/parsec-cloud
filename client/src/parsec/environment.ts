// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Platform } from '@/parsec/types';
import { isPlatform } from '@ionic/vue';
import detectIncognito from 'detectincognitojs';

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

export async function isIncognito(): Promise<boolean> {
  if (!isWeb()) {
    return false;
  }
  const result = await detectIncognito();
  return result.isPrivate;
}

type Browser = 'Chrome' | 'Firefox' | 'Safari' | 'Edge' | 'Brave' | 'Chromium';

export async function detectBrowser(): Promise<Browser | undefined> {
  if (!isWeb()) {
    return;
  }
  if ((window as any).TESTING_MOCK_BROWSER !== undefined) {
    return (window as any).TESTING_MOCK_BROWSER as Browser;
  }
  const result = await detectIncognito();
  return result.browserName as Browser;
}
