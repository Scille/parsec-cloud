<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    button
    lines="full"
    :detail="false"
    :class="{ selected: isSelected }"
    @click="$emit('click', $event, file)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
  >
    <div class="file-list-item">
      <div class="file-selected">
        <ion-checkbox
          aria-label=""
          class="checkbox"
          v-model="isSelected"
          v-show="isSelected || isHovered || showCheckbox"
          @click.stop
          @ion-change="$emit('select', file, isSelected)"
        />
      </div>
      <!-- file name -->
      <div class="file-name">
        <div class="file-name__icons">
          <ion-icon
            class="main-icon"
            :icon="getFileIcon()"
            size="default"
          />
          <ion-icon
            class="cloud-overlay"
            :class="isFileSynced() ? 'cloud-overlay-ok' : 'cloud-overlay-ko'"
            :icon="isFileSynced() ? cloudDone : cloudOffline"
          />
        </div>
        <ion-label class="file-name__label cell">
          {{ file.name }}
        </ion-label>
      </div>

      <!-- updated by -->
      <!-- Can't get the information right now, maybe later -->
      <div
        class="file-updatedBy"
        v-if="false"
      >
        <user-avatar-name
          :user-avatar="file.id"
          :user-name="file.id"
        />
      </div>

      <!-- last update -->
      <div class="file-lastUpdate">
        <ion-label class="label-last-update cell">
          {{ formatTimeSince(file.updated, '--', 'short') }}
        </ion-label>
      </div>

      <!-- file size -->
      <div class="file-size">
        <ion-label
          v-show="file.isFile()"
          class="label-size cell"
        >
          {{ formatFileSize((file as EntryStatFile).size) }}
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
import { formatFileSize } from '@/common/file';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { EntryStat, EntryStatFile } from '@/parsec';
import { IonButton, IonCheckbox, IonIcon, IonItem, IonLabel } from '@ionic/vue';
import { cloudDone, cloudOffline, document, ellipsisHorizontal, folder } from 'ionicons/icons';
import { ref } from 'vue';

const isHovered = ref(false);
const menuOpened = ref(false);
const isSelected = ref(false);

const props = defineProps<{
  file: EntryStat;
  showCheckbox: boolean;
}>();

const emits = defineEmits<{
  (e: 'click', event: Event, file: EntryStat): void;
  (e: 'menuClick', event: Event, file: EntryStat, onFinished: () => void): void;
  (e: 'select', file: EntryStat, selected: boolean): void;
}>();

defineExpose({
  isHovered,
  isSelected,
  props,
});

function getFileIcon(): string {
  if (!props.file.isFile()) {
    return folder;
  }
  return document;
}

function isFileSynced(): boolean {
  return !props.file.needSync;
}

async function onOptionsClick(event: Event): Promise<void> {
  menuOpened.value = true;
  emits('menuClick', event, props.file, () => {
    menuOpened.value = false;
  });
}
</script>

<style lang="scss" scoped>
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
