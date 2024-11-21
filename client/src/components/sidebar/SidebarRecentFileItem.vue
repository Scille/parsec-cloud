<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    lines="none"
    button
    @click="$emit('fileClicked', file)"
    class="sidebar-item menu-default"
    ref="itemRef"
  >
    <ion-text class="sidebar-item-file-label">{{ file.name }}</ion-text>
    <div
      class="file-option"
      @click.stop="$emit('removeClicked', file)"
    >
      <ion-icon :icon="close" />
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { IonItem, IonText, IonIcon } from '@ionic/vue';
import { close } from 'ionicons/icons';
import { RecentFile } from '@/services/recentFiles';

defineProps<{
  file: RecentFile;
}>();

defineEmits<{
  (e: 'fileClicked', file: RecentFile): void;
  (e: 'removeClicked', file: RecentFile): void;
}>();
</script>

<style scoped lang="scss">
.sidebar-item {
  --background: none;
  border-radius: var(--parsec-radius-8);
  border: solid 1px var(--parsec-color-light-primary-800);
  --padding-start: 1rem;
  --padding-end: 1rem;
  --padding-top: 0.5rem;
  --padding-bottom: 0.5rem;

  .file-option {
    color: var(--parsec-color-light-secondary-grey);
    font-size: 1.2rem;
    opacity: 1;
    display: flex;
    align-items: center;

    &:hover {
      color: var(--parsec-color-light-primary-30);
    }
  }

  &:hover {
    border: solid 1px var(--parsec-color-light-primary-30-opacity15);
    cursor: pointer;

    .file-option {
      opacity: 1;
    }
  }

  &:active,
  &.item-selected {
    --background: var(--parsec-color-light-primary-30-opacity15);
  }

  &-file-label {
    position: relative;
    text-overflow: ellipsis;
    white-space: nowrap;
    overflow: hidden;
    color: var(--parsec-color-light-primary-100);
    width: 100%;
  }
}
</style>
