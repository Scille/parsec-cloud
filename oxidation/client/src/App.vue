<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-app>
    <ion-router-outlet />
  </ion-app>
</template>

<script setup lang="ts">
import { IonApp, IonRouterOutlet } from '@ionic/vue';
import { SplashScreen } from '@capacitor/splash-screen';
import { onMounted } from '@vue/runtime-core';
import { Storage } from '@ionic/storage';
import { toggleDarkMode } from './states/darkMode';

onMounted(async (): Promise<void> => {
  SplashScreen.hide();

  // Set dark mode
  const store = new Storage();
  await store.create();
  const userTheme = await store.get('userTheme');
  toggleDarkMode(userTheme);
});

</script>
