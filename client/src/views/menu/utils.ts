// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Ref, ref } from 'vue';

export interface SidebarSavedData {
  width?: number;
  visible?: boolean;
  organizationVisible?: boolean;
  workspacesVisible?: boolean;
  favoritesVisible?: boolean;
  recentWorkspacesVisible?: boolean;
  recentFilesVisible?: boolean;
}

export const SIDEBAR_MENU_DATA_KEY = 'SidebarMenu';
export const SIDEBAR_CLIENTAREA_MENU_DATA_KEY = 'SidebarClientAreaMenu';

export const SidebarDefaultData: Required<SidebarSavedData> = {
  width: 300,
  visible: true,
  organizationVisible: true,
  workspacesVisible: true,
  favoritesVisible: true,
  recentWorkspacesVisible: true,
  recentFilesVisible: true,
};

interface CustomTabBarOptions {
  show: () => void;
  hide: () => void;
  isVisible: Ref<boolean>;
}

const visible = ref(false);

export function useCustomTabBar(): CustomTabBarOptions {
  const show = (): void => {
    visible.value = true;
  };

  const hide = (): void => {
    visible.value = false;
  };

  return {
    show: show,
    hide: hide,
    isVisible: visible,
  };
}
