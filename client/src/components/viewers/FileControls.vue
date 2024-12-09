<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item-divider class="toolbar ion-margin-bottom secondary file-controls">
    <slot />
  </ion-item-divider>
</template>

<script setup lang="ts">
import { IonItemDivider } from '@ionic/vue';
</script>

<style scoped lang="scss">
.file-controls {
  padding: 0.5rem 1rem;
  width: fit-content;
  background: var(--parsec-color-light-primary-100);
  border-radius: 5em;
  box-shadow: var(--parsec-shadow-light);
  border-bottom: 1px solid var(--parsec-color-light-secondary-medium);
  --inner-padding-end: 0;
}
</style>
