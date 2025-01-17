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
import { AccessStrategy, getLoggedInDevices, listAvailableDevices, login } from '@/parsec';
import { getConnectionHandle, navigateTo, Routes } from '@/router';

const injectionProvider: InjectionProvider = inject(InjectionProviderKey)!;
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

  // When in dev mode, we often open directly a connected page,
  // so a few states are not properly set.
  if (import.meta.env.PARSEC_APP_TESTBED_SERVER && (await getLoggedInDevices()).length === 0) {
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
  }
  initialized.value = true;
});
</script>
