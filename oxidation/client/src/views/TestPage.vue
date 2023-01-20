<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<!-- This page serve only for test purposes -->

<template>
  <ion-page>
    <ion-header :translucent="true">
      <ion-toolbar>
        <ion-title>Parsec</ion-title>
      </ion-toolbar>
    </ion-header>

    <ion-content :fullscreen="true">
      <ion-header collapse="condense">
        <ion-toolbar>
          <ion-title size="large">
            Parsec
          </ion-title>
        </ion-toolbar>
      </ion-header>

      <div id="container">
        <strong>Ready to create Parsec?</strong>
        <div>
          <ion-input
            v-model="name"
            placeholder="Your name"
          />,
          <ion-button @click="onSubmit">
            Let's go!
          </ion-button>
        </div>
        <div>
          {{ message }}
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonContent,
  IonHeader,
  IonPage,
  IonTitle,
  IonToolbar,
  IonButton,
  IonInput,
  toastController
} from '@ionic/vue';
import { ref } from 'vue';
import { libparsec } from '../plugins/libparsec';

const name = ref('Scruffy');

const path = 'PATH/TO/.config/parsec/';
const password = 'PASSWORD';

async function onSubmit(): Promise<void> {
  console.log('Submitting');
  const devices = await libparsec.listAvailableDevices(path);
  console.log(devices);
  const handle = await libparsec.login(devices[0].keyFilePath, password);
  // const handle = await libparsec.login(devices[0].slug, password);
  console.log(handle);
  const deviceID = await libparsec.loggedCoreGetDeviceId(0);
  console.log(deviceID);
}
</script>

<style lang="scss" scoped>
#container {
    text-align: center;
    position: absolute;
    left: 0;
    right: 0;
    top: 50%;
    transform: translateY(-50%);

    strong {
        font-size: 20px;
        line-height: 26px;
    }

    a {
        text-decoration: none;
    }
}
</style>
