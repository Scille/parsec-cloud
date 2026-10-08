<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    class="file-card-item ion-no-padding"
    :class="{
      selected: entry.isSelected,
      'file-card-item--hovered': !entry.isSelected && (menuOpened || isHovered),
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
          <span
            v-if="entry.syncStatus === EntrySyncStatus.Uploading && entry.syncProgress"
            class="upload-progress"
          >
            {{ entry.syncProgress }}%
          </span>
        </div>

        <ion-text
          class="file-card__title"
          :class="{ selection: showCheckbox }"
          @click="showCheckbox ? null : !($event.metaKey || $event.ctrlKey) && $emit('openItem', $event, entry)"
          @dblclick.stop
          :title="entry.name"
        >
          {{ entry.name }}
        </ion-text>

        <ion-text class="file-card-last-update">
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
  (e: 'filesAdded', gen: AsyncGenerator<File[]>, destinationFolder?: EntryName): void;
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
  --background: #{ms.color('surface-base-default-secondary')};
  --background-hover: #{ms.color('surface-brand-default-subtle')};
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
  padding: ms.spacing('padding-lg');
  top: 0;
  right: 0;
}

.file-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: ms.spacing('padding-4xl') ms.spacing('padding-lg');
  width: 100%;
  margin: auto;

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-3xl') ms.spacing('padding-lg');
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
      bottom: -9px;
      padding: ms.spacing('padding-sm');
      background: ms.color('surface-base-default-secondary');
      border-radius: ms.radius('full');
      box-shadow: ms.shadow('light');

      &-ok {
        color: ms.color('icon-brand-default');
      }

      &-ko {
        color: ms.color('icon-neutral-default');
      }
    }
  }

  &__title {
    @include ms.font('label-md-medium');
    color: ms.color('text-base-body');
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
    padding-top: ms.spacing('padding-sm');
    @include ms.font('body-sm-regular');
    color: ms.color('text-base-description');
    text-align: center;
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
  }
}
</style>
