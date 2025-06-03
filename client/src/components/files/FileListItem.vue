<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-drop-zone
    :disabled="disableDrop || entry.isFile()"
    :current-path="currentPath"
    @files-added="$emit('filesAdded', $event)"
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
      @dblclick="$emit('click', $event, entry)"
      @mouseenter="isHovered = true"
      @mouseleave="isHovered = false"
      @contextmenu="onOptionsClick"
    >
      <div
        class="file-selected"
        v-if="isLargeDisplay || entry.isSelected || showCheckbox"
      >
        <!-- eslint-disable vue/no-mutating-props -->
        <ms-checkbox
          v-model="entry.isSelected"
          v-show="entry.isSelected || isHovered || showCheckbox"
          @change="$emit('selectedChange', entry, $event)"
          @click.stop
          @dblclick.stop
        />
        <!-- eslint-enable vue/no-mutating-props -->
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
          <ion-label class="file-name__label cell">
            {{ entry.name }}
          </ion-label>
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
          {{ $msTranslate(formatTimeSince(entry.updated, '--', 'short')) }}
        </ion-label>
      </div>

      <!-- last update -->
      <div class="file-creationDate">
        <ion-label class="label-last-update cell">
          {{ $msTranslate(formatTimeSince(entry.created, '--', 'short')) }}
        </ion-label>
      </div>

      <!-- file size -->
      <div class="file-size">
        <ion-label
          v-if="entry.isFile()"
          class="label-size cell"
        >
          {{ $msTranslate(formatFileSize((entry as FileModel).size)) }}
        </ion-label>
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
    </ion-item>
  </file-drop-zone>
</template>

<script setup lang="ts">
import { formatFileSize, getFileIcon } from '@/common/file';
import { Folder, formatTimeSince, MsImage, MsCheckbox, useWindowSize } from 'megashark-lib';
import FileDropZone from '@/components/files/FileDropZone.vue';
import { EntryModel, FileModel } from '@/components/files/types';
import { FileImportTuple } from '@/components/files/utils';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { FsPath, Path } from '@/parsec';
import { IonButton, IonIcon, IonItem, IonLabel, IonText } from '@ionic/vue';
import { cloudDone, cloudOffline, ellipsisHorizontal } from 'ionicons/icons';
import { Ref, onMounted, ref } from 'vue';

const isHovered = ref(false);
const menuOpened = ref(false);
const { isSmallDisplay, isLargeDisplay } = useWindowSize();

const props = defineProps<{
  entry: EntryModel;
  showCheckbox: boolean;
  isWorkspaceReader?: boolean;
  disableDrop?: boolean;
}>();

const emits = defineEmits<{
  (e: 'click', event: Event, entry: EntryModel): void;
  (e: 'menuClick', event: Event, entry: EntryModel, onFinished: () => void): void;
  (e: 'selectedChange', entry: EntryModel, checked: boolean): void;
  (e: 'filesAdded', imports: FileImportTuple[]): void;
  (e: 'dropAsReader'): void;
}>();

defineExpose({
  isHovered,
  props,
});

const currentPath: Ref<FsPath> = ref('/');

onMounted(async () => {
  if (props.entry.isFile()) {
    currentPath.value = await Path.parent(props.entry.path);
  } else {
    currentPath.value = props.entry.path;
  }
});

function isFileSynced(): boolean {
  return !props.entry.needSync;
}

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

  &__label {
    color: var(--parsec-color-light-secondary-text);
    overflow: hidden;
    text-overflow: ellipsis;
    text-wrap: nowrap;
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

  .file-mobile-content {
    gap: 1rem;
  }
}

.file-mobile-text {
  flex-grow: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: var(--parsec-color-light-secondary-grey);
  display: flex;
  gap: 0.125rem;
  flex-direction: column;

  &__data {
    display: flex;
    gap: 0.5rem;
  }
}
</style>
