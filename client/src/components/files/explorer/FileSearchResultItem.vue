<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    button
    :lines="isLargeDisplay ? 'full' : 'none'"
    :detail="false"
    class="file-list-item result-list-item"
    :class="{
      'file-list-item-mobile': isSmallDisplay,
      'file-hovered': menuOpened || isHovered,
    }"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
    @contextmenu="onOptionsClick"
  >
    <div class="list-item-container file-list-item-container">
      <!-- file name -->
      <div
        class="file-name file-name-results"
        :class="{ 'file-mobile-content': isSmallDisplay }"
      >
        <ms-image
          :image="searchItem.stats.isFile() ? getFileIcon(searchItem.stats.name) : Folder"
          class="file-icon"
        />
        <div class="file-name-content">
          <ion-text
            class="label-name can-highlight button-medium"
            :title="searchItem.stats.name"
            @click="onClick"
          >
            {{ searchItem.stats.name }}
          </ion-text>
          <div class="path-content">
            <ion-text
              class="workspace-path body-sm"
              :title="workspaceName"
            >
              {{ workspaceName }}
            </ion-text>
            <ion-text
              class="label-path can-highlight body-sm"
              :title="searchItem.parent"
            >
              {{ searchItem.parent }}
            </ion-text>
          </div>
          <div
            class="file-mobile-text"
            v-if="isSmallDisplay"
          >
            <ion-text class="file-mobile-text__data body-sm">
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
      <div class="file-last-update">
        <ion-text class="label-last-update cell">
          {{ $msTranslate(formatTimeSince(searchItem.stats.updated, '--', 'short')) }}
        </ion-text>
      </div>

      <!-- file size -->
      <div class="file-size">
        <ion-text
          v-if="searchItem.stats.isFile()"
          class="label-size cell"
        >
          {{ $msTranslate(formatFileSize((searchItem.stats as EntryStatFile).size)) }}
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
</template>

<script setup lang="ts">
import { formatFileSize, getFileIcon } from '@/common/file';
import { EntryName, EntryStatFile, SearchResult } from '@/parsec';
import { IonButton, IonIcon, IonItem, IonText } from '@ionic/vue';
import { cloudDone, cloudOffline, ellipsisHorizontal } from 'ionicons/icons';
import { Folder, formatTimeSince, MsImage, useWindowSize } from 'megashark-lib';
import { computed, ref } from 'vue';

const isHovered = ref(false);
const menuOpened = ref(false);
const { isSmallDisplay, isLargeDisplay } = useWindowSize();

const props = defineProps<{
  searchItem: SearchResult;
  workspaceName: EntryName;
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

  menuOpened.value = true;
  emits('menuClick', event, props.searchItem, () => (menuOpened.value = false));
}

async function onClick(): Promise<void> {
  emits('click', props.searchItem);
}
</script>

<style lang="scss" scoped>
.result-list-item {
  padding: 0.25rem 0;
}

.file-name {
  &-content {
    display: flex;
    flex-direction: column;
    overflow: hidden;
    gap: 0.25rem;
  }

  .path-content {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    overflow: hidden;
  }

  .workspace-path {
    color: var(--parsec-color-light-secondary-hard-grey);
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    border: 1px solid var(--parsec-color-light-secondary-medium);
    padding: 1px 0.25rem;
    border-radius: var(--parsec-radius-4);
    background: var(--parsec-color-light-secondary-background);
  }

  .label-path {
    color: var(--parsec-color-light-secondary-hard-grey);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
}
</style>
