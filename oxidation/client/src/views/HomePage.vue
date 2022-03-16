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
          <ion-title size="large">Parsec</ion-title>
        </ion-toolbar>
      </ion-header>

      <div id="container">
        <strong>Ready to create Parsec?</strong>
        <div>
          <ion-button @click="onSubmit">Let's go!</ion-button>
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

const status = ref('uninitialized');
const key = ref('1UT2bs6chdW4AnXbkSS18EuwOAgWIr7ROcHnicUhdAA=');
const message = ref('Ym9ueW91ciE=');

async function onSubmit(): Promise<any> {
  console.log('onSubmit !');
  // Avoid concurrency modification
  const keyValue = key.value;
  const messageValue = message.value;
  let encrypted = '';

  try {
    console.log('calling encrypt...');
    encrypted = await libparsec.encrypt(keyValue, messageValue);

    console.log('calling decrypt...');
    const decrypted = await libparsec.decrypt(keyValue, encrypted);

    if (decrypted !== messageValue) {
      throw `Decrypted data differs from original data !\nDecrypted: ${decrypted}\nEncrypted: ${encrypted}`;
    }
  } catch (error) {
    const errmsg = `Error: ${error}`;
    status.value = errmsg;
    console.log(errmsg);
    const errtoast = await toastController.create({
      message: 'Encryption/decryption error :\'-(',
      duration: 2000
    });
    errtoast.present();
    return;
  }

  const okmsg = `Encrypted message: ${encrypted}`;
  status.value = okmsg;
  const oktoast = await toastController.create({
    message: 'All good ;-)',
    duration: 2000
  });
  oktoast.present();
  toto();
}

function toto(): boolean {
  console.log('toto');
  const yellow = false;
  if (yellow) {
    return true;
  } else {
    return 'chiasse';
  }
}
</script>

<style scoped>
#container {
    text-align: center;
    position: absolute;
    left: 0;
    right: 0;
    top: 50%;
    transform: translateY(-50%);
}

#container strong {
    font-size: 20px;
    line-height: 26px;
}

#container a {
    text-decoration: none;
}
</style>
