<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    v-if="menu.isVisible() || isFileOperationManagerActive"
    class="upload-menu"
    :class="menu.isMinimized() ? 'minimize' : ''"
  >
    <div class="upload-menu-header">
      <ion-text class="upload-menu-header__title">{{ $msTranslate('FoldersPage.ImportFile.title') }}</ion-text>
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
        class="upload-menu-tabs__item"
        @click="onFilterSelected(OperationFilter.InProgress)"
        :class="filter === OperationFilter.InProgress ? 'active' : ''"
      >
        <div class="item-container">
          <ion-text>{{ $msTranslate('FoldersPage.ImportFile.tabs.inProgress') }}</ion-text>
        </div>
      </ion-item>
      <ion-item
        class="upload-menu-tabs__item"
        @click="onFilterSelected(OperationFilter.Done)"
        :class="filter === OperationFilter.Done ? 'active' : ''"
      >
        <div class="item-container">
          <ion-text>{{ $msTranslate('FoldersPage.ImportFile.tabs.done') }}</ion-text>
        </div>
      </ion-item>
      <ion-item
        class="upload-menu-tabs__item"
        @click="onFilterSelected(OperationFilter.Error)"
        :class="filter === OperationFilter.Error ? 'active' : ''"
      >
        <div class="item-container">
          <ion-text>{{ $msTranslate('FoldersPage.ImportFile.tabs.failed') }}</ion-text>
        </div>
      </ion-item>
      <ion-button
        @click="onClearClicked"
        fill="clear"
        size="small"
        class="upload-menu-tabs__clear"
      >
        {{ $msTranslate('FoldersPage.ImportFile.tabs.clear') }}
      </ion-button>
    </ion-list>

    <ion-list class="upload-menu-list">
      <component
        v-for="item in currentItems"
        :is="getOperationComponent(item)"
        :key="item.operationData.id + item.refreshKey"
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
        <ion-text class="upload-menu-list__empty-text">
          {{ $msTranslate(items.length === 0 ? 'FoldersPage.ImportFile.noTasks' : 'FoldersPage.ImportFile.noCurrentTasks') }}
        </ion-text>
      </div>
    </ion-list>
  </div>
</template>

<script setup lang="ts">
import { FileOperationBase, FileOperationImport } from '@/components/files';
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
import { computed, inject, onMounted, onUnmounted, ref, Ref } from 'vue';

interface OperationItem {
  operationData: FileOperationData;
  status: FileOperationEvents;
  eventData?: FileOperationEventData;
  refreshKey: number;
}

enum OperationFilter {
  Done = 'done',
  InProgress = 'in-progress',
  Error = 'error',
}

const menu = useUploadMenu();

const fileOperationManager: Ref<FileOperationManager> = inject(FileOperationManagerKey)!;

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
    case FileOperationDataType.Copy:
    case FileOperationDataType.Move:
    case FileOperationDataType.Restore:
    default:
      return FileOperationBase;
  }
}

onMounted(async () => {
  canceller = await fileOperationManager.value.registerCallback(onFileOperationEvent);
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
      window.nativeAPI.log('warn', `Got event ${event} without operation data`);
    } else {
      isFileOperationManagerActive.value = false;
    }
    return;
  }
  switch (event) {
    case FileOperationEvents.Added: {
      items.value.unshift({ operationData: operationData, status: event, eventData: eventData, refreshKey: 0 });
      menu.show();
      menu.expand();
      scrollToTop();
      filter.value = undefined;
      break;
    }
    case FileOperationEvents.Updated: {
      const index = items.value.findIndex((op) => op.operationData.id === operationData.id);
      if (index !== -1) {
        items.value[index].operationData = operationData;
        items.value[index].refreshKey += 1;
      }
      break;
    }
    case FileOperationEvents.Removed: {
      const index = items.value.findIndex((op) => op.operationData.id === operationData.id);
      if (index !== -1) {
        items.value.splice(index, 1);
      }
      break;
    }
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

    if (op.files.length === 1) {
      const file = op.files.at(0) as File;
      const fullPath = await Path.joinPaths(op.destination, (file as any).relativePath);
      const parent = await Path.parent(fullPath);
      await navigateTo(Routes.Documents, {
        query: {
          workspaceHandle: operation.workspaceHandle,
          documentPath: parent,
          selectFile: file.name,
        },
      });
    } else {
      await navigateTo(Routes.Documents, {
        query: {
          workspaceHandle: operation.workspaceHandle,
          documentPath: op.destination,
          selectFile: op.files.length === 1 ? op.files.at(0)?.name : undefined,
        },
      });
    }
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
  await fileOperationManager.value.cancelOperation(operation.id);
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
  border-radius: ms.radius('lg') ms.radius('lg') 0 0;
  box-shadow: ms.shadow('elevation-2xl');
  background: ms.color('surface-base-default');
  border: 1px solid ms.color('border-base-default');
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
    padding: ms.spacing('padding-sm') ms.spacing('padding-sm') ms.spacing('padding-sm') ms.spacing('padding-3xl');
    background: var(--parsec-color-popover-header-background);
    color: var(--parsec-color-light-secondary-inversed-contrast);

    &__title {
      @include ms.font('heading-h5');
    }

    .menu-header-icons {
      display: flex;
      gap: ms.spacing('gap-lg');

      &__item {
        color: var(--parsec-color-light-secondary-inversed-contrast);
        font-size: 1.25rem;
        cursor: pointer;
        border-radius: ms.radius('lg');
        padding: ms.spacing('padding-lg');

        &:nth-child(1) {
          transition: transform 250ms ease-in-out;
        }

        &:hover {
          background-color: var(--parsec-color-light-primary-30-opacity15);
        }
      }
    }
  }

  &-tabs {
    display: flex;
    padding: ms.spacing('padding-xl') ms.spacing('padding-lg');
    gap: ms.spacing('gap-lg');
    overflow: hidden;
    background: ms.color('surface-base-default');
    position: relative;
    --current-tab: 0;
    align-items: center;

    @include ms.responsive-breakpoint('sm') {
      padding: ms.spacing('padding-sm');
      margin: 1rem 0.5rem 0;
    }

    &__item {
      color: ms.color('text-neutral-default');
      border: ms.border('thin') solid ms.color('border-base-default');
      border-radius: ms.radius('2xl');
      cursor: pointer;
      display: flex;
      align-items: center;
      justify-content: center;
      @include ms.font('label-md-medium');
      --padding-start: #{ms.spacing('padding-none')};
      --inner-padding-end: #{ms.spacing('padding-none')};
      transition: all 150ms ease-in-out;
      opacity: ms.opacity('5');

      &::part(native) {
        background: transparent;
        padding: ms.spacing('padding-lg') ms.spacing('padding-xl');

        @include ms.responsive-breakpoint('sm') {
          padding: ms.spacing('padding-lg') ms.spacing('padding-sm');
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
        gap: ms.spacing('gap-md');
      }

      &:hover {
        color: ms.color('text-neutral-default-hover');
        border: ms.border('thin') solid ms.color('border-base-default-hover');
        opacity: ms.opacity('10');
      }

      &.active {
        color: ms.color('text-brand-default');
        border: ms.border('thin') solid ms.color('border-brand-default');
        background: ms.color('surface-brand-default-subtle-hover');
        opacity: ms.opacity('10');
      }
    }

    &__clear {
      margin-left: auto;

      @include ms.responsive-breakpoint('sm') {
        margin-left: 0;
      }
    }
  }

  &-list {
    display: flex;
    flex-direction: column;
    padding: ms.spacing('padding-none');
    overflow-y: auto;
    height: 60vh;
    padding-bottom: ms.spacing('padding-5xl');
    max-height: 28rem;
    transition: all 250ms ease-in-out;
    background: ms.color('surface-base-default');

    @media screen and (max-height: 1000px) {
      height: 40vh;
    }

    &__empty {
      overflow: hidden;
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: ms.spacing('gap-lg');
      margin: auto;
      color: var(--parsec-color-light-secondary-grey);

      &-text {
        @include ms.font('body-lg-regular');
      }
    }
  }
}

.minimize {
  .upload-menu-list,
  .upload-menu-tabs {
    height: 0;
    padding: ms.spacing('padding-none');
    margin: 0;
  }

  .menu-header-icons__item {
    transform: rotate(180deg);
  }
}
</style>
