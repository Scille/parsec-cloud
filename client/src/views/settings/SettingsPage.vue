<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content class="settings-content">
      <settings-view class="settings-view" />
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { IonContent, IonPage } from '@ionic/vue';

import SettingsView from '@/views/settings/SettingsView.vue';
</script>

<style scoped lang="scss">
.settings-view {
  margin: 3rem 2rem 2rem;
}
</style>
