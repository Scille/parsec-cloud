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
              :key="device.slug"
            >
              <device-card
                :label="device.label || ''"
                :is-current="true"
              />
            </ion-item>
          </ion-list>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { add } from 'ionicons/icons';
import {
  IonButton,
  IonList,
  IonItem,
  IonIcon,
  IonPage,
  IonContent,
  IonText,
} from '@ionic/vue';
import DeviceCard from '@/components/devices/DeviceCard.vue';

const devices = ref([{label: 'A Mock Device', slug: 1}]);

function onAddDeviceClick(): void {
  console.log('Add device clicked');
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
