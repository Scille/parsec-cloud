// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { isDesktop, isLinux, isMacOS, isMobile, isWeb, isWindows } from '@/parsec/environment';

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

export interface Hotkey {
  key: string;
  modifiers: number;
  platforms: number;
  command: () => Promise<void>;
}

class Hotkeys {
  currentHotkeys: Hotkey[];
  id: number;
  enabled: boolean;

  constructor(id: number) {
    this.currentHotkeys = [];
    this.id = id;
    this.enabled = true;
  }

  add(key: string, modifiers: number, platforms: number, command: () => Promise<void>): void {
    this.currentHotkeys.push({ key: key, modifiers: modifiers, platforms: platforms, command: command });
  }

  enable(): void {
    this.enabled = true;
  }

  disable(): void {
    this.enabled = false;
  }
}

export class HotkeyManager {
  currentHotkeys: Hotkeys[];
  index: number;

  constructor() {
    this.currentHotkeys = [];
    this.index = 0;
    window.addEventListener('keydown', async (event: KeyboardEvent): Promise<void> => await this.onKeyPress(event, this.currentHotkeys));
  }

  newHotkeys(): Hotkeys {
    const newKeys = new Hotkeys(this.index);
    this.index++;
    this.currentHotkeys.push(newKeys);
    return newKeys;
  }

  unregister(toRemove: Hotkeys): void {
    this.currentHotkeys = this.currentHotkeys.filter((hotkeys) => hotkeys.id !== toRemove.id);
  }

  private async onKeyPress(event: KeyboardEvent, hotKeys: Array<Hotkeys>): Promise<void> {
    if (!isDesktop() && !isWeb()) {
      return;
    }
    for (const keyPresses of hotKeys) {
      for (const keyPress of keyPresses.currentHotkeys) {
        if (!this.doPlatformsMatch(keyPress.platforms)) {
          continue;
        }
        if (event.key.toLowerCase() === keyPress.key) {
          if (!this.doModifiersMatch(event, keyPress.modifiers)) {
            continue;
          }
          event.preventDefault();
          if (keyPresses.enabled) {
            await keyPress.command();
          }
        }
      }
    }
  }

  private doModifiersMatch(event: KeyboardEvent, modifiers: number): boolean {
    const ctrlKey = isMacOS() ? event.metaKey : isDesktop() ? event.ctrlKey : event.ctrlKey || event.metaKey;
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
