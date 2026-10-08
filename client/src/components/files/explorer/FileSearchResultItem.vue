<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    button
    :lines="isLargeDisplay ? 'full' : 'none'"
    :detail="false"
    class="list-item file-list-item result-list-item"
    :class="{
      'file-list-item-mobile': isSmallDisplay,
      'file-list-item--hovered': menuOpened || isHovered,
    }"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
    @contextmenu="onOptionsClick"
  >
    <div class="list-item-container file-list-item-container">
      <!-- file name -->
      <div
        class="list-item-column file-name file-name-results"
        :class="{ 'file-mobile-content': isSmallDisplay }"
      >
        <ms-image
          :image="searchItem.stats.isFile() ? getFileIcon(searchItem.stats.name) : Folder"
          class="file-icon"
        />
        <div class="file-name-content">
          <ion-text
            class="list-item-label label-name"
            :title="searchItem.stats.name"
            @click="onClick"
          >
            <!-- Those strings have been escaped by the search function -->
            <!-- eslint-disable vue/no-v-html -->
            <span
              v-if="searchItem.highlightedName"
              v-html="searchItem.highlightedName"
            />
            <!-- eslint-enable vue/no-v-html -->
            <span v-else>{{ searchItem.stats.name }}</span>
          </ion-text>
          <div class="path-content">
            <ion-text
              class="workspace-path"
              :title="searchItem.workspaceName"
            >
              {{ searchItem.workspaceName }}
            </ion-text>
            <ion-text
              class="label-path can-highlight"
              :title="searchItem.parent"
            >
              <!-- Those strings have been escaped by the search function -->
              <!-- eslint-disable vue/no-v-html -->
              <span
                v-if="searchItem.highlightedPath"
                v-html="searchItem.highlightedPath"
              />
              <!-- eslint-enable vue/no-v-html -->
              <span v-else>{{ searchItem.parent }}</span>
            </ion-text>
          </div>
          <div
            class="file-mobile-text"
            v-if="isSmallDisplay"
          >
            <ion-text class="file-mobile-text__data">
              <span class="data-date">{{ $msTranslate(formatTimeSince(searchItem.stats.updated, '--', 'short')) }}</span>
              <span v-if="searchItem.stats.isFile()"> &bull; </span>
              <span
                class="data-size"
                v-if="searchItem.stats.isFile()"
              >
                {{ $msTranslate(formatFileSize((searchItem.stats as EntryStatFile).size)) }}
              </span>
            </ion-text>
          </div>
        </div>
        <ion-icon
          class="cloud-overlay"
          :class="syncStatus.class"
          :icon="syncStatus.icon"
        />
      </div>

      <!-- last update -->
      <div class="list-item-column file-last-update">
        <ion-text class="list-item-label label-last-update cell">
          {{ $msTranslate(formatTimeSince(searchItem.stats.updated, '--', 'short')) }}
        </ion-text>
      </div>

      <!-- file size -->
      <div class="list-item-column file-size">
        <ion-text
          v-if="searchItem.stats.isFile()"
          class="list-item-label label-size cell"
        >
          {{ $msTranslate(formatFileSize((searchItem.stats as EntryStatFile).size)) }}
        </ion-text>
      </div>

      <!-- options -->
      <div
        class="list-item-end file-options ion-item-child-clickable"
        v-if="!disableContextMenu"
      >
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
</template>

<script setup lang="ts">
import { formatFileSize, getFileIcon } from '@/common/file';
import { EntryStatFile, SearchResult } from '@/parsec';
import { IonButton, IonIcon, IonItem, IonText } from '@ionic/vue';
import { cloudDone, cloudOffline, ellipsisHorizontal } from 'ionicons/icons';
import { Folder, formatTimeSince, MsImage, useWindowSize } from 'megashark-lib';
import { computed, ref } from 'vue';

const isHovered = ref(false);
const menuOpened = ref(false);
const { isSmallDisplay, isLargeDisplay } = useWindowSize();

const props = defineProps<{
  searchItem: SearchResult;
  disableContextMenu?: boolean;
}>();

const emits = defineEmits<{
  (e: 'click', entry: SearchResult): void;
  (e: 'menuClick', event: Event, entry: SearchResult, onFinished: () => void): void;
}>();

const syncStatus = computed(() => {
  if (props.searchItem.stats.needSync) {
    return { class: 'cloud-overlay-ko', icon: cloudOffline };
  } else {
    return { class: 'cloud-overlay-ok', icon: cloudDone };
  }
});

async function onOptionsClick(event: PointerEvent): Promise<void> {
  event.preventDefault();
  event.stopPropagation();

  if (props.disableContextMenu) {
    return;
  }

  menuOpened.value = true;
  emits('menuClick', event, props.searchItem, () => (menuOpened.value = false));
}

async function onClick(): Promise<void> {
  emits('click', props.searchItem);
}
</script>

<style lang="scss" scoped>
:deep(.highlight) {
  color: ms.color('text-brand-default');
  font-weight: bold;
}

.file-name {
  &-content {
    display: flex;
    flex-direction: column;
    overflow: hidden;
    gap: ms.spacing('gap-sm');
  }

  .path-content {
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-sm');
    overflow: hidden;
  }

  .workspace-path {
    @include ms.font('body-sm-regular');
    color: ms.color('text-base-description');
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    border: ms.border('thin') solid ms.color('border-neutral-default-subtle-hover');
    padding: 1px ms.spacing('padding-sm');
    border-radius: ms.radius('sm');
    background: ms.color('surface-base-default-secondary');
  }

  .label-path {
    @include ms.font('body-sm-regular');
    color: ms.color('text-base-description');
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
}
</style>
