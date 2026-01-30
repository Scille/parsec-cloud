<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-drop-zone
    :disabled="disableDrop || entry.isFile()"
    @files-added="$emit('filesAdded', $event, !entry.isFile() ? entry.name : undefined)"
    :is-reader="isWorkspaceReader"
    class="drop-zone-item"
    @drop-as-reader="$emit('dropAsReader')"
  >
    <ion-item
      button
      :lines="isLargeDisplay ? 'full' : 'none'"
      :detail="false"
      class="file-list-item"
      :class="{
        selected: entry.isSelected,
        'file-hovered': !entry.isSelected && (menuOpened || isHovered),
        'file-list-item-mobile': isSmallDisplay,
      }"
      @dblclick="$emit('openItem', $event, entry)"
      @click="$emit('update:modelValue', !entry.isSelected)"
      @mouseenter="isHovered = true"
      @mouseleave="isHovered = false"
      @contextmenu="onOptionsClick"
    >
      <div class="list-item-container file-list-item-container">
        <div
          class="file-selected"
          v-if="isLargeDisplay || entry.isSelected || showCheckbox"
        >
          <ms-checkbox
            @change="$emit('update:modelValue', !entry.isSelected)"
            :checked="entry.isSelected"
            v-show="entry.isSelected || isHovered || showCheckbox"
            @click.stop
            @dblclick.stop
          />
        </div>

        <!-- file name -->
        <div
          class="file-name"
          :class="{ 'file-mobile-content': isSmallDisplay }"
        >
          <ms-image
            :image="entry.isFile() ? getFileIcon(entry.name) : Folder"
            class="file-icon"
          />
          <div class="file-mobile-text">
            <ion-text
              class="label-name cell"
              :class="{ selection: showCheckbox }"
              :title="entry.name"
              @click="showCheckbox ? null : !($event.metaKey || $event.ctrlKey) && $emit('openItem', $event, entry)"
              @dblclick.stop
            >
              {{ entry.name }}
            </ion-text>
            <ion-text
              v-if="isSmallDisplay"
              class="file-mobile-text__data body-sm"
            >
              <span class="data-date">{{ $msTranslate(formatTimeSince(entry.updated, '--', 'short')) }}</span>
              <span v-if="entry.isFile()"> &bull; </span>
              <span
                class="data-size"
                v-if="entry.isFile()"
              >
                {{ $msTranslate(formatFileSize((entry as FileModel).size)) }}
              </span>
            </ion-text>
          </div>
          <ion-icon
            class="cloud-overlay"
            :class="syncStatus.class"
            :icon="syncStatus.icon"
          />
        </div>

        <!-- updated by -->
        <div
          class="file-updated-by"
          v-if="entry.lastUpdater && isLargeDisplay && ownProfile !== UserProfile.Outsider"
        >
          <user-avatar-name
            :user-avatar="entry.lastUpdater.humanHandle.label"
            :user-name="entry.lastUpdater.humanHandle.label"
          />
        </div>

        <!-- last update -->
        <div class="file-last-update">
          <ion-text class="label-last-update cell">
            {{ $msTranslate(formatTimeSince(entry.updated, '--', 'short')) }}
          </ion-text>
        </div>

        <!-- creation date -->
        <div class="file-creation-date">
          <ion-text class="label-creation-date cell">
            {{ $msTranslate(formatTimeSince(entry.created, '--', 'short')) }}
          </ion-text>
        </div>

        <!-- file size -->
        <div class="file-size">
          <ion-text
            v-if="entry.isFile()"
            class="label-size cell"
          >
            {{ $msTranslate(formatFileSize((entry as FileModel).size)) }}
          </ion-text>
        </div>

        <!-- options -->
        <div class="file-options ion-item-child-clickable">
          <ion-button
            fill="clear"
            v-show="isHovered || menuOpened || isSmallDisplay"
            class="options-button"
            @click.stop="onOptionsClick($event)"
            @dblclick.stop
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
  </file-drop-zone>
</template>

<script setup lang="ts">
import { formatFileSize, getFileIcon } from '@/common/file';
import FileDropZone from '@/components/files/explorer/FileDropZone.vue';
import { EntryModel, EntrySyncStatus, FileModel } from '@/components/files/types';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { EntryName, UserProfile } from '@/parsec';
import { IonButton, IonIcon, IonItem, IonText } from '@ionic/vue';
import { cloudDone, cloudOffline, cloudUpload, ellipsisHorizontal } from 'ionicons/icons';
import { Folder, formatTimeSince, MsCheckbox, MsImage, useWindowSize } from 'megashark-lib';
import { computed, ref } from 'vue';

const isHovered = ref(false);
const menuOpened = ref(false);
const { isSmallDisplay, isLargeDisplay } = useWindowSize();

const props = defineProps<{
  ownProfile: UserProfile;
  entry: EntryModel;
  showCheckbox: boolean;
  isWorkspaceReader?: boolean;
  disableDrop?: boolean;
  modelValue: boolean;
}>();

const emits = defineEmits<{
  (e: 'click', event: Event, entry: EntryModel): void;
  (e: 'openItem', event: Event, entry: EntryModel): void;
  (e: 'menuClick', event: Event, entry: EntryModel, onFinished: () => void): void;
  (e: 'filesAdded', files: Array<File>, destinationFolder?: EntryName): void;
  (e: 'dropAsReader'): void;
  (e: 'update:modelValue', value: boolean): void;
}>();

defineExpose({
  isHovered,
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

async function onOptionsClick(event: PointerEvent): Promise<void> {
  event.preventDefault();
  event.stopPropagation();

  menuOpened.value = true;
  emits('menuClick', event, props.entry, () => (menuOpened.value = false));
}
</script>

<style lang="scss" scoped>
.drop-zone-item {
  height: fit-content;
}

.file-name {
  position: relative;
  display: flex;
  gap: 1rem;

  .file-icon {
    width: 2rem;
    height: 2rem;
    flex-shrink: 0;
  }

  .label-name {
    color: var(--parsec-color-light-secondary-text);

    &:not(.selection):hover {
      text-decoration: underline;
      cursor: pointer !important;
    }
  }

  .cloud-overlay {
    font-size: 1rem;
    flex-shrink: 0;
    margin-left: auto;
    position: absolute;
    right: 0;

    @include ms.responsive-breakpoint('sm') {
      display: none;
    }

    &-ok {
      color: var(--parsec-color-light-primary-500);
    }

    &-ko {
      color: var(--parsec-color-light-secondary-grey);
    }
  }
}

.file-list-item-mobile {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  margin: 0 0.5rem;
  border-radius: var(--parsec-radius-8);

  .file-mobile-content {
    gap: 1rem;
    width: 100%;
  }

  .file-selected {
    max-width: 2.5rem;
    overflow: visible;
  }
}

.file-mobile-text {
  flex-grow: 1;
  overflow: hidden;
  color: var(--parsec-color-light-secondary-grey);
  display: flex;
  gap: 0.125rem;
  flex-direction: column;

  &__data {
    display: flex;
    gap: 0.5rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;

    span {
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
  }
}
</style>
