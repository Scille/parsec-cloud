<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    button
    lines="full"
    :detail="false"
    :class="{ selected: entry.isSelected }"
    @click="$emit('click', $event, entry)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
  >
    <div class="file-list-item">
      <div class="file-selected">
        <!-- eslint-disable vue/no-mutating-props -->
        <ion-checkbox
          aria-label="''"
          class="checkbox"
          v-model="entry.isSelected"
          v-show="entry.isSelected || isHovered || showCheckbox"
          @ion-change="$emit('selectedChange', entry, $event.detail.checked)"
          @click.stop
        />
        <!-- eslint-enable vue/no-mutating-props -->
      </div>
      <!-- file name -->
      <div class="file-name">
        <ms-image
          :image="entry.isFile() ? getFileIcon(entry.name) : Folder"
          class="file-icon"
        />
        <ion-label class="file-name__label cell">
          {{ entry.name }}
        </ion-label>
        <ion-icon
          class="cloud-overlay"
          :class="isFileSynced() ? 'cloud-overlay-ok' : 'cloud-overlay-ko'"
          :icon="isFileSynced() ? cloudDone : cloudOffline"
        />
      </div>

      <!-- updated by -->
      <!-- Can't get the information right now, maybe later -->
      <div
        class="file-updatedBy"
        v-show="false"
      >
        <user-avatar-name
          :user-avatar="entry.id"
          :user-name="entry.id"
        />
      </div>

      <!-- last update -->
      <div class="file-lastUpdate">
        <ion-label class="label-last-update cell">
          {{ formatTimeSince(entry.updated, '--', 'short') }}
        </ion-label>
      </div>

      <!-- file size -->
      <div class="file-size">
        <ion-label
          v-show="entry.isFile()"
          class="label-size cell"
        >
          {{ formatFileSize((entry as FileModel).size) }}
        </ion-label>
      </div>

      <!-- options -->
      <div class="file-options ion-item-child-clickable">
        <ion-button
          fill="clear"
          v-show="isHovered || menuOpened"
          class="options-button"
          @click.stop="onOptionsClick($event)"
        >
          <ion-icon
            :icon="ellipsisHorizontal"
            slot="icon-only"
            class="options-button__icon"
          />
        </ion-button>
      </div>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { formatTimeSince } from '@/common/date';
import { formatFileSize, getFileIcon } from '@/common/file';
import { Folder, MsImage } from '@/components/core/ms-image';
import { EntryModel, FileModel } from '@/components/files/types';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { IonButton, IonCheckbox, IonIcon, IonItem, IonLabel } from '@ionic/vue';
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
  (e: 'selectedChange', entry: EntryModel, checked: boolean): void;
}>();

defineExpose({
  isHovered,
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
.file-name {
  .file-icon {
    width: 2rem;
    height: 2rem;
  }

  &__label {
    color: var(--parsec-color-light-secondary-text);
    margin-left: 1em;
    overflow: hidden;
    text-overflow: ellipsis;
    text-wrap: nowrap;
  }

  .cloud-overlay {
    font-size: 1rem;
    flex-shrink: 0;

    &-ok {
      color: var(--parsec-color-light-primary-500);
    }

    &-ko {
      color: var(--parsec-color-light-secondary-grey);
    }
  }
}

.file-options {
  ion-button::part(native) {
    padding: 0;
  }

  .options-button {
    --background-hover: none;

    &__icon {
      color: var(--parsec-color-light-secondary-grey);
    }

    &:hover {
      .options-button__icon {
        color: var(--parsec-color-light-primary-500);
      }
    }
  }
}
</style>
