// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Platform } from '@/parsec/types';

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
