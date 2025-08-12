<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    class="file-card-item ion-no-padding"
    :class="{
      selected: entry.isSelected,
      'file-hovered': !entry.isSelected && (menuOpened || isHovered),
    }"
    @dblclick="$emit('click', $event, entry)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
    @contextmenu="onOptionsClick"
    @click="onCardClick"
  >
    <file-drop-zone
      :disabled="entry.isFile()"
      :current-path="currentPath"
      @files-added="$emit('filesAdded', $event)"
      :is-reader="isWorkspaceReader"
      @drop-as-reader="$emit('dropAsReader')"
    >
      <div class="card-checkbox">
        <!-- eslint-disable vue/no-mutating-props -->
        <ms-checkbox
          v-model="entry.isSelected"
          v-show="entry.isSelected || isHovered || showCheckbox"
          @ion-change="$emit('select')"
          @click.stop
          @dblclick.stop
        />
        <!-- eslint-enable vue/no-mutating-props -->
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
            :class="isFileSynced() ? 'cloud-overlay-ok' : 'cloud-overlay-ko'"
            :icon="isFileSynced() ? cloudDone : cloudOffline"
          />
        </div>

        <ion-text class="file-card__title body">
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
import { Folder, formatTimeSince, MsImage, MsCheckbox } from 'megashark-lib';
import FileDropZone from '@/components/files/explorer/FileDropZone.vue';
import { EntryModel } from '@/components/files/types';
import { FileImportTuple } from '@/components/files/utils';
import { FsPath, Path } from '@/parsec';
import { IonIcon, IonItem, IonText } from '@ionic/vue';
import { cloudDone, cloudOffline, ellipsisHorizontal } from 'ionicons/icons';
import { Ref, onMounted, ref } from 'vue';

const isHovered = ref(false);
const menuOpened = ref(false);

const currentPath: Ref<FsPath> = ref('/');

onMounted(async () => {
  if (props.entry.isFile()) {
    currentPath.value = await Path.parent(props.entry.path);
  } else {
    currentPath.value = props.entry.path;
  }
});

const props = defineProps<{
  entry: EntryModel;
  showCheckbox: boolean;
  isWorkspaceReader?: boolean;
  modelValue?: boolean;
}>();

const emits = defineEmits<{
  (e: 'click', event: Event, entry: EntryModel): void;
  (e: 'menuClick', event: Event, entry: EntryModel, onFinished: () => void): void;
  (e: 'filesAdded', imports: FileImportTuple[]): void;
  (e: 'dropAsReader'): void;
  (e: 'select'): void;
  (e: 'update:modelValue', value: boolean): void;
}>();

defineExpose({
  props,
});

async function onCardClick(): Promise<void> {
  if (props.showCheckbox) {
    emits('update:modelValue', !props.entry.isSelected);
  }
}

function isFileSynced(): boolean {
  return !props.entry.needSync;
}

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
