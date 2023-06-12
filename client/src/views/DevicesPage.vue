<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <div class="devices-container">
        <h1>{{ $t("DevicesPage.title") }}</h1>
        <ion-button
          fill="outline"
          @click="onAddDeviceClick()"
        >
          <ion-icon
            :icon="add"
          />
          {{ $t("DevicesPage.addDevice") }}
        </ion-button>
        <ion-list>
          <ion-item
            v-for="device in devices"
            :key="device.slug"
          >
            <span>{{ device.deviceLabel }}</span>
          </ion-item>
        </ion-list>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { add } from 'ionicons/icons';
import { IonButton, IonList, IonItem, IonIcon, IonPage, IonContent } from '@ionic/vue';
import { getMockDevices } from '@/common/mocks';

const devices = ref(getMockDevices());

function onAddDeviceClick() {
  console.log('Add device clicked');
}
</script>

<style scoped lang="scss">
.devices-container {
  margin: 2em;
  background-color: white;
}
</style>
