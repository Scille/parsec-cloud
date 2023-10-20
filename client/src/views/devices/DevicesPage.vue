<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <div class="devices-container">
        <div class="title">
          <h1>{{ $t('DevicesPage.title') }}</h1>
          <ion-button
            fill="clear"
            @click="onAddDeviceClick()"
          >
            <ion-icon
              :icon="add"
            />
            {{ $t('DevicesPage.addDevice') }}
          </ion-button>
        </div>
        <div class="list">
          <ion-text v-if="devices.length === 0">
            {{ $t('DevicesPage.noDevices') }}
          </ion-text>
          <ion-list
            v-if="devices.length > 0"
          >
            <ion-item
              v-for="device in devices"
              :key="device.id"
            >
              <device-card
                :label="device.deviceLabel"
                :is-current="device.isCurrent"
              />
            </ion-item>
          </ion-list>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { ref, Ref, onMounted, inject } from 'vue';
import { add } from 'ionicons/icons';
import {
  IonButton,
  IonList,
  IonItem,
  IonIcon,
  IonPage,
  IonContent,
  IonText,
  modalController,
} from '@ionic/vue';
import DeviceCard from '@/components/devices/DeviceCard.vue';
import { listOwnDevices, OwnDeviceInfo } from '@/parsec';
import { NotificationKey } from '@/common/injectionKeys';
import { NotificationCenter, NotificationLevel, Notification } from '@/services/notificationCenter';
import GreetDeviceModal from '@/views/devices/GreetDeviceModal.vue';
import { MsModalResult } from '@/components/core/ms-types';

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const notificationCenter: NotificationCenter = inject(NotificationKey)!;
const devices: Ref<OwnDeviceInfo[]> = ref([]);

onMounted(async () => {
  await refreshDevicesList();
});

async function refreshDevicesList(): Promise<void> {
  const result = await listOwnDevices();
  if (result.ok) {
    devices.value = result.value;
  } else {
    notificationCenter.showToast(new Notification({
      message: 'Failed to retrieve devices',
      level: NotificationLevel.Error,
    }));
    console.log('Could not list devices', result.error);
  }
}

async function onAddDeviceClick(): Promise<void> {
  const modal = await modalController.create({
    component: GreetDeviceModal,
    canDismiss: true,
    cssClass: 'greet-organization-modal',
  });
  await modal.present();
  const modalResult = await modal.onWillDismiss();
  await modal.dismiss();
  if (modalResult.role === MsModalResult.Confirm) {
    await refreshDevicesList();
  }
}
</script>

<style scoped lang="scss">
.devices-container {
  margin: 2em;
  background-color: white;
  width: 50%;
}

.title {
  display: flex;
  border-bottom: 1px solid var(--parsec-color-light-secondary-disabled);
}

.title h1 {
  color: var(--parsec-color-light-primary-700);
}

.title ion-button {
  margin-left: auto;
}

.list {
  padding-top: 2em;
}
</style>
