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
          <ion-button @click="onSubmit">
            Let's go!
          </ion-button>
        </div>
        <div>
          status: {{ status }}
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
  toastController
} from '@ionic/vue';
import { ref } from 'vue';
import { libparsec } from '../plugins/libparsec';
import { Buffer } from 'buffer';

const status = ref('uninitialized');
const key = ref(Uint8Array.from([213, 68, 246, 110, 206, 156, 133, 213, 184, 2, 117, 219, 145, 36,
  181, 240, 75, 176, 56, 8, 22, 34, 190, 209, 57, 193, 231, 137, 197, 33, 116, 0]));
const message = ref('bonyour!');
const configDir = ref('/home/<user>/.config/parsec');

async function onSubmit(): Promise<any> {
  console.log('onSubmit !');
  // Avoid concurrency modification
  const keyValue = key.value;
  const messageValue = message.value;
  let encrypted = Uint8Array.from([]);
  const configDirValue = configDir.value;

  try {
    const version = await libparsec.version();
    console.log(version);

    console.log('calling encrypt...');
    encrypted = await libparsec.encrypt(keyValue, messageValue);

    console.log('calling decrypt...');
    const decrypted = await libparsec.decrypt(keyValue, encrypted);

    if (decrypted !== messageValue) {
      throw `Decrypted data differs from original data !\nDecrypted: ${decrypted}\nEncrypted: ${encrypted}`;
    }

    console.log('calling list_availables_devices...');
    const devices = await libparsec.listAvailableDevices(configDirValue);
    console.log(devices);
  } catch (error) {
    console.log(error);
    const errmsg = `Error: ${error.value}`;
    status.value = errmsg;
    console.log(errmsg);
    const errtoast = await toastController.create({
      message: 'Encryption/decryption error :\'-(',
      duration: 2000
    });
    errtoast.present();
    return;
  }

  const okmsg = `Encrypted message: ${Buffer.from(encrypted).toString('base64')}`;
  status.value = okmsg;
  const oktoast = await toastController.create({
    message: 'All good ;-)',
    duration: 2000
  });
  oktoast.present();
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
