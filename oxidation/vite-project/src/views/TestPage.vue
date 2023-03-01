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
import { ClientEvent, ClientConfig, DeviceAccessParamsPassword } from '../plugins/libparsec/definitions';

const name = ref('Scruffy');

const path = 'PATH/TO/.config/parsec/';
const password = 'PASSWORD';
// eslint-disable-next-line @typescript-eslint/explicit-function-return-type, @typescript-eslint/no-unused-vars
const onEventCallback = (event: ClientEvent) =>  {
  console.log('callback');
};
const config: ClientConfig = {
  configDir: '',
  dataBaseDir: '',
  mountpointBaseDir: '',
  preferredOrgCreationBackendAddr: 'parsec://alice_dev1.example.com:9999',
  workspaceStorageCacheSize: { tag: 'Default' }
};

async function onSubmit(): Promise<void> {
  console.log('Submitting');
  const devices = await libparsec.clientListAvailableDevices(path);
  console.log(devices);
  const param: DeviceAccessParamsPassword = {
    tag: 'Password',
    path: devices[0].keyFilePath,
    // path: devices[0].slug,
    password
  };
  const handle = await libparsec.clientLogin(param, config, onEventCallback).then((x) => x.ok ? x.value : -1);
  console.log(handle);
  const deviceID = await libparsec.clientGetDeviceId(handle).then((x) => x.ok ? x.value : -1);
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
