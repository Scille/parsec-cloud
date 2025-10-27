<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <client-area-page
      id="page"
      :key="BmsAccessInstance.get().reloadKey"
    />
  </ion-page>
</template>

<script setup lang="ts">
import { BmsAccessInstance } from '@/services/bms';
import ClientAreaPage from '@/views/client-area/ClientAreaPage.vue';
import { IonPage } from '@ionic/vue';
</script>
