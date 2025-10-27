<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    button
    lines="full"
    :detail="false"
    :class="{
      selected: entry.isSelected,
      'file-hovered': !entry.isSelected && (menuOpened || isHovered),
    }"
    class="file-list-item"
    @dblclick="$emit('click', $event, entry)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
  >
    <div class="list-item-container">
      <div class="file-selected">
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
      <div class="file-name">
        <ms-image
          :image="entry.isFile() ? getFileIcon(entry.name) : Folder"
          class="file-icon"
        />
        <ion-label class="label-name cell">
          {{ entry.name }}
        </ion-label>
      </div>

      <!-- last update -->
      <div class="file-last-update">
        <ion-label class="label-last-update cell">
          {{ $msTranslate(formatTimeSince(entry.updated, '--', 'short')) }}
        </ion-label>
      </div>

      <!-- file size -->
      <div class="file-size">
        <ion-label
          v-if="entry.isFile()"
          class="label-size cell"
        >
          {{ $msTranslate(formatFileSize((entry as WorkspaceHistoryFileModel).size)) }}
        </ion-label>
      </div>

      <div class="label-space" />
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { formatFileSize, getFileIcon } from '@/common/file';
import { WorkspaceHistoryEntryModel, WorkspaceHistoryFileModel } from '@/components/files/types';
import { FsPath, Path } from '@/parsec';
import { IonItem, IonLabel } from '@ionic/vue';
import { Folder, MsCheckbox, MsImage, formatTimeSince } from 'megashark-lib';
import { Ref, onMounted, ref } from 'vue';

const isHovered = ref(false);
const menuOpened = ref(false);

const props = defineProps<{
  entry: WorkspaceHistoryEntryModel;
  showCheckbox: boolean;
}>();

defineEmits<{
  (e: 'click', event: Event, entry: WorkspaceHistoryEntryModel): void;
  (e: 'selectedChange', entry: WorkspaceHistoryEntryModel, checked: boolean): void;
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
</script>

<style lang="scss" scoped>
.file-selected {
  flex-shrink: 0;
  max-width: 2.5rem;
}

.file-name {
  position: relative;
  display: flex;
  gap: 1rem;

  .file-icon {
    width: 2rem;
    height: 2rem;
    flex-shrink: 0;

    @include ms.responsive-breakpoint('sm') {
      width: 1.75rem;
      height: 1.75rem;
    }
  }

  .label-name {
    color: var(--parsec-color-light-secondary-text);
    margin-left: 1em;
  }
}
</style>
