// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { SIDEBAR_MENU_DATA_KEY, SidebarDefaultData, SidebarSavedData } from '@/views/menu';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { nextTick } from 'vue';

// Mock Vue's inject function
vi.mock('vue', async () => {
  const actual = await vi.importActual('vue');
  return {
    ...actual,
    inject: vi.fn(),
  };
});

// Mock the storage manager
const mockStorageManager = {
  retrieveComponentData: vi.fn(),
  updateComponentData: vi.fn(),
};

async function waitForInitialization(): Promise<void> {
  await new Promise((resolve) => setTimeout(resolve, 0));
  await nextTick();
}

describe('useSidebarMenu', () => {
  beforeEach(async () => {
    vi.clearAllMocks();
    const { inject } = await import('vue');
    vi.mocked(inject).mockReturnValue(mockStorageManager);

    // Provide default mock behavior
    mockStorageManager.retrieveComponentData.mockResolvedValue(SidebarDefaultData);
    mockStorageManager.updateComponentData.mockResolvedValue(undefined);

    // Reset service global state by reimporting the module
    await vi.resetModules();
  });

  describe('initialization', () => {
    it('should start with default values before loading', async () => {
      const { default: useSidebarMenu } = await import('@/services/sidebarMenu');
      const sidebar = useSidebarMenu();

      expect(sidebar.width.value).toBe(300); // DEFAULT_WIDTH
      expect(sidebar.isVisible.value).toBe(false); // Hidden during loading
    });

    it('should load visible state from storage', async () => {
      const savedData: SidebarSavedData = {
        width: 400,
        visible: true,
      };
      mockStorageManager.retrieveComponentData.mockResolvedValue(savedData);

      const { default: useSidebarMenu } = await import('@/services/sidebarMenu');
      const sidebar = useSidebarMenu();
      await waitForInitialization();

      expect(sidebar.width.value).toBe(400);
      expect(sidebar.isVisible.value).toBe(true);
    });

    it('should load hidden state from storage', async () => {
      const savedData: SidebarSavedData = {
        width: 350,
        visible: false,
      };
      mockStorageManager.retrieveComponentData.mockResolvedValue(savedData);

      const { default: useSidebarMenu } = await import('@/services/sidebarMenu');
      const sidebar = useSidebarMenu();
      await waitForInitialization();

      expect(sidebar.width.value).toBe(0); // HIDDEN_WIDTH
      expect(sidebar.isVisible.value).toBe(false);
    });

    it('should handle storage errors gracefully', async () => {
      mockStorageManager.retrieveComponentData.mockRejectedValue(new Error('Storage error'));
      const consoleSpy = vi.spyOn(console, 'warn').mockImplementation(() => {});

      const { default: useSidebarMenu } = await import('@/services/sidebarMenu');
      const sidebar = useSidebarMenu();
      await waitForInitialization();

      expect(sidebar.width.value).toBe(300); // DEFAULT_WIDTH
      expect(sidebar.isVisible.value).toBe(true);
      expect(consoleSpy).toHaveBeenCalledWith('Failed to load sidebar state from storage:', expect.any(Error));

      consoleSpy.mockRestore();
    });
  });

  describe('hide function', () => {
    it('should hide visible sidebar and persist by default', async () => {
      const savedData: SidebarSavedData = { width: 400, visible: true };
      mockStorageManager.retrieveComponentData.mockResolvedValue(savedData);
      mockStorageManager.updateComponentData.mockResolvedValue(undefined);

      const { default: useSidebarMenu } = await import('@/services/sidebarMenu');
      const sidebar = useSidebarMenu();
      await waitForInitialization();

      expect(sidebar.isVisible.value).toBe(true);

      sidebar.hide();

      expect(sidebar.width.value).toBe(0);
      expect(sidebar.isVisible.value).toBe(false);
      expect(mockStorageManager.updateComponentData).toHaveBeenCalledWith(
        SIDEBAR_MENU_DATA_KEY,
        { width: 400, visible: false },
        SidebarDefaultData,
      );
    });

    it('should hide without persisting when persist=false', async () => {
      const savedData: SidebarSavedData = { width: 400, visible: true };
      mockStorageManager.retrieveComponentData.mockResolvedValue(savedData);

      const { default: useSidebarMenu } = await import('@/services/sidebarMenu');
      const sidebar = useSidebarMenu();
      await waitForInitialization();

      sidebar.hide(false);

      expect(sidebar.isVisible.value).toBe(false);
      expect(mockStorageManager.updateComponentData).not.toHaveBeenCalled();
    });

    it('should do nothing if already hidden', async () => {
      const savedData: SidebarSavedData = { width: 400, visible: false };
      mockStorageManager.retrieveComponentData.mockResolvedValue(savedData);

      const { default: useSidebarMenu } = await import('@/services/sidebarMenu');
      const sidebar = useSidebarMenu();
      await waitForInitialization();

      expect(sidebar.isVisible.value).toBe(false);

      sidebar.hide();

      expect(mockStorageManager.updateComponentData).not.toHaveBeenCalled();
    });
  });

  describe('show function', () => {
    it('should show hidden sidebar with stored width', async () => {
      const savedData: SidebarSavedData = { width: 450, visible: false };
      mockStorageManager.retrieveComponentData.mockResolvedValue(savedData);
      mockStorageManager.updateComponentData.mockResolvedValue(undefined);

      const { default: useSidebarMenu } = await import('@/services/sidebarMenu');
      const sidebar = useSidebarMenu();
      await waitForInitialization();

      expect(sidebar.isVisible.value).toBe(false);

      sidebar.show();

      expect(sidebar.width.value).toBe(450);
      expect(sidebar.isVisible.value).toBe(true);
      expect(mockStorageManager.updateComponentData).toHaveBeenCalledWith(
        SIDEBAR_MENU_DATA_KEY,
        { width: 450, visible: true },
        SidebarDefaultData,
      );
    });
  });

  describe('loading state behavior', () => {
    it('should show isVisible as false during loading', async () => {
      mockStorageManager.retrieveComponentData.mockImplementation(
        () => new Promise((resolve) => setTimeout(() => resolve({ width: 400, visible: true }), 100)),
      );

      const { default: useSidebarMenu } = await import('@/services/sidebarMenu');
      const sidebar = useSidebarMenu();

      // During loading
      expect(sidebar.isVisible.value).toBe(false);
    });

    it('should update isVisible after loading completes', async () => {
      const savedData: SidebarSavedData = { width: 400, visible: true };
      mockStorageManager.retrieveComponentData.mockResolvedValue(savedData);

      const { default: useSidebarMenu } = await import('@/services/sidebarMenu');
      const sidebar = useSidebarMenu();

      // During loading
      expect(sidebar.isVisible.value).toBe(false);

      // After loading
      await waitForInitialization();

      expect(sidebar.isVisible.value).toBe(true);
    });
  });
});
