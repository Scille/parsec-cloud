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

// Whether or not module functions should return mock values.
// Currently, this can be used on web, since the bindings are not fully
// implemented, but it could also prove useful when in a testing environment.
export function needsMocks(): boolean {
  return !isDesktop();
}

export function isElectron(): boolean {
  return isPlatform('electron');
}
