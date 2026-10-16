<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-item
    button
    lines="full"
    :detail="false"
    :class="{
      selected: selected,
      'file-list-item--hovered': !selected && isHovered,
      'is-folder': !entry.isFile(),
    }"
    class="list-item file-list-item history-file-list-item"
    @click.stop="toggleSelected()"
    @dblclick="$emit('click', $event, entry)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
  >
    <div class="list-item-container">
      <div class="list-item-column file-selected">
        <ms-checkbox
          :checked="selected"
          v-show="(selected || isHovered || showCheckbox) && !readOnly"
          @change="toggleSelected()"
          @click.stop
          @dblclick.stop
        />
      </div>

      <!-- file name -->
      <div class="list-item-column file-name">
        <ms-image
          :image="entry.isFile() ? getFileIcon(entry.name) : Folder"
          class="file-icon"
        />
        <ion-label
          class="list-item-label label-name cell"
          :class="{ selection: showCheckbox }"
          @click.stop="showCheckbox ? toggleSelected() : $emit('click', $event, entry)"
        >
          {{ entry.name }}
        </ion-label>
      </div>

      <!-- last update -->
      <div class="list-item-column file-last-update">
        <ion-label class="list-item-label label-last-update cell">
          {{ $msTranslate(formatTimeSince(entry.updated, '--', 'short')) }}
        </ion-label>
      </div>

      <!-- file size -->
      <div class="list-item-column file-size">
        <ion-label
          v-if="entry.isFile()"
          class="list-item-label label-size cell"
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
import { IonItem, IonLabel } from '@ionic/vue';
import { Folder, MsCheckbox, MsImage, formatTimeSince } from 'megashark-lib';
import { ref } from 'vue';

const isHovered = ref(false);

const props = defineProps<{
  entry: WorkspaceHistoryEntryModel;
  showCheckbox: boolean;
  readOnly: boolean;
  selected: boolean;
}>();

const emits = defineEmits<{
  (e: 'click', event: Event, entry: WorkspaceHistoryEntryModel): void;
  (e: 'update:selected', value: boolean): void;
}>();

defineExpose({
  isHovered,
  props,
});

function toggleSelected(): void {
  if (props.readOnly) {
    return;
  }
  emits('update:selected', !props.selected);
}
</script>

<style lang="scss" scoped>
.history-file-list-item {
  .list-item-container:has(.label-name:not(.selection):hover) .ms-checkbox {
    outline: none !important;
  }
}

.file-selected {
  overflow: visible;
  width: 2.5rem;

  @include ms.responsive-breakpoint('sm') {
    flex-basis: 2.5rem;
  }
}

.file-name {
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

    &:not(.selection):hover {
      cursor: pointer;
      text-decoration: underline;
    }
  }
}
</style>
