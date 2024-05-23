<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content>
      <ion-router-outlet />
      <file-operation-menu />
    </ion-content>
  </ion-page>
</template>

<script lang="ts" setup>
import FileOperationMenu from '@/views/files/FileOperationMenu.vue';
import { IonContent, IonPage, IonRouterOutlet } from '@ionic/vue';
</script>
