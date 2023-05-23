<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content class="settings-content">
      <settings-view />
    </ion-content>
  </ion-page>
</template>

<script setup lang = "ts" >
import {
  IonContent,
  IonPage
} from '@ionic/vue';

import SettingsView from '@/components/SettingsView.vue';
</script>

<style scoped>

</style>
