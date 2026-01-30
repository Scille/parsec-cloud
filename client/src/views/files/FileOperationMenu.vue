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
        @click="onFilterSelected(OperationFilter.InProgress)"
        :class="filter === OperationFilter.InProgress ? 'active' : ''"
      >
        <div class="item-container">
          <ion-text>{{ $msTranslate('FoldersPage.ImportFile.tabs.inProgress') }}</ion-text>
        </div>
      </ion-item>
      <ion-item
        class="upload-menu-tabs__item button-medium"
        @click="onFilterSelected(OperationFilter.Done)"
        :class="filter === OperationFilter.Done ? 'active' : ''"
      >
        <div class="item-container">
          <ion-text>{{ $msTranslate('FoldersPage.ImportFile.tabs.done') }}</ion-text>
        </div>
      </ion-item>
      <ion-item
        class="upload-menu-tabs__item button-medium"
        @click="onFilterSelected(OperationFilter.Error)"
        :class="filter === OperationFilter.Error ? 'active' : ''"
      >
        <div class="item-container">
          <ion-text>{{ $msTranslate('FoldersPage.ImportFile.tabs.failed') }}</ion-text>
        </div>
      </ion-item>
      <ion-button
        @click="onClearClicked"
        fill="outline"
        class="upload-menu-tabs__delete"
      >
        {{ $msTranslate('FoldersPage.ImportFile.tabs.clear') }}
      </ion-button>
    </ion-list>

    <ion-list class="upload-menu-list">
      <component
        v-for="item in currentItems"
        :is="getOperationComponent(item)"
        :key="item.operationData.id"
        :operation-data="item.operationData"
        :status="item.status"
        :event-data="item.eventData"
        @cancel="onOperationCancelClick"
        @click="onOperationClick"
      />
      <div
        class="upload-menu-list__empty"
        v-if="currentItems.length === 0"
      >
        <ms-image :image="NoImportInProgress" />
        <ion-text class="body-lg">
          {{ $msTranslate(items.length === 0 ? 'FoldersPage.ImportFile.noTasks' : 'FoldersPage.ImportFile.noCurrentTasks') }}
        </ion-text>
      </div>
    </ion-list>
  </div>
</template>

<script setup lang="ts">
import { FileOperationBase, FileOperationDownloadArchive, FileOperationImport } from '@/components/files';
import { Path } from '@/parsec';
import { navigateTo, Routes } from '@/router';
import {
  FileOperationCopyData,
  FileOperationData,
  FileOperationDataType,
  FileOperationImportData,
  FileOperationMoveData,
  FileOperationRestoreData,
} from '@/services/fileOperation';
import { FileEventRegistrationCanceller, FileOperationEventData, FileOperationEvents } from '@/services/fileOperation/events';
import { FileOperationManager, FileOperationManagerKey } from '@/services/fileOperation/manager';
import useUploadMenu from '@/services/fileUploadMenu';
import { IonButton, IonIcon, IonItem, IonList, IonText } from '@ionic/vue';
import { chevronDown, close } from 'ionicons/icons';
import { MsImage, NoImportInProgress } from 'megashark-lib';
import type { Component } from 'vue';
import { computed, inject, onMounted, onUnmounted, ref } from 'vue';

interface OperationItem {
  operationData: FileOperationData;
  status: FileOperationEvents;
  eventData?: FileOperationEventData;
}

enum OperationFilter {
  Done = 'done',
  InProgress = 'in-progress',
  Error = 'error',
}

const menu = useUploadMenu();

const fileOperationManager: FileOperationManager = inject(FileOperationManagerKey)!;

const items = ref<Array<OperationItem>>([]);

const filter = ref<OperationFilter | undefined>(undefined);

const currentItems = computed(() => {
  if (!filter.value) {
    return items.value;
  }
  return items.value.filter((item) => {
    if (
      filter.value === OperationFilter.InProgress &&
      [FileOperationEvents.Added, FileOperationEvents.Progress, FileOperationEvents.Started].includes(item.status)
    ) {
      return true;
    } else if (filter.value === OperationFilter.Done && item.status === FileOperationEvents.Finished) {
      return true;
    } else if (
      filter.value === OperationFilter.Error &&
      [FileOperationEvents.Cancelled, FileOperationEvents.Failed].includes(item.status)
    ) {
      return true;
    }
    return false;
  });
});

let canceller!: FileEventRegistrationCanceller;
const isFileOperationManagerActive = ref(false);
const uploadMenuList = ref();

function toggleMenu(): void {
  if (menu.isMinimized()) {
    menu.expand();
  } else {
    menu.minimize();
  }
}

function onFilterSelected(newFilter: OperationFilter): void {
  if (filter.value === newFilter) {
    filter.value = undefined;
  } else {
    filter.value = newFilter;
  }
}

function getOperationComponent(item: OperationItem): Component {
  switch (item.operationData.type) {
    case FileOperationDataType.Import:
      return FileOperationImport;
    case FileOperationDataType.DownloadArchive:
      return FileOperationDownloadArchive;
    case FileOperationDataType.Copy:
    case FileOperationDataType.Move:
    case FileOperationDataType.Restore:
    case FileOperationDataType.Download:
    default:
      return FileOperationBase;
  }
}

onMounted(async () => {
  canceller = await fileOperationManager.registerCallback(onFileOperationEvent);
});

onUnmounted(async () => {
  canceller.cancel();
});

async function onFileOperationEvent(
  event: FileOperationEvents,
  operationData?: FileOperationData,
  eventData?: FileOperationEventData,
): Promise<void> {
  isFileOperationManagerActive.value = true;
  if (!operationData) {
    if (event !== FileOperationEvents.AllFinished) {
      window.electronAPI.log('warn', `Got event ${event} without operation data`);
    } else {
      isFileOperationManagerActive.value = false;
    }
    return;
  }
  switch (event) {
    case FileOperationEvents.Added:
      items.value.unshift({ operationData: operationData, status: event, eventData: eventData });
      menu.show();
      menu.expand();
      scrollToTop();
      filter.value = undefined;
      break;
    case FileOperationEvents.Cancelled:
    case FileOperationEvents.Failed:
    case FileOperationEvents.Finished:
    case FileOperationEvents.Finalizing:
    case FileOperationEvents.Progress:
    case FileOperationEvents.Started: {
      const operation = items.value.find((item) => item.operationData.id === operationData.id);
      if (operation) {
        operation.status = event;
        operation.eventData = eventData;
      }
      break;
    }
    default:
      break;
  }
}

async function onClearClicked(): Promise<void> {
  items.value = items.value.filter((item) =>
    [FileOperationEvents.Progress, FileOperationEvents.Started, FileOperationEvents.Added].includes(item.status),
  );
  filter.value = undefined;
}

async function onOperationClick(
  operation: FileOperationData,
  status: FileOperationEvents,
  _eventData?: FileOperationEventData,
): Promise<void> {
  if (status !== FileOperationEvents.Finished) {
    return;
  }
  if (operation.type === FileOperationDataType.Import) {
    const op = operation as FileOperationImportData;
    await navigateTo(Routes.Documents, {
      query: {
        workspaceHandle: operation.workspaceHandle,
        documentPath: op.destination,
        selectFile: op.files.length === 1 ? op.files.at(0)?.name : undefined,
      },
    });
  } else if (operation.type === FileOperationDataType.Move || operation.type === FileOperationDataType.Copy) {
    const op = operation as FileOperationMoveData | FileOperationCopyData;
    await navigateTo(Routes.Documents, {
      query: {
        workspaceHandle: operation.workspaceHandle,
        documentPath: op.destination,
        selectFile: op.sources.length === 1 ? op.sources.at(0)?.name : undefined,
      },
    });
  } else if (operation.type === FileOperationDataType.Restore && (operation as FileOperationRestoreData).entries.length === 1) {
    const op = operation as FileOperationRestoreData;
    await navigateTo(Routes.Documents, {
      query: {
        workspaceHandle: operation.workspaceHandle,
        documentPath: await Path.parent(op.entries[0].path),
        selectFile: op.entries.at(0)?.name,
      },
    });
  }
}

async function onOperationCancelClick(operation: FileOperationData): Promise<void> {
  await fileOperationManager.cancelOperation(operation.id);
}

function scrollToTop(): void {
  if (uploadMenuList.value?.$el) {
    uploadMenuList.value.$el.scrollTo({ top: 0, behavior: 'smooth' });
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
  z-index: 20;

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
    padding: 0.625rem 0.5rem;
    gap: 0.5rem;
    overflow: hidden;
    background: var(--parsec-color-light-secondary-white);
    position: relative;
    --current-tab: 0;
    align-items: center;

    @include ms.responsive-breakpoint('sm') {
      padding: 0.25rem;
      margin: 1rem 0.5rem 0;
    }

    &__item {
      color: var(--parsec-color-light-secondary-grey);
      border: 1px solid var(--parsec-color-light-secondary-medium);
      border-radius: var(--parsec-radius-12);
      cursor: pointer;
      display: flex;
      align-items: center;
      justify-content: center;
      --padding-start: 0;
      --inner-padding-end: 0px;
      transition: all 150ms ease-in-out;

      &::part(native) {
        background: transparent;
        padding: 0.5rem 0.625rem;

        @include ms.responsive-breakpoint('sm') {
          padding: 0.5rem 0.25rem;
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
      }

      &:hover {
        color: var(--parsec-color-light-secondary-hard-grey);
        border: 1px solid var(--parsec-color-light-secondary-light);
      }

      &.active {
        color: var(--parsec-color-light-primary-600);
        border: 1px solid var(--parsec-color-light-primary-600);
        background: var(--parsec-color-light-primary-50);

        &:hover {
          color: var(--parsec-color-light-primary-700);
          background: var(--parsec-color-light-primary-100);
          border: 1px solid var(--parsec-color-light-primary-600);
        }
      }
    }

    &__delete {
      margin-left: auto;
      background: var(--parsec-color-light-secondary-inversed-contrast);
      border-radius: var(--parsec-radius-12);
      transition: all 150ms ease-in-out;

      &::part(native) {
        background: none;
        box-shadow: var(--parsec-shadow-input);
        border: 1px solid var(--parsec-color-light-secondary-medium);
        color: var(--parsec-color-light-primary-600);
        padding: 0.5rem 0.75rem;
      }

      @include ms.responsive-breakpoint('sm') {
        margin-left: 0;
      }

      &:hover {
        background: var(--parsec-color-light-secondary-premiere);
        color: var(--parsec-color-light-primary-700);
      }
    }
  }

  &-list {
    display: flex;
    flex-direction: column;
    padding: 0;
    overflow-y: auto;
    height: 60vh;
    padding-bottom: 2rem;
    max-height: 28rem;
    transition: all 250ms ease-in-out;
    background: var(--parsec-color-light-secondary-white);

    @media screen and (max-height: 1000px) {
      height: 40vh;
    }

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
