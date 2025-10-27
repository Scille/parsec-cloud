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
        class="upload-menu-tabs__item button-medium"
        :class="{ active: currentTab === Tabs.InProgress }"
        @click="currentTab = Tabs.InProgress"
      >
        <div class="item-container">
          <ion-text>{{ $msTranslate('FoldersPage.ImportFile.tabs.inProgress') }}</ion-text>
          <span class="text-counter">{{ inProgressAndQueuedLength > 99 ? '+99' : inProgressAndQueuedLength }}</span>
        </div>
      </ion-item>

      <ion-item
        class="upload-menu-tabs__item button-medium"
        :class="{ active: currentTab === Tabs.Done }"
        @click="currentTab = Tabs.Done"
      >
        <div class="item-container">
          <ion-text>{{ $msTranslate('FoldersPage.ImportFile.tabs.done') }}</ion-text>
          <span class="text-counter">{{ doneItemsCount > 99 ? '+99' : doneItemsCount }}</span>
        </div>
      </ion-item>

      <ion-item
        class="upload-menu-tabs__item button-medium"
        :class="{ active: currentTab === Tabs.Error }"
        @click="currentTab = Tabs.Error"
      >
        <div class="item-container">
          <ion-text>{{ $msTranslate('FoldersPage.ImportFile.tabs.failed') }}</ion-text>
          <span class="text-counter">{{ errorItemsCount > 99 ? '+99' : errorItemsCount }}</span>
        </div>
      </ion-item>
    </ion-list>

    <ion-list class="upload-menu-list">
      <template v-if="currentTab === Tabs.InProgress">
        <file-aggregate-queued-item
          v-if="queuedUploadItems.length > 0"
          :amount="queuedUploadItems.length"
          :type="FileOperationDataType.Import"
          @cancel="cancelQueuedOperations(queuedUploadItems)"
        />
        <file-aggregate-queued-item
          v-if="queuedCopyItems.length > 0"
          :amount="queuedCopyItems.length"
          :type="FileOperationDataType.Copy"
          @cancel="cancelQueuedOperations(queuedCopyItems)"
        />
        <file-aggregate-queued-item
          v-if="queuedMoveItems.length > 0"
          :amount="queuedMoveItems.length"
          :type="FileOperationDataType.Move"
          @cancel="cancelQueuedOperations(queuedMoveItems)"
        />
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
          v-if="inProgressAndQueuedLength === 0"
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
        <file-aggregate-done-item
          v-if="doneItemsCount > MAX_DONE_ERROR_ITEMS"
          :amount="doneItemsCount - MAX_DONE_ERROR_ITEMS"
        />
        <div
          class="upload-menu-list__empty"
          v-if="doneItemsCount === 0"
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
        <file-aggregate-done-item
          v-if="errorItemsCount > MAX_DONE_ERROR_ITEMS"
          :amount="errorItemsCount - MAX_DONE_ERROR_ITEMS"
          :is-error="true"
        />
        <div
          class="upload-menu-list__empty"
          v-if="errorItemsCount === 0"
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
import { FIFO } from '@/common/queue';
import {
  FileAggregateDoneItem,
  FileAggregateQueuedItem,
  FileCopyItem,
  FileDownloadItem,
  FileMoveItem,
  FileRestoreItem,
  FileUploadItem,
} from '@/components/files';
import FileDownloadArchive from '@/components/files/FileDownloadArchive.vue';
import { navigateToWorkspace } from '@/router';
import {
  DownloadData,
  FileOperationData,
  FileOperationDataType,
  FileOperationManager,
  FileOperationManagerKey,
  FileOperationState,
  ImportData,
  StateData,
} from '@/services/fileOperationManager';
import useUploadMenu from '@/services/fileUploadMenu';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { IonIcon, IonItem, IonList, IonText } from '@ionic/vue';
import { chevronDown, close } from 'ionicons/icons';
import { DateTime } from 'luxon';
import { MsImage, NoImportDone, NoImportError, NoImportInProgress } from 'megashark-lib';
import type { Component } from 'vue';
import { computed, inject, onMounted, onUnmounted, ref } from 'vue';

interface OperationItem {
  data: FileOperationData;
  state: FileOperationState;
  stateData?: StateData;
  finishedDate?: DateTime;
}

const MAX_DONE_ERROR_ITEMS: number = 10;

const menu = useUploadMenu();
const informationManager: InformationManager = inject(InformationManagerKey)!;

const fileOperationManager: FileOperationManager = inject(FileOperationManagerKey)!;
const errorHappenedInUpload = ref<boolean>(false);
const inProgressItems = ref<Array<OperationItem>>([]);
const queuedCopyItems = ref<Array<OperationItem>>([]);
const queuedMoveItems = ref<Array<OperationItem>>([]);
const queuedUploadItems = ref<Array<OperationItem>>([]);
const queuedRestoreItems = ref<Array<OperationItem>>([]);
const queuedDownloadItems = ref<Array<OperationItem>>([]);
const queuedArchiveItems = ref<Array<OperationItem>>([]);

const doneItems = ref(new FIFO<OperationItem>(MAX_DONE_ERROR_ITEMS));
const errorItems = ref(new FIFO<OperationItem>(MAX_DONE_ERROR_ITEMS));
const doneItemsCount = ref<number>(0);
const errorItemsCount = ref<number>(0);

const inProgressAndQueuedLength = computed(() => {
  return (
    inProgressItems.value.length +
    queuedUploadItems.value.length +
    queuedCopyItems.value.length +
    queuedMoveItems.value.length +
    queuedRestoreItems.value.length +
    queuedDownloadItems.value.length +
    queuedArchiveItems.value.length
  );
});

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
    case FileOperationDataType.Restore:
      return FileRestoreItem;
    case FileOperationDataType.Download:
      return FileDownloadItem;
    case FileOperationDataType.DownloadArchive:
      return FileDownloadArchive;
  }
}

function updateImportState(data: FileOperationData, state: FileOperationState, stateData?: StateData): void {
  let index: number;
  const itemDataType: FileOperationDataType = data.getDataType();

  if (state === FileOperationState.OperationProgress) {
    // Check if item has to be moved from queued to inProgress
    if (!inProgressItems.value.find((op) => op.data.id === data.id)) {
      switch (itemDataType) {
        case FileOperationDataType.Import:
          index = queuedUploadItems.value.findIndex((op) => op.data.id === data.id);
          if (queuedUploadItems.value[index]) {
            const item = queuedUploadItems.value.splice(index, 1)[0];
            inProgressItems.value.push(item);
          }
          break;
        case FileOperationDataType.Copy:
          index = queuedCopyItems.value.findIndex((op) => op.data.id === data.id);
          if (queuedCopyItems.value[index]) {
            const item = queuedCopyItems.value.splice(index, 1)[0];
            inProgressItems.value.push(item);
          }
          break;
        case FileOperationDataType.Move:
          index = queuedMoveItems.value.findIndex((op) => op.data.id === data.id);
          if (queuedMoveItems.value[index]) {
            const item = queuedMoveItems.value.splice(index, 1)[0];
            inProgressItems.value.push(item);
          }
          break;
        case FileOperationDataType.Restore:
          index = queuedRestoreItems.value.findIndex((op) => op.data.id === data.id);
          if (queuedRestoreItems.value[index]) {
            const item = queuedRestoreItems.value.splice(index, 1)[0];
            inProgressItems.value.push(item);
          }
          break;
        case FileOperationDataType.Download:
          index = queuedDownloadItems.value.findIndex((op) => op.data.id === data.id);
          if (queuedDownloadItems.value[index]) {
            const item = queuedDownloadItems.value.splice(index, 1)[0];
            inProgressItems.value.push(item);
          }
          break;
        case FileOperationDataType.DownloadArchive:
          index = queuedArchiveItems.value.findIndex((op) => op.data.id === data.id);
          if (queuedArchiveItems.value[index]) {
            const item = queuedArchiveItems.value.splice(index, 1)[0];
            inProgressItems.value.push(item);
          }
          break;
      }
    }
  }

  index = inProgressItems.value.findIndex((op) => op.data.id === data.id);
  let currentOperationArray: Array<OperationItem> = inProgressItems.value;

  if (index === -1) {
    // The item is not in progress but might be queued
    switch (itemDataType) {
      case FileOperationDataType.Import:
        currentOperationArray = queuedUploadItems.value;
        break;
      case FileOperationDataType.Copy:
        currentOperationArray = queuedCopyItems.value;
        break;
      case FileOperationDataType.Move:
        currentOperationArray = queuedMoveItems.value;
        break;
      case FileOperationDataType.Restore:
        currentOperationArray = queuedRestoreItems.value;
        break;
      case FileOperationDataType.Download:
        currentOperationArray = queuedDownloadItems.value;
        break;
      case FileOperationDataType.DownloadArchive:
        currentOperationArray = queuedArchiveItems.value;
    }
    index = currentOperationArray.findIndex((op) => op.data.id === data.id);
  }

  const operation = currentOperationArray[index];

  if (operation) {
    operation.state = state;
    operation.stateData = stateData;
    if (
      [
        FileOperationState.FileImported,
        FileOperationState.EntryMoved,
        FileOperationState.EntryCopied,
        FileOperationState.EntryRestored,
        FileOperationState.EntryDownloaded,
        FileOperationState.ArchiveDownloaded,
      ].includes(state)
    ) {
      operation.finishedDate = DateTime.now();
      doneItems.value.push(operation);
      doneItemsCount.value += 1;
      currentOperationArray.splice(index, 1);
    } else if (
      [
        FileOperationState.CreateFailed,
        FileOperationState.MoveFailed,
        FileOperationState.CopyFailed,
        FileOperationState.ImportFailed,
        FileOperationState.Cancelled,
        FileOperationState.RestoreFailed,
        FileOperationState.DownloadFailed,
        FileOperationState.DownloadArchiveFailed,
      ].includes(state)
    ) {
      operation.finishedDate = DateTime.now();
      errorItems.value.push(operation);
      errorItemsCount.value += 1;
      currentOperationArray.splice(index, 1);
      errorHappenedInUpload.value = true;
    }
  }
}

async function onOperationFinishedClick(operationData: FileOperationData, state: FileOperationState): Promise<void> {
  if (state !== FileOperationState.FileImported && state !== FileOperationState.EntryDownloaded) {
    return;
  }
  if (operationData.getDataType() === FileOperationDataType.Import) {
    await navigateToWorkspace(operationData.workspaceHandle, (operationData as ImportData).path, (operationData as ImportData).file.name);
    menu.minimize();
  } else if (operationData.getDataType() === FileOperationDataType.Download) {
    try {
      const file = await (operationData as DownloadData).saveHandle.getFile();
      const url = URL.createObjectURL(file);
      window.open(url);
      menu.minimize();
    } catch (_err: any) {
      informationManager.present(
        new Information({
          message: 'FoldersPage.DownloadFile.openFailed',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    }
  }
}

async function cancelOperation(importId: string): Promise<void> {
  await fileOperationManager.cancelOperation(importId);
}

async function cancelQueuedOperations(queuedItems: Array<OperationItem>): Promise<void> {
  for (const operation of queuedItems) {
    await fileOperationManager.cancelOperation(operation.data.id);
  }
}

function scrollToTop(): void {
  if (uploadMenuList.value?.$el) {
    uploadMenuList.value.$el.scrollTo({ top: 0, behavior: 'smooth' });
  }
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
      queuedUploadItems.value.push({
        data: fileOperationData as FileOperationData,
        state: state,
        stateData: stateData,
      });
      scrollToTop();
      break;
    case FileOperationState.MoveAdded:
      queuedMoveItems.value.push({
        data: fileOperationData as FileOperationData,
        state: state,
        stateData: stateData,
      });
      scrollToTop();
      break;
    case FileOperationState.CopyAdded:
      queuedCopyItems.value.push({
        data: fileOperationData as FileOperationData,
        state: state,
        stateData: stateData,
      });
      scrollToTop();
      break;
    case FileOperationState.RestoreAdded:
      queuedRestoreItems.value.push({
        data: fileOperationData as FileOperationData,
        state: state,
        stateData: stateData,
      });
      scrollToTop();
      break;
    case FileOperationState.DownloadAdded:
      queuedDownloadItems.value.push({
        data: fileOperationData as FileOperationData,
        state: state,
        stateData: stateData,
      });
      scrollToTop();
      break;
    case FileOperationState.DownloadArchiveAdded:
      queuedArchiveItems.value.push({
        data: fileOperationData as FileOperationData,
        state: state,
        stateData: stateData,
      });
      scrollToTop();
      break;
    case FileOperationState.OperationProgress:
      updateImportState(fileOperationData as FileOperationData, state, stateData);
      break;
    case FileOperationState.FileImported:
    case FileOperationState.EntryMoved:
    case FileOperationState.EntryCopied:
    case FileOperationState.EntryRestored:
    case FileOperationState.EntryDownloaded:
    case FileOperationState.ArchiveDownloaded:
      updateImportState(fileOperationData as FileOperationData, state, stateData);
      break;
    case FileOperationState.CreateFailed:
    case FileOperationState.MoveFailed:
    case FileOperationState.CopyFailed:
    case FileOperationState.ImportFailed:
    case FileOperationState.RestoreFailed:
    case FileOperationState.DownloadFailed:
    case FileOperationState.DownloadArchiveFailed:
      updateImportState(fileOperationData as FileOperationData, state, stateData);
      break;
    case FileOperationState.Cancelled:
      updateImportState(fileOperationData as FileOperationData, state);
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
  background: var(--parsec-color-light-secondary-white);
  bottom: 0;
  right: 2rem;
  overflow: hidden;
  display: flex;
  flex-direction: column;

  @include ms.responsive-breakpoint('sm') {
    right: 0;
    min-width: 100%;
    width: 100%;
  }

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
    position: relative;
    --current-tab: 0;

    @include ms.responsive-breakpoint('sm') {
      background: var(--parsec-color-light-secondary-premiere);
      padding: 0.25rem;
      border-radius: var(--parsec-radius-24);
      margin: 1rem 0.5rem 0;
    }

    &__item {
      color: var(--parsec-color-light-secondary-grey);
      cursor: pointer;
      display: flex;
      align-items: center;
      justify-content: center;
      --padding-start: 0;
      --inner-padding-end: 0px;

      &::part(native) {
        padding: 1rem 0.75rem 0.5rem 0.75rem;
        background: transparent;

        @include ms.responsive-breakpoint('sm') {
          padding: 0.5rem 0.25rem;
        }

        @include ms.responsive-breakpoint('sm') {
          padding: 0.5rem 0;
        }
      }

      @include ms.responsive-breakpoint('sm') {
        width: 100%;
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

    &::before {
      content: '';
      position: absolute;
      bottom: 0;
      height: calc(100% - 0.5rem);
      width: calc(100% / 3 - 0.5rem);
      border-radius: var(--parsec-radius-24);
      background-color: var(--parsec-color-light-secondary-white);
      transition: transform 250ms ease-in-out;
    }

    &:has(.upload-menu-tabs__item:nth-child(1).active) {
      &::before {
        transform: translate(0, -0.25rem);
      }
    }

    &:has(.upload-menu-tabs__item:nth-child(2).active) {
      &::before {
        transform: translate(calc(100% + 0.5rem), -0.25rem);
      }
    }

    &:has(.upload-menu-tabs__item:nth-child(3).active) {
      &::before {
        transform: translate(calc(200% + 1rem), -0.25rem);
      }
    }
  }

  &-list {
    display: flex;
    flex-direction: column;
    padding: 0.5rem 0;
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
    font-size: 1.25rem;
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
