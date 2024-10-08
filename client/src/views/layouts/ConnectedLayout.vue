<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <!-- Don't load children components before we inject everything -->
  <ion-page v-if="initialized">
    <ion-content>
      <ion-router-outlet />
    </ion-content>
  </ion-page>
</template>

<script lang="ts" setup>
import { getConnectionInfo, getTOS, needsMocks, logout as parsecLogout, acceptTOS } from '@/parsec';
import { getConnectionHandle, navigateTo, Routes } from '@/router';
import { EventData, EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { FileOperationManagerKey } from '@/services/fileOperationManager';
import { Information, InformationLevel, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { InjectionProvider, InjectionProviderKey, Injections } from '@/services/injectionProvider';
import { IonContent, IonPage, IonRouterOutlet, modalController } from '@ionic/vue';
import { inject, onMounted, onUnmounted, provide, Ref, ref } from 'vue';
import TOSModal from '@/views/organizations/TOSModal.vue';
import useUploadMenu from '@/services/fileUploadMenu';
import { MsModalResult, openSpinnerModal } from 'megashark-lib';
import { DateTime } from 'luxon';

const injectionProvider: InjectionProvider = inject(InjectionProviderKey)!;
const injections: Ref<Injections | null> = ref(null);
const initialized = ref(false);
const modalOpened = ref(false);
let intervalId: number | null = null;
let callbackId: string | null = null;
const lastAccepted: Ref<DateTime | null> = ref(null);

onMounted(async () => {
  const handle = getConnectionHandle();
  if (!handle) {
    console.error('Could not retrieve connection handle');
    return;
  }
  if (needsMocks() && !injectionProvider.hasInjections(handle)) {
    injectionProvider.createNewInjections(handle, new EventDistributor());
  }
  injections.value = injectionProvider.getInjections(handle);
  // Provide the injections to children
  provide(FileOperationManagerKey, injections.value.fileOperationManager);
  provide(InformationManagerKey, injections.value.informationManager);
  provide(EventDistributorKey, injections.value.eventDistributor);
  initialized.value = true;

  callbackId = await injections.value.eventDistributor.registerCallback(
    Events.TOSAcceptRequired | Events.LogoutRequested,
    async (event: Events, _data?: EventData) => {
      if (event === Events.LogoutRequested) {
        await logout();
      } else if (event === Events.TOSAcceptRequired) {
        await tryOpeningTOSModal();
      }
    },
  );

  const connInfo = getConnectionInfo();
  if (connInfo && connInfo.shouldAcceptTos) {
    await showTOSModal();
  }
});

onUnmounted(async () => {
  if (intervalId !== null) {
    window.clearInterval(intervalId);
    intervalId = null;
  }
  if (injections.value && callbackId !== null) {
    injections.value.eventDistributor.removeCallback(callbackId);
  }
});

async function tryOpeningTOSModal(): Promise<void> {
  if (modalOpened.value) {
    return;
  }
  if ((await modalController.getTop()) || injections.value?.fileOperationManager.hasOperations()) {
    if (intervalId === null) {
      // Try again in 1 minute
      intervalId = window.setInterval(async () => {
        await tryOpeningTOSModal();
      }, 60000);
    }
  } else {
    if (intervalId) {
      window.clearInterval(intervalId);
      intervalId = null;
    }
    await showTOSModal();
  }
}

async function logout(): Promise<void> {
  const handle = getConnectionHandle();
  if (!handle) {
    console.error('Already logged out');
    return;
  }
  const modal = await openSpinnerModal('HomePage.topbar.logoutWait');
  const menuCtrls = useUploadMenu();
  menuCtrls.hide();
  // Cleaning the injections will automatically cancel the imports
  await injectionProvider.clean(handle);
  const logoutResult = await parsecLogout();
  if (!logoutResult.ok) {
    window.electronAPI.log('error', `Error when logging out: ${logoutResult.error}`);
  }
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
  if (result.value.updatedOn === lastAccepted.value) {
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
  modalOpened.value = false;

  if (role === MsModalResult.Confirm) {
    const acceptResult = await acceptTOS(result.value.updatedOn);
    if (acceptResult.ok) {
      const connInfo = getConnectionInfo();
      if (connInfo) {
        connInfo.shouldAcceptTos = false;
      }
      lastAccepted.value = result.value.updatedOn;
      injections.value?.informationManager.present(
        new Information({
          message: 'CreateOrganization.acceptTOS.update.confirmationMessage',
          level: InformationLevel.Info,
        }),
        PresentationMode.Toast,
      );
      // Early return here. If the user didn't accept the TOS or the acception fails,
      // we will log them out.
      return;
    } else {
      window.electronAPI.log('error', `Error when accepting the TOS: ${acceptResult.error}`);
      injections.value?.informationManager.present(
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
</script>
