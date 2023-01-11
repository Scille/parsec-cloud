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
const message = ref('bonyour!');

async function onSubmit(): Promise<void> {
  console.log('Submitting');
  const key = await libparsec.login('a7d1edf7a8014e76b2d89e63f430d9fc@010cfc8b95ea4436bd594d919ec966fc');
  console.log(key);
  const rep = await libparsec.loggedCoreGetTestDeviceId(key);
  console.log(rep);
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
