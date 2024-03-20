<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    class="file-card-item ion-no-padding"
    :class="{ selected: entry.isSelected }"
    @dblclick="$emit('click', $event, entry)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
  >
    <div class="card-checkbox">
      <!-- eslint-disable vue/no-mutating-props -->
      <ion-checkbox
        aria-label="''"
        class="checkbox"
        v-model="entry.isSelected"
        v-show="entry.isSelected || isHovered || showCheckbox"
        @click.stop
      />
      <!-- eslint-enable vue/no-mutating-props -->
    </div>
    <div
      class="card-option"
      v-show="isHovered || menuOpened"
      @click.stop="onOptionsClick($event)"
    >
      <ion-icon :icon="ellipsisHorizontal" />
    </div>
    <div class="card-content">
      <div class="card-content-icons">
        <ms-image
          :image="entry.isFile() ? getFileIcon(entry.name) : Folder"
          class="file-icon"
        />
        <ion-icon
          class="cloud-overlay"
          :class="isFileSynced() ? 'cloud-overlay-ok' : 'cloud-overlay-ko'"
          :icon="isFileSynced() ? cloudDone : cloudOffline"
        />
      </div>

      <ion-title class="card-content__title body">
        {{ entry.name }}
      </ion-title>

      <ion-text class="card-content-last-update body-sm">
        <span>{{ $t('FoldersPage.File.lastUpdate') }}</span>
        <span>{{ formatTimeSince(entry.updated, '--', 'short') }}</span>
      </ion-text>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { formatTimeSince } from '@/common/date';
import { getFileIcon } from '@/common/file';
import { Folder, MsImage } from '@/components/core/ms-image';
import { EntryModel } from '@/components/files/types';
import { IonCheckbox, IonIcon, IonItem, IonText, IonTitle } from '@ionic/vue';
import { cloudDone, cloudOffline, ellipsisHorizontal } from 'ionicons/icons';
import { ref } from 'vue';

const isHovered = ref(false);
const menuOpened = ref(false);

const props = defineProps<{
  entry: EntryModel;
  showCheckbox: boolean;
}>();

const emits = defineEmits<{
  (e: 'click', event: Event, entry: EntryModel): void;
  (e: 'menuClick', event: Event, entry: EntryModel, onFinished: () => void): void;
}>();

defineExpose({
  props,
});

function isFileSynced(): boolean {
  return !props.entry.needSync;
}

async function onOptionsClick(event: Event): Promise<void> {
  menuOpened.value = true;
  emits('menuClick', event, props.entry, () => {
    menuOpened.value = false;
  });
}
</script>

<style lang="scss" scoped>
.file-card-item {
  position: relative;
  cursor: default;
  text-align: center;
  --background: var(--parsec-color-light-secondary-background);
  background: var(--parsec-color-light-secondary-background);
  border: 1px solid var(--parsec-color-light-secondary-medium);
  user-select: none;
  border-radius: var(--parsec-radius-12);
  width: 10.5rem;

  &::part(native) {
    --inner-padding-end: 0px;
  }

  &:hover {
    --background: var(--parsec-color-light-primary-30);
    --background-hover: var(--parsec-color-light-primary-30);
    --background-hover-opacity: 1;
  }

  &.selected {
    --background: var(--parsec-color-light-primary-100);
    --background-hover: var(--parsec-color-light-primary-100);
    border: 1px solid var(--parsec-color-light-primary-100);
  }
}

.card-option,
.card-checkbox {
  position: absolute;
}

.card-checkbox {
  left: 0.5rem;
  top: 0.5rem;
}

.card-option {
  color: var(--parsec-color-light-secondary-grey);
  text-align: right;
  display: flex;
  align-items: center;
  top: 0;
  right: 0;
  font-size: 1.5rem;
  padding: 0.5rem;
  cursor: pointer;

  &:hover {
    color: var(--parsec-color-light-primary-500);
  }
}

.card-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 1.5rem 0.5rem;
  width: 100%;
  margin: auto;

  &-icons {
    position: relative;
    height: fit-content;
    width: fit-content;
    margin: 0 auto 0.875rem;

    .file-icon {
      width: 3rem;
      height: 3rem;
    }

    .cloud-overlay {
      position: absolute;
      font-size: 1.25rem;
      left: 58%;
      bottom: -6px;
      padding: 2px;
      background: var(--parsec-color-light-secondary-background);
      border-radius: 50%;

      &-ok {
        color: var(--parsec-color-light-primary-500);
      }

      &-ko {
        color: var(--parsec-color-light-secondary-text);
      }
    }
  }

  &__title {
    color: var(--parsec-color-light-primary-900);
    text-align: center;
    padding: 0 0 0.25rem;
    text-overflow: ellipsis;
    white-space: nowrap;
    width: inherit;

    ion-text {
      width: 100%;
      overflow: hidden;
    }
  }
}

.card-content-last-update {
  color: var(--parsec-color-light-secondary-grey);
  text-align: center;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
}

/* No idea how to change the color of the ion-item */
.card-content__title::part(native),
.card-content-last-update::part(native) {
  background-color: var(--parsec-color-light-secondary-background);
}
</style>
