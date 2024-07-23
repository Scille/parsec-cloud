<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="header-content">
    <ion-button @click="logout"> BYE BYE </ion-button>
  </div>
</template>

<script setup lang="ts">
import { IonButton } from '@ionic/vue';
import { BmsAccessInstance } from '@/services/bms';
import { navigateTo, Routes } from '@/router';

async function logout(): Promise<void> {
  await BmsAccessInstance.get().logout();
  await navigateTo(Routes.Home, { replace: true });
}
</script>

<style scoped lang="scss">
.header-content {
  background-color: red;
  height: 12em;
}
</style>
