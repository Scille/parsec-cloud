// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

import { menuController } from '@ionic/vue';
import { Component, shallowRef } from 'vue';

interface MenuComponent {
  component: Component;
  props?: Record<string, unknown>;
}

const MENU_ID = 'side-popover-menu';
const currentMenu = shallowRef<MenuComponent | undefined>(undefined);

export function useSidePopoverMenu() {
  async function open(options: MenuComponent): Promise<void> {
    if (currentMenu.value) {
      window.nativeAPI.log('warn', 'A menu is already opened');
      return;
    }
    currentMenu.value = { component: options.component, props: options.props };
    const result = await menuController.open(MENU_ID);
    if (!result) {
      currentMenu.value = undefined;
    }
  }

  async function close(): Promise<void> {
    if (await menuController.isOpen(MENU_ID)) {
      await menuController.close(MENU_ID);
    }
    currentMenu.value = undefined;
  }

  async function _onClose(): Promise<void> {
    currentMenu.value = undefined;
  }

  return {
    open,
    close,
    _onClose,
    MENU_ID,
    currentMenu,
  };
}
