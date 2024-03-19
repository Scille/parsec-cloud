// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { isDesktop, isLinux, isMacOS, isMobile, isWeb, isWindows, needsMocks } from '@/parsec/environment';

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

export enum Groups {
  Home = 1,
  Workspaces,
  Documents,
  Users,
  Global,
  Unique,
}

interface Hotkey {
  key: string;
  modifiers: number;
  platforms: number;
  callback: () => Promise<void>;
}

export class Hotkeys {
  currentHotkeys: Hotkey[];
  id: number;
  group: Groups;

  constructor(id: number, group: Groups) {
    this.currentHotkeys = [];
    this.id = id;
    this.group = group;
  }

  add(key: string, modifiers: number, platforms: number, callback: () => Promise<void>): void {
    if (needsMocks()) {
      platforms |= Platforms.Web;
    }
    this.currentHotkeys.push({ key: key, modifiers: modifiers, platforms: platforms, callback: callback });
  }
}

interface HotkeyGroup {
  group: Groups;
  hotkeys: Hotkeys[];
  disabledCount: number;
}

export class HotkeyManager {
  groups: HotkeyGroup[];
  uniqueGroup: Hotkeys[];
  index: number;

  constructor() {
    this.groups = [];
    this.uniqueGroup = [];
    this.index = 0;
    window.addEventListener(
      'keydown',
      async (event: KeyboardEvent): Promise<void> => await this.onKeyPress(event, this.groups, this.uniqueGroup),
    );
  }

  newHotkeys(group: Groups): Hotkeys {
    const keys = new Hotkeys(this.index, group);

    if (group === Groups.Unique) {
      this.uniqueGroup.unshift(keys);
    } else {
      let hkGroup = this.groups.find((item) => item.group === group);

      if (!hkGroup) {
        hkGroup = { group: group, hotkeys: [], disabledCount: 0 };
        this.groups.unshift(hkGroup);
      }
      // Insert at the beginning so it will have the priority
      hkGroup.hotkeys.unshift(keys);
    }

    this.index++;
    return keys;
  }

  unregister(toRemove: Hotkeys): void {
    if (toRemove.group === Groups.Unique) {
      this.uniqueGroup = this.uniqueGroup.filter((item) => item.id !== toRemove.id);
    } else {
      const hkGroup = this.groups.find((item) => item.group === toRemove.group);

      if (!hkGroup) {
        return;
      }
      hkGroup.hotkeys = hkGroup.hotkeys.filter((item) => item.id !== toRemove.id);
    }
  }

  enableGroup(group: Groups): void {
    const hkGroup = this.groups.find((item) => item.group === group);

    if (hkGroup) {
      hkGroup.disabledCount = hkGroup.disabledCount < 1 ? 0 : hkGroup.disabledCount - 1;
    }
  }

  disableGroup(group: Groups): void {
    const hkGroup = this.groups.find((item) => item.group === group);

    if (hkGroup) {
      hkGroup.disabledCount += 1;
    }
  }

  private async checkKey(event: KeyboardEvent, groupIsDisabled: boolean, key: Hotkey): Promise<boolean> {
    if (!this.doPlatformsMatch(key.platforms)) {
      return false;
    }
    if (event.key.toLowerCase() === key.key) {
      if (!this.doModifiersMatch(event, key.modifiers)) {
        return false;
      }
      event.preventDefault();
      // Only checking if the group is disabled here,
      // because we still want to prevent the default behavior
      // if we handle this hotkey
      if (groupIsDisabled) {
        return false;
      }
      await key.callback();
      return true;
    }
    return false;
  }

  private async onKeyPress(event: KeyboardEvent, groups: HotkeyGroup[], uniqueGroup: Hotkeys[]): Promise<void> {
    if (!event.key || ['control', 'shift', 'alt'].includes(event.key.toLowerCase())) {
      return;
    }
    if (['control', 'shift', 'alt'].includes(event.key.toLowerCase())) {
      return;
    }
    if (event.repeat) {
      return;
    }

    if (!isDesktop() && !isWeb()) {
      return;
    }
    // Unique take priority
    for (const keyPresses of uniqueGroup) {
      for (const keyPress of keyPresses.currentHotkeys) {
        await this.checkKey(event, false, keyPress);
      }
    }

    for (const group of groups) {
      for (const keyPresses of group.hotkeys) {
        for (const keyPress of keyPresses.currentHotkeys) {
          if (await this.checkKey(event, group.disabledCount > 0, keyPress)) {
            return;
          }
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
