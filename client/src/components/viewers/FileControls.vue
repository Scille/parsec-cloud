<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item-divider class="toolbar ion-margin-bottom secondary file-controls">
    <file-controls-button
      v-for="(action, key) in actions"
      :key="key"
      :icon="action.icon"
      @click="action.handler"
      :button-label="action.text"
    />
  </ion-item-divider>
</template>

<script setup lang="ts">
import { Translatable } from 'megashark-lib';
import { FileControlsButton } from '@/components/viewers';
import { IonItemDivider } from '@ionic/vue';

// TODO: add a disabled state to the button (ex: max zoom level).
// TODO: extract this interface to a separate file.
interface FileViewerAction {
  icon?: string;
  text?: Translatable;
  handler: () => void;
}

defineProps<{
  actions: Array<FileViewerAction>;
}>();
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
