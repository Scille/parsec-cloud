<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <h1>DUMMY NOT IMPLEMENTED</h1>
      <h2>WorkspaceId is {{ workspaceId }}</h2>
      <h2>Path is {{ path }}</h2>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonPage,
  IonContent
} from '@ionic/vue';
import { useRoute } from 'vue-router';
import { computed } from 'vue';
const currentRoute = useRoute();

// Use computed so the variables will automatically update when the route changes
const path = computed(() => {
  return currentRoute.query.path ?? '/';
});
const workspaceId = computed(() => {
  return currentRoute.params.workspaceId;
});
</script>

<style scoped></style>
