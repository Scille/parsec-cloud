<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    lines="none"
    :disabled="disabled"
    button
    @click="$emit('fileClicked', file)"
    class="sidebar-item button-medium ion-no-padding"
  >
    <div class="sidebar-item-file">
      <ion-text class="sidebar-item-file__label">{{ file.name }}</ion-text>
      <div
        class="sidebar-item-file__option"
        @click.stop="$emit('removeClicked', file)"
      >
        <ion-icon :icon="close" />
      </div>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { RecentFile } from '@/services/recentDocuments';
import { IonIcon, IonItem, IonText } from '@ionic/vue';
import { close } from 'ionicons/icons';

defineProps<{
  file: RecentFile;
  disabled?: boolean;
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
  --min-height: 0;
  --padding-start: 0.75rem;
  --padding-end: 0.75rem;
  --padding-bottom: 0.5rem;
  --padding-top: 0.5rem;

  &:active,
  &.item-selected {
    --background: var(--parsec-color-light-primary-30-opacity15);
  }

  .sidebar-item-file {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;

    &__label {
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
      color: var(--parsec-color-light-secondary-premiere);
      width: 100%;
    }

    &__option {
      color: var(--parsec-color-light-secondary-light);
      display: none;
      margin-left: auto;
      font-size: 1rem;

      &:hover {
        color: var(--parsec-color-light-primary-30);
      }
    }
  }

  &:hover {
    outline: solid 1px var(--parsec-color-light-primary-30-opacity15);
    cursor: pointer;

    .sidebar-item-file__option {
      display: flex;
    }
  }
}
</style>
