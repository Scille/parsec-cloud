<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-app :class="titlebarOsClass">
    <div class="app-titlebar-drag-region" />
    <ion-router-outlet />
  </ion-app>
</template>

<script setup lang="ts">
import { isLinux, isMacOS, isWindows } from '@/parsec';
import { IonApp, IonRouterOutlet } from '@ionic/vue';

const titlebarOsClass = {
  'desktop-app-titlebar-macos': isMacOS(),
  'desktop-app-titlebar-windows': isWindows(),
  'desktop-app-titlebar-linux': isLinux(),
};
</script>

<style scoped>
.app-titlebar-drag-region {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  /* Keep in sync with the titleBarOverlay height in electron/src/setup.ts. */
  height: env(titlebar-area-height, 40px);
  -webkit-app-region: drag;
}
</style>
