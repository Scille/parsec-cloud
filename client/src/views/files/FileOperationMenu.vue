<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    v-if="menu.isVisible() || isFileOperationManagerActive"
    class="upload-menu"
    :class="menu.isMinimized() ? 'minimize' : ''"
  >
    <div class="upload-menu-header">
      <ion-text class="title-h4">{{ $msTranslate('FoldersPage.ImportFile.title') }}</ion-text>
      <div class="menu-header-icons">
        <ion-icon
          class="menu-header-icons__item"
          :icon="chevronDown"
          @click="toggleMenu()"
        />
        <ion-icon
          v-if="!isFileOperationManagerActive"
          class="menu-header-icons__item"
          :icon="close"
          @click="menu.hide()"
        />
      </div>
    </div>
    <ion-list class="upload-menu-tabs">
      <ion-item
        class="upload-menu-tabs__item body"
        :class="{ active: currentTab === Tabs.InProgress }"
        @click="currentTab = Tabs.InProgress"
      >
        <div class="item-container">
          <ion-text>{{ $msTranslate('FoldersPage.ImportFile.tabs.inProgress') }}</ion-text>
          <span class="text-counter">{{ inProgressItems.length > 99 ? '+99' : inProgressItems.length }}</span>
        </div>
      </ion-item>

      <ion-item
        class="upload-menu-tabs__item body"
        :class="{ active: currentTab === Tabs.Done }"
        @click="currentTab = Tabs.Done"
      >
        <div class="item-container">
          <ion-text>{{ $msTranslate('FoldersPage.ImportFile.tabs.done') }}</ion-text>
          <span class="text-counter">{{ doneItems.length > 99 ? '+99' : doneItems.length }}</span>
        </div>
      </ion-item>

      <ion-item
        class="upload-menu-tabs__item body"
        :class="{ active: currentTab === Tabs.Error }"
        @click="currentTab = Tabs.Error"
      >
        <div class="item-container">
          <ion-text>{{ $msTranslate('FoldersPage.ImportFile.tabs.failed') }}</ion-text>
          <span class="text-counter">{{ errorItems.length > 99 ? '+99' : errorItems.length }}</span>
        </div>
      </ion-item>
    </ion-list>

    <ion-list class="upload-menu-list">
      <template v-if="currentTab === Tabs.InProgress">
        <component
          v-for="item in inProgressItems"
          :is="getFileOperationComponent(item)"
          :key="item.data.id"
          :progress="item.progress"
          :state="item.state"
          :operation-data="item.data"
          @cancel="cancelOperation"
        />
        <div
          class="upload-menu-list__empty"
          v-if="inProgressItems.length === 0"
        >
          <ms-image :image="NoImportInProgress" />
          <ion-text class="body-lg">
            {{ $msTranslate('FoldersPage.ImportFile.noImportInProgress') }}
          </ion-text>
        </div>
      </template>
      <template v-if="currentTab === Tabs.Done">
        <component
          v-for="item in doneItems"
          :is="getFileOperationComponent(item)"
          :key="item.data.id"
          :progress="item.progress"
          :state="item.state"
          :operation-data="item.data"
          @cancel="cancelOperation"
          @click="onOperationFinishedClick"
        />
        <div
          class="upload-menu-list__empty"
          v-if="doneItems.length === 0"
        >
          <ms-image :image="NoImportDone" />
          <ion-text class="body-lg">
            {{ $msTranslate('FoldersPage.ImportFile.noImportDone') }}
          </ion-text>
        </div>
      </template>
      <template v-if="currentTab === Tabs.Error">
        <component
          v-for="item in errorItems"
          :is="getFileOperationComponent(item)"
          :key="item.data.id"
          :progress="item.progress"
          :state="item.state"
          :operation-data="item.data"
          @cancel="cancelOperation"
        />
        <div
          class="upload-menu-list__empty"
          v-if="errorItems.length === 0"
        >
          <ms-image :image="NoImportError" />
          <ion-text class="body-lg">
            {{ $msTranslate('FoldersPage.ImportFile.noImportFailed') }}
          </ion-text>
        </div>
      </template>
    </ion-list>
  </div>
</template>

<script setup lang="ts">
import { MsImage, NoImportDone, NoImportError, NoImportInProgress } from 'megashark-lib';
import { FileUploadItem, FileCopyItem, FileMoveItem } from '@/components/files';
import { navigateToWorkspace } from '@/router';
import useUploadMenu from '@/services/fileUploadMenu';
import {
  OperationProgressStateData,
  ImportData,
  FileOperationManager,
  FileOperationManagerKey,
  FileOperationData,
  FileOperationState,
  StateData,
  FileOperationDataType,
} from '@/services/fileOperationManager';
import { IonIcon, IonItem, IonList, IonText } from '@ionic/vue';
import { chevronDown, close } from 'ionicons/icons';
import { Ref, computed, inject, onMounted, onUnmounted, ref } from 'vue';
import type { Component } from 'vue';

interface OperationItem {
  data: FileOperationData;
  state: FileOperationState;
  progress: number;
}

const menu = useUploadMenu();

const fileOperationManager: FileOperationManager = inject(FileOperationManagerKey)!;
const fileOperations: Ref<Array<OperationItem>> = ref([]);
let dbId: string;
const isFileOperationManagerActive = ref(false);
const uploadMenuList = ref();

enum Tabs {
  InProgress,
  Done,
  Error,
}

const currentTab = ref(Tabs.InProgress);

const inProgressItems = computed(() => {
  return fileOperations.value.filter((op) =>
    [FileOperationState.OperationProgress, FileOperationState.FileAdded, FileOperationState.MoveAdded].includes(op.state),
  );
});

const doneItems = computed(() => {
  return fileOperations.value.filter((op) =>
    [FileOperationState.FileImported, FileOperationState.Cancelled, FileOperationState.EntryMoved].includes(op.state),
  );
});

const errorItems = computed(() => {
  return fileOperations.value.filter((op) => [FileOperationState.CreateFailed, FileOperationState.MoveFailed].includes(op.state));
});

function toggleMenu(): void {
  if (menu.isMinimized()) {
    menu.expand();
  } else {
    menu.minimize();
  }
}

onMounted(async () => {
  dbId = await fileOperationManager.registerCallback(onFileOperationEvent);
});

onUnmounted(async () => {
  await fileOperationManager.removeCallback(dbId);
});

function getFileOperationComponent(item: OperationItem): Component | undefined {
  switch (item.data.getDataType()) {
    case FileOperationDataType.Import:
      return FileUploadItem;
    case FileOperationDataType.Copy:
      return FileCopyItem;
    case FileOperationDataType.Move:
      return FileMoveItem;
  }
}

function updateImportState(id: string, state: FileOperationState, progress?: number): void {
  const operation = fileOperations.value.find((op) => op.data.id === id);
  if (operation) {
    operation.state = state;
    if (progress !== undefined) {
      operation.progress = progress;
    }
  }
}

async function onOperationFinishedClick(operationData: FileOperationData, state: FileOperationState): Promise<void> {
  if (state !== FileOperationState.FileImported) {
    return;
  }
  if (operationData.getDataType() === FileOperationDataType.Import) {
    await navigateToWorkspace(operationData.workspaceHandle, (operationData as ImportData).path, (operationData as ImportData).file.name);
  }
  menu.minimize();
}

async function cancelOperation(importId: string): Promise<void> {
  await fileOperationManager.cancelOperation(importId);
}

async function onFileOperationEvent(
  state: FileOperationState,
  fileOperationData?: FileOperationData,
  stateData?: StateData,
): Promise<void> {
  switch (state) {
    case FileOperationState.OperationAllStarted:
      isFileOperationManagerActive.value = true;
      currentTab.value = Tabs.InProgress;
      menu.show();
      menu.expand();
      break;
    case FileOperationState.OperationAllFinished:
      isFileOperationManagerActive.value = false;
      currentTab.value = Tabs.Done;
      break;
    case FileOperationState.FileAdded:
    case FileOperationState.MoveAdded:
      fileOperations.value.push({
        data: fileOperationData as FileOperationData,
        state: state,
        progress: 0,
      });
      if (uploadMenuList.value?.$el) {
        uploadMenuList.value.$el.scrollTo({ top: 0, behavior: 'smooth' });
      }
      break;
    case FileOperationState.OperationProgress:
      updateImportState((fileOperationData as FileOperationData).id, state, (stateData as OperationProgressStateData).progress);
      break;
    case FileOperationState.FileImported:
    case FileOperationState.EntryMoved:
      updateImportState((fileOperationData as FileOperationData).id, state, 100);
      break;
    case FileOperationState.CreateFailed:
    case FileOperationState.MoveFailed:
      updateImportState((fileOperationData as FileOperationData).id, state, -1);
      break;
    case FileOperationState.Cancelled:
      updateImportState((fileOperationData as FileOperationData).id, state);
  }
}
</script>

<style scoped lang="scss">
.upload-menu {
  display: flex;
  min-width: 25rem;
  position: absolute;
  border-radius: var(--parsec-radius-8) var(--parsec-radius-8) 0 0;
  box-shadow: var(--parsec-shadow-strong);
  bottom: 0;
  right: 2rem;
  overflow: hidden;
  display: flex;
  flex-direction: column;

  &-header {
    display: flex;
    align-items: center;
    height: fit-content;
    justify-content: space-between;
    width: 100%;
    padding: 0.25rem 0.25rem 0.25rem 1rem;
    background: var(--parsec-color-light-primary-800);
    color: var(--parsec-color-light-secondary-inversed-contrast);
  }

  &-tabs {
    display: flex;
    padding: 0.375rem 0.5rem 0rem 0.5rem;
    background-color: var(--parsec-color-light-secondary-premiere);
    overflow: hidden;

    &__item {
      --background: none;
      color: var(--parsec-color-light-secondary-grey);
      cursor: pointer;
      width: 100%;
      border-radius: var(--parsec-radius-8) var(--parsec-radius-8) 0 0;

      &::part(native) {
        padding: 0.5rem 0rem;
        --inner-padding-end: 0px;
      }

      &:hover {
        color: var(--parsec-color-light-primary-500);
      }

      .item-container {
        display: flex;
        justify-content: center;
        align-items: center;
        width: 100%;
        gap: 0.5rem;

        .text-counter {
          display: flex;
          flex-direction: column;
          align-items: baseline;
          line-height: normal;
          padding-inline: 0.25rem;
          background: var(--parsec-color-light-secondary-disabled);
          border-radius: var(--parsec-radius-12);
        }
      }

      &.active {
        box-shadow: var(--parsec-shadow-light);
        --background: var(--parsec-color-light-secondary-white);
        color: var(--parsec-color-light-primary-500);

        .text-counter {
          background: var(--parsec-color-light-primary-50);
        }
      }
    }
  }

  &-list {
    background-color: none;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding: 1em 0.5rem;
    overflow-y: auto;
    height: 40vh;
    max-height: 25rem;
    transition: all 250ms ease-in-out;

    &__empty {
      overflow: hidden;
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 0.5rem;
      margin: auto;
      color: var(--parsec-color-light-secondary-grey);
    }
  }
}

.menu-header-icons {
  display: flex;
  gap: 0.5rem;

  &__item {
    color: var(--parsec-color-light-secondary-inversed-contrast);
    font-size: 1.5rem;
    cursor: pointer;
    border-radius: var(--parsec-radius-8);
    padding: 0.5rem;

    &:nth-child(1) {
      transition: transform 250ms ease-in-out;
    }

    &:hover {
      background-color: var(--parsec-color-light-primary-30-opacity15);
    }
  }
}

.minimize {
  .upload-menu-list,
  .upload-menu-tabs {
    height: 0;
    padding: 0;
    margin: 0;
  }

  .menu-header-icons__item {
    transform: rotate(180deg);
  }
}
</style>
