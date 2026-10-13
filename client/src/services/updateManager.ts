// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { listAvailableDevices } from '@/parsec';
import { APP_VERSION } from '@/services/environment';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { inject, ref, Ref } from 'vue';

const updatePromptSuppressed: Ref<boolean> = ref(false);

// Returns the `X.Y` part of a version (`v3.5.1-a.0+dev` -> `3.5`), or `undefined` if it can't be parsed
export function getMajorMinorVersion(version: string): string | undefined {
  const match = version.trim().match(/^v?(\d+)\.(\d+)/);
  return match ? `${match[1]}.${match[2]}` : undefined;
}

const APP_MAJOR_MINOR_VERSION = getMajorMinorVersion(APP_VERSION);

export function useUpdateManager() {
  const storageManager: StorageManager = inject(StorageManagerKey)!;

  function isUpdatePromptAllowed(): boolean {
    return !updatePromptSuppressed.value;
  }

  function suppressUpdatePrompt(): void {
    updatePromptSuppressed.value = true;
  }

  async function hasAppBeenUpdated(ignorePatches = true): Promise<boolean> {
    const config = await storageManager.retrieveConfig();

    if (!config.appVersionLastVisit) {
      // Users from before `appVersionLastVisit` existed won't have it set, but we consider
      // them as returning users if they already have a device in this browser.
      // Can be removed in later releases once most users have `appVersionLastVisit` stored.
      const hasDevices = (await listAvailableDevices(false)).length > 0;
      // Storing immediately in this case, we won't bother them on subsequent refreshes.
      await storeAppVersion();
      return hasDevices;
    }
    if (ignorePatches) {
      const lastMajorMinor = getMajorMinorVersion(config.appVersionLastVisit);
      if (lastMajorMinor && APP_MAJOR_MINOR_VERSION) {
        return lastMajorMinor !== APP_MAJOR_MINOR_VERSION;
      }
    }
    return config.appVersionLastVisit !== APP_VERSION;
  }

  async function storeAppVersion(): Promise<void> {
    await storageManager.updateConfig({ appVersionLastVisit: APP_VERSION });
  }

  return {
    isUpdatePromptAllowed,
    suppressUpdatePrompt,
    hasAppBeenUpdated,
    storeAppVersion,
  };
}
