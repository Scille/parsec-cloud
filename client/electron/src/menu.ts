// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Menu, MenuItem, MenuItemConstructorOptions } from 'electron';
import { WindowToPageChannel } from './communicationChannels.js';

// Keep in sync with src/common/menu.ts
export interface ParsecMenuAction {
  type: 'action';
  label: string;
  accelerator?: string;
  action?: string;
  children?: ParsecMenuItem[];
  disabled?: boolean;
  hidden?: boolean;
}

export interface ParsecMenuSeparator {
  type: 'separator';
}

export type ParsecMenuItem = ParsecMenuAction | ParsecMenuSeparator;

export function setupAppMenu(app: any, items: ParsecMenuItem[]): void {
  if (!items) {
    Menu.setApplicationMenu(null);
  }
  const menu = new Menu();

  function buildMenu(items: ParsecMenuItem[]): MenuItemConstructorOptions[] {
    const menuItems: MenuItemConstructorOptions[] = [];
    for (const item of items) {
      if (item.type === 'separator') {
        menuItems.push({ type: 'separator' });
      } else {
        menuItems.push({
          label: item.label,
          accelerator: item.accelerator,
          role: item.action === undefined || item.action.startsWith('parsec-') ? undefined : (item.action as any),
          click:
            item.action !== undefined && item.action.startsWith('parsec-')
              ? () => {
                  item.action === 'parsec-close-app'
                    ? app.sendEvent(WindowToPageChannel.CloseRequest)
                    : app.sendEvent(WindowToPageChannel.MenuActionClicked, item.action);
                }
              : undefined,
          enabled: !(item.disabled === true),
          submenu: item.children ? buildMenu(item.children) : undefined,
          visible: !(item.hidden === true),
        });
      }
    }
    return menuItems;
  }

  for (const item of buildMenu(items)) {
    menu.append(new MenuItem(item));
  }
  Menu.setApplicationMenu(menu);
}
