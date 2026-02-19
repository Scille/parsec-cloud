<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="devices-container">
    <ion-button
      id="add-device-button"
      class="button-normal button-default"
      fill="clear"
      @click="onAddDeviceClick()"
    >
      {{ $msTranslate('DevicesPage.addDevice') }}
    </ion-button>

    <!-- device list -->
    <div class="devices-content">
      <ion-text
        class="no-device"
        v-if="devices.length === 0"
      >
        {{ $msTranslate('DevicesPage.noDevices') }}
      </ion-text>
      <ion-list
        class="devices-list"
        id="devices-list"
        v-if="devices.length > 0"
      >
        <ion-item
          v-for="device in devices"
          :key="device.id"
          class="device-list-item ion-no-padding"
        >
          <device-card
            :device="device"
            :is-current="device.isCurrent"
            :show-id="true"
          />
        </ion-item>
      </ion-list>
    </div>
  </div>
</template>

<script setup lang="ts">
import DeviceCard from '@/components/devices/DeviceCard.vue';
import { OwnDeviceInfo, createDeviceInvitation, listOwnDevices } from '@/parsec';
import { Routes, getCurrentRouteName, watchRoute } from '@/router';
import { EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import GreetDeviceModal from '@/views/devices/GreetDeviceModal.vue';
import { IonButton, IonItem, IonList, IonText, modalController } from '@ionic/vue';
import { MsModalResult } from 'megashark-lib';
import { Ref, inject, onMounted, onUnmounted, ref } from 'vue';

const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
const eventDistributor: Ref<EventDistributor> = inject(EventDistributorKey)!;
const devices: Ref<OwnDeviceInfo[]> = ref([]);

const routeWatchCancel = watchRoute(async () => {
  if (getCurrentRouteName() !== Routes.MyProfile) {
    return;
  }
  await refreshDevicesList();
});

onMounted(async () => {
  await refreshDevicesList();
});

onUnmounted(async () => {
  routeWatchCancel();
});

async function refreshDevicesList(): Promise<void> {
  const result = await listOwnDevices();
  if (result.ok) {
    devices.value = result.value.filter((d) => !d.isRecovery && !d.isShamir && !d.isRegistration).sort((d) => (d.isCurrent ? -1 : 1));
  } else {
    informationManager.value.present(
      new Information({
        message: 'DevicesPage.greet.errors.retrieveDeviceInfoFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    window.electronAPI.log('error', `Failed to list devices ${JSON.stringify(result.error)}`);
  }
}

async function onAddDeviceClick(): Promise<void> {
  const result = await createDeviceInvitation(false);
  if (!result.ok) {
    informationManager.value.present(
      new Information({
        message: 'DevicesPage.greet.errors.startFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return;
  }

  const [invitationAddr, _] = result.value.addr;
  const modal = await modalController.create({
    component: GreetDeviceModal,
    canDismiss: true,
    backdropDismiss: false,
    cssClass: 'greet-organization-modal',
    componentProps: {
      informationManager: informationManager.value,
      invitationLink: invitationAddr,
      token: result.value.token,
    },
  });
  await modal.present();
  const modalResult = await modal.onWillDismiss();
  await modal.dismiss();
  if (modalResult.role === MsModalResult.Confirm) {
    await refreshDevicesList();
    const lastDevice = devices.value.toSorted((d1, d2) => (d1.createdOn > d2.createdOn ? -1 : 1))[0];
    if (lastDevice) {
      eventDistributor.value.dispatchEvent(Events.DeviceCreated, { info: lastDevice });
    }
  }
}
</script>

<style scoped lang="scss">
.devices-container {
  display: flex;
  flex-direction: column;
  justify-content: end;
  position: relative;
}

#add-device-button {
  margin-left: auto;
  --background: var(--parsec-color-light-secondary-text);
  --background-hover: var(--parsec-color-light-secondary-contrast);
  color: var(--parsec-color-light-secondary-white);

  @include ms.responsive-breakpoint('xs') {
    position: fixed;
    bottom: 2rem;
    left: 2rem;
    transform: translateX(50%, 50%);
    width: calc(100% - 4rem);
    margin: auto;
    z-index: 2;
    box-shadow: var(--parsec-shadow-strong);
  }
}

.devices-content {
  margin-top: 1rem;
  overflow-y: auto;

  .devices-list {
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
}
</style>
