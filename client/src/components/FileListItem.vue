<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-item
    button
    class="file-list-item"
    lines="full"
    :detail="false"
    :class="{ selected: isSelected, 'no-padding-end': !isSelected }"
    @click="$emit('click', $event, file)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
  >
    <div
      class="file-selected"
    >
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
    <div class="file-updatedBy">
      <user-tag
        :user="file.updater"
      />
    </div>

    <!-- last update -->
    <div class="file-lastUpdate">
      <ion-label class="label-last-update cell">
        {{ timeSince(file.lastUpdate, '--', 'short') }}
      </ion-label>
    </div>

    <!-- file size -->
    <div class="file-size">
      <ion-label
        v-show="file.type === 'file'"
        class="label-size cell"
      >
        {{ fileSize(file.size) }}
      </ion-label>
    </div>

    <!-- options -->
    <div class="file-options ion-item-child-clickable">
      <ion-button
        fill="clear"
        class="options-button"
        @click.stop="$emit('menuClick', $event, file)"
      >
        <ion-icon
          :icon="ellipsisHorizontal"
          slot="icon-only"
          class="options-button__icon"
        />
      </ion-button>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import {
  ellipsisHorizontal,
  cloudDone,
  cloudOffline,
  folder,
  document,
} from 'ionicons/icons';
import { ref, inject } from 'vue';
import { IonIcon, IonButton, IonItem, IonLabel, IonCheckbox } from '@ionic/vue';
import { FormattersKey, Formatters } from '@/common/injectionKeys';
import { MockFile } from '@/common/mocks';
import UserTag from '@/components/UserTag.vue';

const isHovered = ref(false);
const isSelected = ref(false);

const props = defineProps<{
  file: MockFile
  showCheckbox: boolean
}>();

defineEmits<{
  (e: 'click', event: Event, file: MockFile): void
  (e: 'menuClick', event: Event, file: MockFile): void
  (e: 'select', file: MockFile, selected: boolean): void
}>();

defineExpose({
  isHovered,
  isSelected,
  props,
});

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const { timeSince, fileSize } = inject(FormattersKey)! as Formatters;

function getFileIcon(): string {
  if (props.file.type === 'folder') {
    return folder;
  }
  return document;
}

function isFileSynced(): boolean {
  return true;
}
</script>

<style lang="scss" scoped>
.file-list-item {
  border-radius: var(--parsec-radius-4);
  --show-full-highlight: 0;

  &::part(native) {
    --padding-start: 0px;
  }

  &:hover:not(.selected) {
    --background-hover: var(--parsec-color-light-primary-30);
    --background-hover-opacity: 1;
  }

  &:hover, &.selected {
    .cell, .options-button__icon {
      color: var(--parsec-color-light-secondary-text);
      --background: red;
    }
  }

  &:focus, &:active, &.selected {
    --background-focused: var(--parsec-color-light-primary-100);
    --background: var(--parsec-color-light-primary-100);
    --background-focused-opacity: 1;
    --border-width: 0;
  }
}

.file-list-item>[class^="file-"] {
  padding: 0 1rem;
  display: flex;
  align-items: center;
  height: 4rem;
}

.file-selected {
  min-width: 4rem;
  justify-content: end;
}

.file-name {
  padding: .75rem 1rem;
  width: 100%;
  white-space: nowrap;
  overflow: hidden;

  &__icons {
  position: relative;
  padding: 5px;

    .main-icon {
      color: var(--parsec-color-light-primary-600);
      font-size: 1.5rem;
    }

    .cloud-overlay {
      height: 40%;
      width: 40%;
      position: absolute;
      font-size: 1.5rem;
      bottom: 1px;
      left: 53%;
      padding: 2px;
      background: var(--parsec-color-light-secondary-inversed-contrast);
      border-radius: 50%;
    }

    .cloud-overlay-ok {
      color: var(--parsec-color-light-primary-500);
    }

    .cloud-overlay-ko {
      color: var(--parsec-color-light-secondary-text);
    }
  }

  &__label {
    color: var(--parsec-color-light-secondary-text);
    margin-left: 1em;
  }
}

.file-updatedBy {
  min-width: 11.25rem;
  max-width: 10vw;
  flex-grow: 2;
}

.file-lastUpdate {
  min-width: 11.25rem;
  flex-grow: 0;
}

.file-size {
  min-width: 11.25rem;
}

.file-options {
  flex-grow: 0;
  margin-left: auto;

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

.label-size, .label-last-update {
  color: var(--parsec-color-light-secondary-grey);
}
</style>
