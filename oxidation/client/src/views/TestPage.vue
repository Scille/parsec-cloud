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
          <ion-input v-model="name" placeholder="Your name"></ion-input>,
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
  console.log(`calling helloWorld(${name.value})`);
  const rep = await libparsec.helloWorld(name.value);
  console.log('helloWorld returned', rep);
  switch (rep.ok) {
  case true:
    message.value = rep.value;
    break;
  case false:
    switch (rep.error.tag) {
    case 'EmptySubject':
      message.value = 'Where is your name ?';
      break;
    case 'YouAreADog':
      message.value = `Who's a good boy ? ${rep.error.hello} ! You are !`;
      break;
    }
    break;
  }
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
