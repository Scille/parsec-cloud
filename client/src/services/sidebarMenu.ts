// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { SIDEBAR_MENU_DATA_KEY, SidebarDefaultData, SidebarSavedData } from '@/views/menu';
import { computed, ComputedRef, inject, Ref, ref } from 'vue';

const DEFAULT_WIDTH = 300;
const HIDDEN_WIDTH = 0;
const computedWidth = ref<number>(DEFAULT_WIDTH);
const storedWidth = ref<number>(DEFAULT_WIDTH);
const isInitialized = ref<boolean>(false);
const isVisible = computed(() => {
  return !isInitialized.value ? false : computedWidth.value > HIDDEN_WIDTH;
});

interface SidebarMenu {
  width: Ref<number>;
  isVisible: ComputedRef<boolean>;
  reset: (persist?: boolean) => void;
  hide: (persist?: boolean) => void;
  show: (persist?: boolean) => void;
  setWidth: (width: number, persist?: boolean) => void;
}

export default function useSidebarMenu(): SidebarMenu {
  const storageManager: StorageManager | null = inject(StorageManagerKey, null);

  // Start async initialization
  init()
    .then()
    .catch((error) => {
      console.warn('Failed to initialize sidebar menu:', error);
    })
    .finally(() => {
      isInitialized.value = true;
    });

  async function init(): Promise<void> {
    if (!storageManager) {
      return;
    }

    try {
      const savedData = await storageManager.retrieveComponentData<SidebarSavedData>(SIDEBAR_MENU_DATA_KEY, SidebarDefaultData);

      const savedWidth = savedData.width || DEFAULT_WIDTH;
      const isVisible = savedData.visible !== undefined ? savedData.visible : true; // Default to visible

      if (isVisible) {
        computedWidth.value = savedWidth;
        storedWidth.value = savedWidth;
      } else {
        computedWidth.value = HIDDEN_WIDTH;
        storedWidth.value = savedWidth;
      }
    } catch (error) {
      console.warn('Failed to load sidebar state from storage:', error);
      // Default to visible state with default width on storage errors
      computedWidth.value = DEFAULT_WIDTH;
      storedWidth.value = DEFAULT_WIDTH;
    }
  }

  async function _saveToStorage(): Promise<void> {
    if (!storageManager) {
      return;
    }

    try {
      await storageManager.updateComponentData<SidebarSavedData>(
        SIDEBAR_MENU_DATA_KEY,
        {
          width: computedWidth.value <= HIDDEN_WIDTH ? storedWidth.value : computedWidth.value,
          visible: isVisible.value,
        },
        SidebarDefaultData,
      );
    } catch (error) {
      console.warn('Failed to save sidebar state to storage:', error);
    }
  }

  function reset(persist = true): void {
    computedWidth.value = DEFAULT_WIDTH;
    storedWidth.value = DEFAULT_WIDTH;
    if (persist) {
      _saveToStorage();
    }
  }

  function hide(persist = true): void {
    if (isVisible.value) {
      storedWidth.value = computedWidth.value;
      computedWidth.value = HIDDEN_WIDTH;
      if (persist) {
        _saveToStorage();
      }
    }
  }

  function show(persist = true): void {
    computedWidth.value = storedWidth.value;
    if (persist) {
      _saveToStorage();
    }
  }

  function setWidth(width: number, persist = true): void {
    computedWidth.value = width;
    if (persist) {
      _saveToStorage();
    }
  }

  return {
    width: computedWidth,
    isVisible,
    reset,
    hide,
    show,
    setWidth,
  };
}
