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
          <ion-text>{{ $msTranslate('FoldersPage.ImportFile.tabs.lastUploaded') }}</ion-text>
          <span class="text-counter">{{ doneItems.length() > 99 ? '+99' : doneItems.length() }}</span>
        </div>
      </ion-item>

      <ion-item
        class="upload-menu-tabs__item body"
        :class="{ active: currentTab === Tabs.Error }"
        @click="currentTab = Tabs.Error"
      >
        <div class="item-container">
          <ion-text>{{ $msTranslate('FoldersPage.ImportFile.tabs.failed') }}</ion-text>
          <span class="text-counter">{{ errorItems.length() > 99 ? '+99' : errorItems.length() }}</span>
        </div>
      </ion-item>
    </ion-list>

    <ion-list class="upload-menu-list">
      <template v-if="currentTab === Tabs.InProgress">
        <component
          v-for="item in inProgressItems"
          :is="getFileOperationComponent(item)"
          :key="item.data.id"
          :state="item.state"
          :state-data="item.stateData"
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
          v-for="item in doneItems.getEntries()"
          :is="getFileOperationComponent(item)"
          :key="item.data.id"
          :state="item.state"
          :state-data="item.stateData"
          :operation-data="item.data"
          @click="onOperationFinishedClick"
        />
        <div
          class="upload-menu-list__empty"
          v-if="doneItems.length() === 0"
        >
          <ms-image :image="NoImportDone" />
          <ion-text class="body-lg">
            {{ $msTranslate('FoldersPage.ImportFile.noImportDone') }}
          </ion-text>
        </div>
      </template>
      <template v-if="currentTab === Tabs.Error">
        <component
          v-for="item in errorItems.getEntries()"
          :is="getFileOperationComponent(item)"
          :key="item.data.id"
          :state="item.state"
          :state-data="item.stateData"
          :operation-data="item.data"
        />
        <div
          class="upload-menu-list__empty"
          v-if="errorItems.length() === 0"
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
import { Ref, inject, onMounted, onUnmounted, ref } from 'vue';
import type { Component } from 'vue';
import { DateTime } from 'luxon';
import { FIFO } from '@/common/queue';

interface OperationItem {
  data: FileOperationData;
  state: FileOperationState;
  stateData?: StateData;
  finishedDate?: DateTime;
}

const menu = useUploadMenu();

const fileOperationManager: FileOperationManager = inject(FileOperationManagerKey)!;
const errorHappenedInUpload: Ref<boolean> = ref(false);
const inProgressItems: Ref<Array<OperationItem>> = ref([]);
const doneItems = ref(new FIFO<OperationItem>(10));
const errorItems = ref(new FIFO<OperationItem>(10));

let dbId: string;
const isFileOperationManagerActive = ref(false);
const uploadMenuList = ref();

enum Tabs {
  InProgress,
  Done,
  Error,
}

const currentTab = ref(Tabs.InProgress);

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

function updateImportState(id: string, state: FileOperationState, stateData?: StateData): void {
  const index = inProgressItems.value.findIndex((op) => op.data.id === id);
  const operation = inProgressItems.value[index];
  if (operation) {
    operation.state = state;
    operation.stateData = stateData;
    if ([FileOperationState.FileImported, FileOperationState.EntryMoved, FileOperationState.EntryCopied].includes(state)) {
      operation.finishedDate = DateTime.now();
      doneItems.value.push(operation);
      inProgressItems.value.splice(index, 1);
    } else if (
      [
        FileOperationState.CreateFailed,
        FileOperationState.MoveFailed,
        FileOperationState.CopyFailed,
        FileOperationState.Cancelled,
      ].includes(state)
    ) {
      operation.finishedDate = DateTime.now();
      errorItems.value.push(operation);
      inProgressItems.value.splice(index, 1);
      errorHappenedInUpload.value = true;
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
      if (errorHappenedInUpload.value === true) {
        currentTab.value = Tabs.Error;
        errorHappenedInUpload.value = false;
      } else {
        currentTab.value = Tabs.Done;
      }
      break;
    case FileOperationState.FileAdded:
    case FileOperationState.MoveAdded:
    case FileOperationState.CopyAdded:
      inProgressItems.value.push({
        data: fileOperationData as FileOperationData,
        state: state,
        stateData: stateData,
      });
      if (uploadMenuList.value?.$el) {
        uploadMenuList.value.$el.scrollTo({ top: 0, behavior: 'smooth' });
      }
      break;
    case FileOperationState.OperationProgress:
      updateImportState((fileOperationData as FileOperationData).id, state, stateData);
      break;
    case FileOperationState.FileImported:
    case FileOperationState.EntryMoved:
    case FileOperationState.EntryCopied:
      updateImportState((fileOperationData as FileOperationData).id, state, stateData);
      break;
    case FileOperationState.CreateFailed:
    case FileOperationState.MoveFailed:
    case FileOperationState.CopyFailed:
      updateImportState((fileOperationData as FileOperationData).id, state, stateData);
      break;
    case FileOperationState.Cancelled:
      updateImportState((fileOperationData as FileOperationData).id, state);
  }
}
</script>

<style scoped lang="scss">
.upload-menu {
  display: flex;
  min-width: 28rem;
  max-width: 25rem;
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
    padding: 0;
    overflow: hidden;
    background: var(--parsec-color-light-secondary-white);

    &__item {
      color: var(--parsec-color-light-secondary-grey);
      cursor: pointer;

      &::part(native) {
        padding: 1rem 0.75rem 0.5rem 0.75rem;
        --inner-padding-end: 0px;
      }

      .item-container {
        display: flex;
        justify-content: center;
        align-items: center;
        width: 100%;
        gap: 0.375rem;

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

      &:hover {
        color: var(--parsec-color-light-secondary-text);

        .text-counter {
          background: var(--parsec-color-light-secondary-disabled);
        }
      }

      &.active {
        color: var(--parsec-color-light-primary-500);

        .text-counter {
          background: var(--parsec-color-light-primary-50);
        }
      }
    }
  }

  &-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding: 1em 0.5rem;
    overflow-y: auto;
    height: 40vh;
    max-height: 25rem;
    transition: all 250ms ease-in-out;
    background: var(--parsec-color-light-secondary-white);

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
