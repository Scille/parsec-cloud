<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <!-- Don't load children components before we inject everything -->
  <ion-page v-if="initialized">
    <ion-router-outlet />
  </ion-page>
</template>

<script lang="ts" setup>
import { getConnectionInfo, getTOS, logout as parsecLogout, acceptTOS, getClientInfo } from '@/parsec';
import { getConnectionHandle, navigateTo, Routes } from '@/router';
import { EventData, EventDistributorKey, Events } from '@/services/eventDistributor';
import { FileOperationManagerKey } from '@/services/fileOperationManager';
import { Information, InformationLevel, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { InjectionProvider, InjectionProviderKey, Injections } from '@/services/injectionProvider';
import { IonPage, IonRouterOutlet, modalController } from '@ionic/vue';
import { inject, onMounted, onUnmounted, provide, Ref, ref } from 'vue';
import TOSModal from '@/views/organizations/TOSModal.vue';
import useUploadMenu from '@/services/fileUploadMenu';
import { MsModalResult, openSpinnerModal } from 'megashark-lib';
import { DateTime } from 'luxon';
import { StorageManagerKey, StorageManager } from '@/services/storageManager';
import { recentDocumentManager } from '@/services/recentDocuments';

const injectionProvider: InjectionProvider = inject(InjectionProviderKey)!;
const storageManager: StorageManager = inject(StorageManagerKey)!;
let injections: Injections;
const initialized = ref(false);
const modalOpened = ref(false);
let timeoutId: number | null = null;
let callbackId: string | null = null;
const lastAccepted: Ref<DateTime | null> = ref(null);

onMounted(async () => {
  const handle = getConnectionHandle();

  // No handle
  if (!handle) {
    window.electronAPI.log('error', 'Failed to retrieve connection handle while logged in');
    await navigateTo(Routes.Home, { replace: true, skipHandle: true });
    return;
  }

  // Vue wants `provide` to be the absolute first thing.
  injections = injectionProvider.getInjections(handle);

  // Provide the injections to children
  provide(FileOperationManagerKey, injections.fileOperationManager);
  provide(InformationManagerKey, injections.informationManager);
  provide(EventDistributorKey, injections.eventDistributor);

  const clientInfoResult = await getClientInfo(handle);
  // The handle is invalid
  if (!clientInfoResult.ok) {
    window.electronAPI.log('error', `Failed to retrieve client info: ${clientInfoResult.error.tag} (${clientInfoResult.error.error})`);
    await navigateTo(Routes.Home, { replace: true, skipHandle: true });
    return;
  }

  callbackId = await injections.eventDistributor.registerCallback(
    Events.TOSAcceptRequired | Events.LogoutRequested,
    async (event: Events, _data?: EventData) => {
      if (event === Events.LogoutRequested) {
        await logout();
      } else if (event === Events.TOSAcceptRequired) {
        await tryOpeningTOSModal();
      }
    },
  );

  recentDocumentManager.resetHistory();
  await recentDocumentManager.loadFromStorage(storageManager, handle);

  initialized.value = true;

  // Making sure we get a notification as soon as possible
  window.electronAPI.getUpdateAvailability();

  const connInfo = getConnectionInfo();
  if (connInfo && connInfo.shouldAcceptTos) {
    await showTOSModal();
  }
});

onUnmounted(async () => {
  if (timeoutId !== null) {
    window.clearTimeout(timeoutId);
    timeoutId = null;
  }
  if (injections && callbackId !== null) {
    injections.eventDistributor.removeCallback(callbackId);
  }
});

async function tryOpeningTOSModal(): Promise<void> {
  if (modalOpened.value) {
    return;
  }
  if ((await modalController.getTop()) || injections.fileOperationManager.hasOperations()) {
    if (timeoutId !== null) {
      window.clearTimeout(timeoutId);
    }
    // Try again in 10 seconds
    timeoutId = window.setTimeout(async () => {
      await tryOpeningTOSModal();
    }, 10000);
  } else {
    if (timeoutId) {
      window.clearInterval(timeoutId);
      timeoutId = null;
    }
    await showTOSModal();
  }
}

async function logout(): Promise<void> {
  const modal = await openSpinnerModal('HomePage.topbar.logoutWait');
  const menuCtrls = useUploadMenu();
  menuCtrls.hide();

  const handle = getConnectionHandle();
  if (!handle) {
    window.electronAPI.log('error', 'No handle found when trying to log out');
  } else {
    const connInfo = getConnectionInfo(handle);

    if (connInfo) {
      const storedDeviceDataDict = await storageManager.retrieveDevicesData();
      if (!storedDeviceDataDict[connInfo.device.deviceId]) {
        storedDeviceDataDict[connInfo.device.deviceId] = {
          lastLogin: DateTime.now(),
        };
      } else {
        storedDeviceDataDict[connInfo.device.deviceId].lastLogin = DateTime.now();
      }
      await storageManager.storeDevicesData(storedDeviceDataDict);
    }

    // Cleaning the injections will automatically cancel the imports
    await injectionProvider.clean(handle);
    const logoutResult = await parsecLogout();
    if (!logoutResult.ok) {
      window.electronAPI.log('error', `Error when logging out: ${JSON.stringify(logoutResult.error)}`);
    }
  }
  recentDocumentManager.resetHistory();

  await modal.dismiss();
  await navigateTo(Routes.Home, { replace: true, skipHandle: true });
}

async function showTOSModal(): Promise<void> {
  const result = await getTOS();

  if (!result.ok) {
    return;
  }
  if (result.value.perLocaleUrls.size === 0) {
    window.electronAPI.log('warn', 'Received empty Terms of Service dictionary');
    return;
  }
  if (result.value.updatedOn.toMillis() === lastAccepted.value?.toMillis()) {
    window.electronAPI.log('warn', 'Already accepted those TOS');
    return;
  }
  modalOpened.value = true;
  const tosModal = await modalController.create({
    component: TOSModal,
    cssClass: 'modal-tos',
    componentProps: {
      tosLinks: result.value.perLocaleUrls,
    },
    canDismiss: true,
    backdropDismiss: false,
    showBackdrop: true,
  });
  await tosModal.present();
  const { role } = await tosModal.onDidDismiss();
  await tosModal.dismiss();

  if (role === MsModalResult.Confirm) {
    const acceptResult = await acceptTOS(result.value.updatedOn);
    if (acceptResult.ok) {
      const connInfo = getConnectionInfo();
      if (connInfo) {
        connInfo.shouldAcceptTos = false;
      }
      lastAccepted.value = result.value.updatedOn;
      injections.informationManager.present(
        new Information({
          message: 'CreateOrganization.acceptTOS.update.confirmationMessage',
          level: InformationLevel.Info,
        }),
        PresentationMode.Toast,
      );
      modalOpened.value = false;
      // Early return here. If the user didn't accept the TOS or the acception fails,
      // we will log them out.
      return;
    } else {
      window.electronAPI.log('error', `Error when accepting the TOS: ${JSON.stringify(acceptResult.error)}`);
      injections.informationManager.present(
        new Information({
          message: 'CreateOrganization.acceptTOS.update.acceptError',
          level: InformationLevel.Info,
        }),
        PresentationMode.Toast,
      );
    }
  }
  modalOpened.value = false;
  await logout();
}
</script>
