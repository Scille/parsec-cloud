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
import { AccessStrategy, closeFile, getLoggedInDevices, listAvailableDevices, listWorkspaces, login, openFile, writeFile } from '@/parsec';
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

  if (
    !import.meta.env.PARSEC_APP_TESTBED_SERVER ||
    (await getLoggedInDevices()).length !== 0 ||
    import.meta.env.PARSEC_APP_TESTBED_AUTO_LOGIN !== 'true'
  ) {
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
  // Not filtering the devices, because we need alice first device, not the second one
  const devices = await listAvailableDevices(false);
  let device = devices.find((d) => d.humanHandle.label === 'Alicey McAliceFace' && d.deviceLabel.includes('dev1'));

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
  if (import.meta.env.PARSEC_APP_POPULATE_DEFAULT_WORKSPACE === 'true') {
    await populate();
  }

  initialized.value = true;
});

async function populate(): Promise<void> {
  // Avoid importing files if unnecessary
  const mockFiles = await import('@/parsec/mock_files');

  window.electronAPI.log('debug', 'Creating mock files');
  const workspaces = await listWorkspaces(getConnectionHandle());
  if (!workspaces.ok) {
    window.electronAPI.log('error', 'Failed to list workspaces');
    return;
  }
  for (const workspace of workspaces.value) {
    for (const fileType in mockFiles.MockFileType) {
      console.log(workspace.currentName, fileType);
      const fileName = `document_${fileType}.${fileType.toLocaleLowerCase()}`;
      const openResult = await openFile(workspace.handle, `/${fileName}`, { write: true, truncate: true, create: true });

      if (!openResult.ok) {
        window.electronAPI.log('error', `Could not open file ${fileName}`);
        continue;
      }
      const content = await mockFiles.getMockFileContent(fileType as any);
      const writeResult = await writeFile(workspace.handle, openResult.value, 0, content);
      if (!writeResult.ok) {
        window.electronAPI.log('error', `Failed to write file ${fileName}`);
        continue;
      }
      await closeFile(workspace.handle, openResult.value);
    }
  }
}
</script>
