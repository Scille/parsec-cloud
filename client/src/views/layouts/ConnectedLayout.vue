<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <!-- Don't load children components before we inject everything -->
  <ion-page
    v-if="initialized"
    class="connected-layout"
  >
    <ion-router-outlet />
  </ion-page>
</template>

<script lang="ts" setup>
import {
  acceptTOS,
  archiveDevice,
  AvailableDevice,
  getClientInfo,
  getTOS,
  listAvailableDevices,
  listStartedClients,
  logout as parsecLogout,
} from '@/parsec';
import { getConnectionHandle, navigateTo, Routes } from '@/router';
import { APP_VERSION } from '@/services/environment';
import {
  EntryDeletedData,
  EntryRenamedData,
  EventData,
  EventDistributor,
  EventDistributorKey,
  Events,
  UpdateAvailabilityData,
} from '@/services/eventDistributor';
import { FileOperationManagerKey } from '@/services/fileOperationManager';
import useUploadMenu from '@/services/fileUploadMenu';
import { Information, InformationLevel, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { InjectionProvider, InjectionProviderKey, Injections } from '@/services/injectionProvider';
import { recentDocumentManager } from '@/services/recentDocuments';
import useRefreshWarning from '@/services/refreshWarning';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { useUpdateManager } from '@/services/updateManager';
import UpdateAppModal from '@/views/about/UpdateAppModal.vue';
import TOSModal from '@/views/organizations/TOSModal.vue';
import { IonPage, IonRouterOutlet, modalController } from '@ionic/vue';
import { DateTime } from 'luxon';
import { MsModalResult, openSpinnerModal } from 'megashark-lib';
import { inject, onMounted, onUnmounted, provide, Ref, ref } from 'vue';

const injectionProvider: InjectionProvider = inject(InjectionProviderKey)!;
const storageManager: StorageManager = inject(StorageManagerKey)!;
const { isUpdatePromptAllowed, suppressUpdatePrompt } = useUpdateManager();
let injections: Injections;
const initialized = ref(false);
const modalOpened = ref(false);
let timeoutId: number | null = null;
let callbackId: string | null = null;
const lastAccepted: Ref<DateTime | null> = ref(null);

const refreshWarning = useRefreshWarning();

onMounted(async () => {
  const handle = getConnectionHandle();

  // No handle
  if (!handle) {
    window.electronAPI.log('error', 'Failed to retrieve connection handle while logged in');
    await navigateTo(Routes.Home, { replace: true, skipHandle: true });
    return;
  }

  // Vue wants `provide` to be the absolute first thing.
  if (!injectionProvider.hasInjections(handle)) {
    const eventDistributor = new EventDistributor();
    injectionProvider.createNewInjections(handle, eventDistributor);
  }
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
    Events.TOSAcceptRequired |
      Events.LogoutRequested |
      Events.EntryDeleted |
      Events.EntryRenamed |
      Events.OrganizationNotFound |
      Events.IncompatibleServer |
      Events.ExpiredOrganization |
      Events.ClientRevoked |
      Events.ClientFrozen |
      Events.UpdateAvailability,
    eventCallback,
  );

  recentDocumentManager.resetHistory();
  await recentDocumentManager.loadFromStorage(storageManager, handle);

  initialized.value = true;

  // Making sure we get a notification as soon as possible
  window.electronAPI.getUpdateAvailability();

  if (clientInfoResult.value.mustAcceptTos) {
    await showTOSModal();
  }
  refreshWarning.warnOnRefresh();
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

async function eventCallback(event: Events, data?: EventData): Promise<void> {
  switch (event) {
    case Events.LogoutRequested:
      await logout();
      break;
    case Events.TOSAcceptRequired:
      await tryOpeningTOSModal();
      break;
    case Events.EntryDeleted:
      recentDocumentManager.removeFileById((data as EntryDeletedData).entryId);
      break;
    case Events.EntryRenamed:
      recentDocumentManager.updateFile((data as EntryRenamedData).entryId, {
        name: (data as EntryRenamedData).newName,
        path: (data as EntryRenamedData).newPath,
      });
      break;
    case Events.OrganizationNotFound:
      await injections.informationManager.present(
        new Information({
          message: 'globalErrors.organizationNotFound',
          level: InformationLevel.Error,
        }),
        PresentationMode.Modal,
      );
      break;
    case Events.IncompatibleServer:
      injections.informationManager.present(
        new Information({
          message: 'notification.incompatibleServer',
          level: InformationLevel.Error,
        }),
        PresentationMode.Notification,
      );
      await injections.informationManager.present(
        new Information({
          message: 'globalErrors.incompatibleServer',
          level: InformationLevel.Error,
        }),
        PresentationMode.Modal,
      );
      break;
    case Events.ExpiredOrganization:
      injections.informationManager.present(
        new Information({
          message: 'notification.expiredOrganization',
          level: InformationLevel.Error,
        }),
        PresentationMode.Notification,
      );
      await injections.informationManager.present(
        new Information({
          message: 'globalErrors.expiredOrganization',
          level: InformationLevel.Error,
        }),
        PresentationMode.Modal,
      );
      break;
    case Events.ClientRevoked:
      await injections.informationManager.present(
        new Information({
          message: 'globalErrors.clientRevoked',
          level: InformationLevel.Error,
        }),
        PresentationMode.Modal,
      );
      const clientInfo = await getClientInfo();
      let currentDevices: Array<AvailableDevice> = [];
      if (clientInfo.ok) {
        currentDevices = (await listAvailableDevices(false)).filter((device) => device.userId === clientInfo.value.userId);
      }
      for (const device of currentDevices) {
        await archiveDevice(device);
      }
      await logout();
      break;
    case Events.ClientFrozen:
      await injections.informationManager.present(
        new Information({
          message: 'globalErrors.clientFrozen',
          level: InformationLevel.Error,
        }),
        PresentationMode.Modal,
      );
      break;
    case Events.UpdateAvailability: {
      const updateData = data as UpdateAvailabilityData;
      await showUpdateModal(updateData);
      break;
    }
  }
}

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
    const startedClients = await listStartedClients();
    const deviceId = startedClients.find(([sHandle, _deviceId]) => sHandle === handle)?.[1];

    if (deviceId) {
      const storedDeviceDataDict = await storageManager.retrieveDevicesData();
      if (!storedDeviceDataDict[deviceId]) {
        storedDeviceDataDict[deviceId] = {
          lastLogin: DateTime.now(),
        };
      } else {
        storedDeviceDataDict[deviceId].lastLogin = DateTime.now();
      }
      await storageManager.storeDevicesData(storedDeviceDataDict);
    }

    // If there's only one client started (the current one), it's safe
    // to remove the warning. Things become a bit more complicated when there
    // are multiple started client because we don't know if another tab is
    // opened, and if that's not the case, refreshing the home page
    // would also mean losing those clients, so we want to keep the
    // warning.
    if (startedClients.length === 1) {
      refreshWarning.removeWarning();
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
      lastAccepted.value = result.value.updatedOn;
      injections.informationManager.present(
        new Information({
          message: 'CreateOrganization.acceptTOS.update.confirmationMessage',
          level: InformationLevel.Info,
        }),
        PresentationMode.Toast,
      );
      modalOpened.value = false;
      // Early return here. If the user didn't accept the TOS or the acceptance fails,
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

  await logout();
}

async function showUpdateModal(updateData: UpdateAvailabilityData): Promise<void> {
  if (!isUpdatePromptAllowed()) return;

  const existingModal = await modalController.getTop();
  if (existingModal) {
    window.electronAPI.log('debug', 'An existing modal is opened, skipping update prompt');
    return;
  }

  if (!updateData.version) {
    window.electronAPI.log('error', 'Version missing from update data');
    return;
  }

  const modal = await modalController.create({
    component: UpdateAppModal,
    canDismiss: true,
    cssClass: 'update-app-modal',
    backdropDismiss: false,
    componentProps: {
      currentVersion: APP_VERSION,
      targetVersion: updateData.version,
    },
  });
  await modal.present();
  const { role } = await modal.onWillDismiss();
  await modal.dismiss();

  if (role === MsModalResult.Confirm) {
    window.electronAPI.prepareUpdate();
  }
  suppressUpdatePrompt();
}
</script>
