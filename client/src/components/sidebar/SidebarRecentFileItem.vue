<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-item
    lines="none"
    :disabled="disabled"
    button
    @click="$emit('fileClicked', file)"
    class="sidebar-item ion-no-padding"
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
  @include ms.font('label-md-medium');
  --background: none;
  border-radius: ms.radius('lg');
  border: ms.border('thin') solid transparent;
  --min-height: 0;
  --padding-start: #{ms.spacing('padding-2xl')};
  --padding-end: #{ms.spacing('padding-2xl')};
  --padding-bottom: #{ms.spacing('padding-lg')};
  --padding-top: #{ms.spacing('padding-lg')};

  &:active,
  &.item-selected {
    --background: #{ms.color('surface-neutral-on-color')};
  }

  .sidebar-item-file {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: ms.spacing('gap-lg');
    width: 100%;

    &__label {
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
      color: ms.color('text-on-color-label');
      width: 100%;
    }

    &__option {
      color: ms.color('icon-on-color-label');
      display: none;
      margin-left: auto;
      font-size: 1rem;

      &:hover {
        color: ms.color('icon-on-color-heading');
      }
    }
  }

  &:hover {
    border-color: ms.color('border-neutral-on-color');
    cursor: pointer;

    .sidebar-item-file__label {
      color: ms.color('text-on-color-heading');
    }

    .sidebar-item-file__option {
      display: flex;
    }
  }
}
</style>
