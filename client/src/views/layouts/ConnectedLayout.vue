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
import { FileOperationManager, FileOperationManagerKey } from '@/services/fileOperation/manager';
import useUploadMenu from '@/services/fileUploadMenu';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
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

enum Modals {
  TOS = 'tos',
  Update = 'update',
  OrganizationNotFound = 'organization-not-found',
  IncompatibleServer = 'incompatible-server',
  ExpiredOrganization = 'expired-organization',
  Revoked = 'revoked',
  Frozen = 'frozen',
}

// Order matters, some modal should take priority
const modalsSequencer = new Map<Modals, boolean>([
  [Modals.OrganizationNotFound, false],
  [Modals.IncompatibleServer, false],
  [Modals.ExpiredOrganization, false],
  [Modals.Revoked, false],
  [Modals.Frozen, false],
  [Modals.TOS, false],
  [Modals.Update, false],
]);
const injectionProvider: InjectionProvider = inject(InjectionProviderKey)!;
const storageManager: StorageManager = inject(StorageManagerKey)!;
const { isUpdatePromptAllowed, suppressUpdatePrompt } = useUpdateManager();
let injections: Injections;
const initialized = ref(false);
let timeoutId: any = null;
let callbackId: string | null = null;
const lastAccepted: Ref<DateTime | null> = ref(null);
const updateAvailableData = ref<UpdateAvailabilityData | undefined>(undefined);

const refreshWarning = useRefreshWarning();

const fileOperationManager = ref<FileOperationManager | null>(null);
const eventDistributor = ref<EventDistributor | null>(null);
const informationManager = ref<InformationManager | null>(null);

provide(FileOperationManagerKey, fileOperationManager);
provide(InformationManagerKey, informationManager);
provide(EventDistributorKey, eventDistributor);

onMounted(async () => {
  const handle = getConnectionHandle();

  // No handle
  if (!handle) {
    window.electronAPI.log('error', 'Failed to retrieve connection handle while logged in');
    await navigateTo(Routes.Home, { replace: true, skipHandle: true });
    return;
  }

  if (!injectionProvider.hasInjections(handle)) {
    const eventDistributor = new EventDistributor();
    injectionProvider.createNewInjections(handle, eventDistributor);
  }
  const clientInfoResult = await getClientInfo(handle);
  // The handle is invalid
  if (!clientInfoResult.ok) {
    window.electronAPI.log('error', `Failed to retrieve client info: ${clientInfoResult.error.tag} (${clientInfoResult.error.error})`);
    await navigateTo(Routes.Home, { replace: true, skipHandle: true });
    return;
  }

  injections = injectionProvider.getInjections(handle);
  fileOperationManager.value = injections.fileOperationManager;
  informationManager.value = injections.informationManager;
  eventDistributor.value = injections.eventDistributor;

  callbackId = await injections.eventDistributor.registerCallback(
    [
      Events.TOSAcceptRequired,
      Events.LogoutRequested,
      Events.EntryDeleted,
      Events.EntryRenamed,
      Events.OrganizationNotFound,
      Events.IncompatibleServer,
      Events.ExpiredOrganization,
      Events.ClientRevoked,
      Events.ClientFrozen,
      Events.UpdateAvailability,
    ],
    eventCallback,
  );

  recentDocumentManager.resetHistory();
  await recentDocumentManager.loadFromStorage(storageManager, handle);

  initialized.value = true;

  // Making sure we get a notification as soon as possible
  window.electronAPI.getUpdateAvailability();

  if (clientInfoResult.value.mustAcceptTos) {
    modalsSequencer.set(Modals.TOS, true);
  }
  refreshWarning.warnOnRefresh();
  watchModals();
});

onUnmounted(async () => {
  window.clearTimeout(timeoutId);
  timeoutId = null;
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
      window.electronAPI.log('info', `Toggling modal ${Modals.TOS}`);
      modalsSequencer.set(Modals.TOS, true);
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
      window.electronAPI.log('info', `Toggling modal ${Modals.OrganizationNotFound}`);
      modalsSequencer.set(Modals.OrganizationNotFound, true);
      break;
    case Events.IncompatibleServer:
      window.electronAPI.log('info', `Toggling modal ${Modals.IncompatibleServer}`);
      modalsSequencer.set(Modals.IncompatibleServer, true);
      break;
    case Events.ExpiredOrganization:
      window.electronAPI.log('info', `Toggling modal ${Modals.ExpiredOrganization}`);
      modalsSequencer.set(Modals.ExpiredOrganization, true);
      break;
    case Events.ClientRevoked:
      window.electronAPI.log('info', `Toggling modal ${Modals.Revoked}`);
      modalsSequencer.set(Modals.Revoked, true);
      break;
    case Events.ClientFrozen:
      window.electronAPI.log('info', `Toggling modal ${Modals.Frozen}`);
      modalsSequencer.set(Modals.Frozen, true);
      break;
    case Events.UpdateAvailability: {
      window.electronAPI.log('info', `Toggling modal ${Modals.Update}`);
      modalsSequencer.set(Modals.Update, true);
      updateAvailableData.value = data as UpdateAvailabilityData;
      break;
    }
  }
}

async function watchModals(): Promise<void> {
  const CALLBACKS = new Map([
    [Modals.ExpiredOrganization, onExpiredOrganization],
    [Modals.Frozen, onFrozen],
    [Modals.IncompatibleServer, onIncompatibleServer],
    [Modals.OrganizationNotFound, onOrganizationNotFound],
    [Modals.Revoked, onClientRevoked],
    [Modals.TOS, onTermsOfService],
    [Modals.Update, onAppUpdate],
  ]);
  for (const [modal, toggled] of modalsSequencer.entries()) {
    if (toggled && !(await modalController.getTop())) {
      const cb = CALLBACKS.get(modal);
      if (cb) {
        modalsSequencer.set(modal, false);
        window.electronAPI.log('info', `Showing ${modal} modal`);
        await cb();
      }
    }
  }
  timeoutId = window.setTimeout(watchModals, (window as any).TESTING ? 500 : 5000);
}

async function onFrozen(): Promise<void> {
  await injections.informationManager.present(
    new Information({
      message: 'globalErrors.clientFrozen',
      level: InformationLevel.Error,
    }),
    PresentationMode.Modal,
  );
}

async function onIncompatibleServer(): Promise<void> {
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
}

async function onOrganizationNotFound(): Promise<void> {
  await injections.informationManager.present(
    new Information({
      message: 'globalErrors.organizationNotFound',
      level: InformationLevel.Error,
    }),
    PresentationMode.Modal,
  );
}

async function onExpiredOrganization(): Promise<void> {
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
}

async function onClientRevoked(): Promise<void> {
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
}

async function onTermsOfService(): Promise<void> {
  const result = await getTOS();

  if (!result.ok) {
    return;
  }
  if (!result.value.perLocaleUrls.size) {
    window.electronAPI.log('warn', 'Received empty Terms of Service dictionary');
    return;
  }
  if (result.value.updatedOn.toMillis() === lastAccepted.value?.toMillis()) {
    window.electronAPI.log('warn', 'Already accepted those TOS');
    return;
  }
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

async function onAppUpdate(): Promise<void> {
  if (!isUpdatePromptAllowed() || !updateAvailableData.value) {
    return;
  }

  const existingModal = await modalController.getTop();
  if (existingModal) {
    window.electronAPI.log('debug', 'An existing modal is opened, skipping update prompt');
    return;
  }

  if (!updateAvailableData.value.version) {
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
      targetVersion: updateAvailableData.value.version,
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

async function logout(): Promise<void> {
  window.clearTimeout(timeoutId);
  timeoutId = null;
  const top = await modalController.getTop();
  if (top) {
    await top.dismiss();
  }
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
</script>
