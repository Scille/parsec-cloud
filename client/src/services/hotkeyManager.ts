// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { isDesktop, isLinux, isMacOS, isMobile, isWeb, isWindows, needsMocks } from '@/parsec/environment';
import { Routes, currentRouteIs } from '@/router';
import { modalController } from '@ionic/vue';

export const HotkeyManagerKey = 'hotkey';

export enum Modifiers {
  None = 0,
  Ctrl = 1 << 0,
  Alt = 1 << 1,
  Shift = 1 << 2,
}

export enum Platforms {
  Windows = 1 << 0,
  MacOS = 1 << 1,
  Linux = 1 << 2,
  Web = 1 << 3,
  Mobile = 1 << 4,
  Desktop = Windows | MacOS | Linux,
  All = Desktop | Web | Mobile,
}

interface Hotkey {
  key: string;
  modifiers: number;
  platforms: number;
  disableIfModal: boolean;
  route?: Routes;
  callback: () => Promise<void>;
}

interface HotkeyOptions {
  key: string;
  modifiers: number;
  platforms: number;
  disableIfModal?: boolean;
  route?: Routes;
}

export class HotkeyGroup {
  hotkeys: Hotkey[];
  id: number;

  constructor(id: number) {
    this.hotkeys = [];
    this.id = id;
  }

  add(options: HotkeyOptions, callback: () => Promise<void>): void {
    if (needsMocks()) {
      options.platforms |= Platforms.Web;
    }
    this.hotkeys.push({
      key: options.key,
      modifiers: options.modifiers,
      platforms: options.platforms,
      callback: callback,
      disableIfModal: options.disableIfModal === true,
      route: options.route,
    });
  }
}

export class HotkeyManager {
  groups: HotkeyGroup[];
  index: number;

  constructor() {
    this.groups = [];
    this.index = 0;
    window.addEventListener('keydown', async (event: KeyboardEvent): Promise<void> => await this.onKeyPress(event, this.groups));
  }

  newHotkeys(): HotkeyGroup {
    const group = new HotkeyGroup(this.index);

    this.groups.unshift(group);

    this.index++;
    return group;
  }

  unregister(toRemove: HotkeyGroup): void {
    const index = this.groups.findIndex((item) => item.id === toRemove.id);
    if (index !== -1) {
      this.groups.splice(index, 1);
    }
  }

  private async checkKey(event: KeyboardEvent, hotkey: Hotkey): Promise<boolean> {
    // Checking the platform
    if (!this.doPlatformsMatch(hotkey.platforms)) {
      return false;
    }
    if (event.key.toLowerCase() !== hotkey.key) {
      return false;
    }
    // Checking the current route (current displayed page)
    if (hotkey.route && !currentRouteIs(hotkey.route)) {
      return false;
    }
    // Disable the hotkey if a modal is opened
    if (hotkey.disableIfModal && (await modalController.getTop())) {
      return false;
    }
    if (!this.doModifiersMatch(event, hotkey.modifiers)) {
      return false;
    }
    event.preventDefault();
    await hotkey.callback();
    return true;
  }

  private async onKeyPress(event: KeyboardEvent, groups: HotkeyGroup[]): Promise<void> {
    if (!event.key || ['control', 'shift', 'alt'].includes(event.key.toLowerCase())) {
      return;
    }
    if (event.repeat) {
      return;
    }

    if (!isDesktop() && !isWeb()) {
      return;
    }

    for (const group of groups) {
      for (const hotkey of group.hotkeys) {
        if (await this.checkKey(event, hotkey)) {
          return;
        }
      }
    }
  }

  private doModifiersMatch(event: KeyboardEvent, modifiers: number): boolean {
    const ctrlKey = event.ctrlKey || (isMacOS() && event.metaKey) || (isWeb() && event.metaKey);
    let eventMods = 0;
    eventMods |= ctrlKey ? Modifiers.Ctrl : Modifiers.None;
    eventMods |= event.altKey ? Modifiers.Alt : Modifiers.None;
    eventMods |= event.shiftKey ? Modifiers.Shift : Modifiers.None;
    return eventMods === modifiers;
  }

  private doPlatformsMatch(platforms: number): boolean {
    return ((platforms & Platforms.Windows && isWindows()) ||
      (platforms & Platforms.Linux && isLinux()) ||
      (platforms & Platforms.MacOS && isMacOS()) ||
      (platforms & Platforms.Web && isWeb()) ||
      (platforms & Platforms.Mobile && isMobile())) as boolean;
  }
}
