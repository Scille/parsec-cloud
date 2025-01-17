<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page v-if="initialized">
    <ion-router-outlet />
  </ion-page>
</template>

<script setup lang="ts">
import { inject, onMounted, ref } from 'vue';
import { IonPage, IonRouterOutlet } from '@ionic/vue';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';
import { EventDistributor } from '@/services/eventDistributor';
import {
  AccessStrategy,
  closeFile,
  createWorkspace,
  getLoggedInDevices,
  listAvailableDevices,
  login,
  openFile,
  startWorkspace,
  writeFile,
} from '@/parsec';
import { getConnectionHandle, navigateTo, Routes } from '@/router';
import { StorageManagerKey, StorageManager } from '@/services/storageManager';

const injectionProvider: InjectionProvider = inject(InjectionProviderKey)!;
const storageManager: StorageManager = inject(StorageManagerKey)!;
const initialized = ref(false);
const DEV_DEFAULT_HANDLE = 1;

onMounted(async () => {
  initialized.value = false;
  const handle = getConnectionHandle();

  if (!handle) {
    window.electronAPI.log('error', 'Failed to retrieve connection handle while logged in');
    await navigateTo(Routes.Home, { replace: true, skipHandle: true });
    return;
  }

  if (!import.meta.env.PARSEC_APP_TESTBED_SERVER || (await getLoggedInDevices()).length !== 0) {
    initialized.value = true;
    return;
  }

  // When in dev mode, we often open directly a connected page,
  // so a few states are not properly set.
  if (handle !== DEV_DEFAULT_HANDLE) {
    window.electronAPI.log('error', `In dev mode, you should use "${DEV_DEFAULT_HANDLE}" as the default handle`);
    // eslint-disable-next-line no-alert
    alert(`Use "${DEV_DEFAULT_HANDLE}" as the default handle when not connecting properly`);
    return;
  }
  window.electronAPI.log('info', 'Page was refreshed, login in a default device');
  injectionProvider.createNewInjections(DEV_DEFAULT_HANDLE, new EventDistributor());
  const devices = await listAvailableDevices();
  let device = devices.find((d) => d.humanHandle.label === 'Alicey McAliceFace');

  if (!device) {
    device = devices[0];
    window.electronAPI.log('error', `Could not find Alice device, using ${device.humanHandle.label}`);
  }

  const result = await login(
    injectionProvider.getInjections(DEV_DEFAULT_HANDLE).eventDistributor,
    device,
    AccessStrategy.usePassword(device, 'P@ssw0rd.'),
  );
  if (!result.ok) {
    window.electronAPI.log('error', `Failed to log in on a default device: ${JSON.stringify(result.error)}`);
  } else if (result.value !== DEV_DEFAULT_HANDLE) {
    window.electronAPI.log('error', `Lib returned handle ${result.value} instead of ${DEV_DEFAULT_HANDLE}`);
  } else {
    window.electronAPI.log('info', `Logged in as ${device.humanHandle.label}`);
  }

  if (import.meta.env.PARSEC_APP_CLEAR_CACHE === 'true') {
    await storageManager.clearAll();
  }
  if (import.meta.env.PARSEC_APP_CREATE_DEFAULT_WORKSPACES === 'true') {
    await populate();
  }

  initialized.value = true;
});

async function populate(): Promise<void> {
  // Avoid importing files if unnecessary
  const mockFiles = (await import('@/parsec/mock_files')).MockFiles;

  window.electronAPI.log('debug', 'Creating mock workspaces and files');
  for (const workspaceName of ['The Copper Coronet', 'Trademeet', "Watcher's Keep"]) {
    const wkResult = await createWorkspace(workspaceName);
    if (!wkResult.ok) {
      window.electronAPI.log('error', `Could not create dev workspace ${workspaceName}`);
      continue;
    }
    const startWkResult = await startWorkspace(wkResult.value);
    if (!startWkResult.ok) {
      window.electronAPI.log('error', `Could not start dev workspace ${workspaceName}`);
      continue;
    }
    for (const fileType in mockFiles) {
      console.log(workspaceName, fileType);
      const fileName = `document_${fileType}.${fileType.toLocaleLowerCase()}`;
      const openResult = await openFile(startWkResult.value, `/${fileName}`, { write: true, truncate: true, create: true });

      if (!openResult.ok) {
        window.electronAPI.log('error', `Could not open file ${fileName}`);
        continue;
      }
      const writeResult = await writeFile(startWkResult.value, openResult.value, 0, mockFiles[fileType as keyof typeof mockFiles]);
      if (!writeResult.ok) {
        window.electronAPI.log('error', `Failed to write file ${fileName}`);
        continue;
      }
      await closeFile(startWkResult.value, openResult.value);
    }
  }
}
</script>
