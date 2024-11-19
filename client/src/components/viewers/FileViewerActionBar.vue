<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-action-bar class="file-viewer-action-bar">
    <ms-action-bar-button
      v-for="(action, key) in actions"
      :key="key"
      :icon="action.icon"
      @click="action.handler"
      :button-label="action.text"
    />
  </ms-action-bar>
</template>

<script setup lang="ts">
import { MsActionBar, MsActionBarButton, Translatable } from 'megashark-lib';

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

<style scoped>
.file-viewer-action-bar {
  padding: 0.5rem 1rem;
  width: fit-content;
  background: var(--parsec-color-light-primary-100);
  border-radius: 5em;
  box-shadow: var(--parsec-shadow-light);
}
</style>
