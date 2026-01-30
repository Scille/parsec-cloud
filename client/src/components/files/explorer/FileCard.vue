<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    class="file-card-item ion-no-padding"
    :class="{
      selected: entry.isSelected,
      'file-hovered': !entry.isSelected && (menuOpened || isHovered),
    }"
    @dblclick="$emit('openItem', $event, entry)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
    @contextmenu="onOptionsClick"
    @click.stop="$emit('update:modelValue', !entry.isSelected)"
  >
    <file-drop-zone
      :disabled="entry.isFile()"
      @files-added="$emit('filesAdded', $event, !entry.isFile() ? entry.name : undefined)"
      :is-reader="isWorkspaceReader"
      @drop-as-reader="$emit('dropAsReader')"
    >
      <div class="card-checkbox">
        <ms-checkbox
          :checked="entry.isSelected"
          @change="$emit('update:modelValue', !entry.isSelected)"
          v-show="entry.isSelected || isHovered || showCheckbox"
          @click.stop
          @dblclick.stop
        />
      </div>
      <div
        class="card-option"
        v-show="isHovered || menuOpened"
        @click.stop="onOptionsClick($event)"
        @dblclick.stop
      >
        <ion-icon :icon="ellipsisHorizontal" />
      </div>
      <div class="file-card">
        <div class="file-card-icons">
          <ms-image
            :image="entry.isFile() ? getFileIcon(entry.name) : Folder"
            class="file-icon"
          />
          <ion-icon
            class="cloud-overlay"
            :class="syncStatus.class"
            :icon="syncStatus.icon"
          />
        </div>

        <ion-text
          class="file-card__title cell"
          :class="{ selection: showCheckbox }"
          @click="showCheckbox ? null : !($event.metaKey || $event.ctrlKey) && $emit('openItem', $event, entry)"
          @dblclick.stop
          :title="entry.name"
        >
          {{ entry.name }}
        </ion-text>

        <ion-text class="file-card-last-update body-sm">
          {{ $msTranslate(formatTimeSince(entry.updated, '--', 'short')) }}
        </ion-text>
      </div>
    </file-drop-zone>
  </ion-item>
</template>

<script setup lang="ts">
import { getFileIcon } from '@/common/file';
import FileDropZone from '@/components/files/explorer/FileDropZone.vue';
import { EntryModel, EntrySyncStatus } from '@/components/files/types';
import { EntryName } from '@/parsec';
import { IonIcon, IonItem, IonText } from '@ionic/vue';
import { cloudDone, cloudOffline, cloudUpload, ellipsisHorizontal } from 'ionicons/icons';
import { Folder, formatTimeSince, MsCheckbox, MsImage } from 'megashark-lib';
import { computed, ref } from 'vue';

const isHovered = ref(false);
const menuOpened = ref(false);

const props = defineProps<{
  entry: EntryModel;
  showCheckbox: boolean;
  isWorkspaceReader?: boolean;
  modelValue: boolean;
}>();

const emits = defineEmits<{
  (e: 'openItem', event: Event, entry: EntryModel): void;
  (e: 'menuClick', event: Event, entry: EntryModel, onFinished: () => void): void;
  (e: 'filesAdded', files: Array<File>, destinationFolder?: EntryName): void;
  (e: 'dropAsReader'): void;
  (e: 'update:modelValue', value: boolean): void;
}>();

defineExpose({
  props,
});

const syncStatus = computed(() => {
  switch (props.entry.syncStatus) {
    case EntrySyncStatus.Synced:
      return { class: 'cloud-overlay-ok', icon: cloudDone };
    case EntrySyncStatus.Uploading:
      return { class: 'cloud-overlay-ko', icon: cloudUpload };
    default:
      return { class: 'cloud-overlay-ko', icon: cloudOffline };
  }
});

async function onOptionsClick(event: Event): Promise<void> {
  event.preventDefault();
  event.stopPropagation();
  menuOpened.value = true;
  emits('menuClick', event, props.entry, () => {
    menuOpened.value = false;
  });
}
</script>

<style lang="scss" scoped>
.file-card-item {
  --background: var(--parsec-color-light-secondary-background);
  background: var(--parsec-color-light-secondary-background);
  cursor: default;
  text-align: center;
  user-select: none;
  width: 10.5rem;
  transition: width 0.2s ease-in-out;

  @include ms.responsive-breakpoint('xs') {
    width: 9rem;
  }

  @include ms.responsive-breakpoint('xs') {
    width: 8rem;
  }
}

.card-checkbox {
  left: 0.5rem;
  top: 0.25rem;
}

.card-option {
  padding: 0.5rem;
  top: 0;
  right: 0;
}

.file-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 1.5rem 0.5rem;
  width: 100%;
  margin: auto;

  @include ms.responsive-breakpoint('sm') {
    padding: 1rem 0.5rem;
  }

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
    max-height: 3rem;
    width: inherit;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
    text-overflow: ellipsis;

    &:not(.selection):hover {
      text-decoration: underline;
      cursor: pointer !important;
    }
  }

  &-last-update {
    padding-top: 0.25rem;
  }
}

.file-card-last-update {
  color: var(--parsec-color-light-secondary-grey);
  text-align: center;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
}

/* No idea how to change the color of the ion-item */
.file-card__title::part(native),
.file-card-last-update::part(native) {
  background-color: var(--parsec-color-light-secondary-background);
}
</style>
