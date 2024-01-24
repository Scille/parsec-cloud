<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content>
      <ion-router-outlet />
      <file-upload-menu />
    </ion-content>
  </ion-page>
</template>

<script lang="ts" setup>
import FileUploadMenu from '@/views/files/FileUploadMenu.vue';
import { IonContent, IonPage, IonRouterOutlet } from '@ionic/vue';
</script>
