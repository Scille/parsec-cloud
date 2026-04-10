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
import { OwnDeviceInfo, listOwnDevices } from '@/parsec';
import { Routes, getCurrentRouteName, watchRoute } from '@/router';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { openDeviceGreetModal } from '@/views/devices/utils';
import { IonButton, IonItem, IonList, IonText } from '@ionic/vue';
import { MsModalResult } from 'megashark-lib';
import { Ref, inject, onMounted, onUnmounted, ref } from 'vue';

const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
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
  const modalResult = await openDeviceGreetModal(informationManager.value);
  if (modalResult === MsModalResult.Confirm) {
    await refreshDevicesList();
  }
}
</script>

<style scoped lang="scss">
.devices-container {
  display: flex;
  flex-direction: column;
  justify-content: end;
  position: relative;
  border-radius: var(--parsec-radius-12);
  background: var(--parsec-color-light-secondary-white);
  padding: 1.5rem;
  box-shadow:
    0 1px 1px 0 rgba(0, 0, 0, 0.05),
    0 1px 4px 0 rgba(0, 0, 0, 0.03),
    0 0 1px 0 rgba(0, 0, 0, 0.2);
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
