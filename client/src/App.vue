<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <!--
    The entire app is remounted when the key changes.
    `multiOrganizationUpdateKey` only changes when we switch organization manually
  -->
  <ion-app :key="multiOrganizationUpdateKey">
    <ion-router-outlet />
  </ion-app>
</template>

<script setup lang="ts">
import { multiOrganizationUpdateKey } from '@/router/watchers';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { toggleDarkMode } from '@/states/darkMode';
import { SplashScreen } from '@capacitor/splash-screen';
import { IonApp, IonRouterOutlet } from '@ionic/vue';
import { inject, onMounted } from 'vue';

const storageManager: StorageManager = inject(StorageManagerKey)!;

onMounted(async (): Promise<void> => {
  SplashScreen.hide();

  const config = await storageManager.retrieveConfig();
  toggleDarkMode(config.theme);
});
</script>
