<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div
    class="upload-menu"
    :class="{
      minimize: menu.isMinimized(),
      'minimized-with-status': menu.isMinimized() && showUploadStatus,
    }"
  >
    <div
      class="upload-menu-header"
      @click="toggleMenu()"
    >
      <ion-text
        v-if="!menu.isMinimized() || (menu.isMinimized() && showUploadStatus)"
        class="upload-menu-header__title"
      >
        {{ $msTranslate('FoldersPage.ImportFile.title') }}
      </ion-text>
      <ion-icon
        class="menu-header-icons"
        :icon="menu.isMinimized() && !showUploadStatus ? sync : chevronDown"
      />
    </div>

    <upload-status
      v-if="showUploadStatus"
      class="upload-menu-status"
      :status="uploadProgressStatus"
      :rate-calculator="rateCalculator as TransferRateCalculator"
      @close="showUploadStatus = false"
    />

    <ion-list
      v-if="!menu.isMinimized()"
      class="upload-menu-tabs"
    >
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
        <ms-image :image="theme === Theme.Dark ? NoImportInProgressDark : NoImportInProgress" />
        <ion-text class="upload-menu-list__empty-text">
          {{ $msTranslate(items.length === 0 ? 'FoldersPage.ImportFile.noTasks' : 'FoldersPage.ImportFile.noCurrentTasks') }}
        </ion-text>
      </div>
    </ion-list>
  </div>
</template>

<script setup lang="ts">
import { TransferRateCalculator } from '@/common/transferRate';
import { FileOperationBase, FileOperationImport } from '@/components/files';
import UploadStatus from '@/components/files/operations/UploadStatus.vue';
import { Path, UploadProgress, WorkspaceID } from '@/parsec';
import { navigateTo, Routes } from '@/router';
import { EventData, EventDistributor, EventDistributorKey, Events, UploadProgressData } from '@/services/eventDistributor';
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
import { chevronDown, sync } from 'ionicons/icons';
import { MsImage, NoImportInProgress, NoImportInProgressDark, Theme, useTheme } from 'megashark-lib';
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
const { theme } = useTheme();

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
const uploadProgressStatus = ref<UploadProgress>({ totalBytes: 0, totalFiles: 0 });
const workspacesBacklog = new Map<WorkspaceID, UploadProgress>();
const rateCalculator = ref<TransferRateCalculator>(new TransferRateCalculator());
const showUploadStatus = ref(false);
const eventDistributor: Ref<EventDistributor> = inject(EventDistributorKey)!;
let eventRegisterKey: string | undefined = undefined;

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
  eventRegisterKey = await eventDistributor.value.registerCallback([Events.UploadProgress], onEvent);
});

onUnmounted(async () => {
  canceller.cancel();
  if (eventRegisterKey) {
    eventDistributor.value.removeCallback(eventRegisterKey);
  }
});

async function onEvent(event: Events, evData?: EventData): Promise<void> {
  if (event !== Events.UploadProgress || !evData) {
    return;
  }
  const data = evData as UploadProgressData;
  if (data.filesLeft === 0 && data.bytesLeft === 0) {
    workspacesBacklog.delete(data.workspaceId);
  } else {
    workspacesBacklog.set(data.workspaceId, { totalFiles: data.filesLeft, totalBytes: data.bytesLeft });
  }
  const total: UploadProgress = { totalBytes: 0, totalFiles: 0 };
  for (const backlog of workspacesBacklog.values()) {
    total.totalBytes += backlog.totalBytes;
    total.totalFiles += backlog.totalFiles;
  }
  uploadProgressStatus.value = total;
  rateCalculator.value.update(total.totalBytes, total.totalFiles);
  if (total.totalFiles > 0) {
    showUploadStatus.value = true;
  }
}

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
  flex-direction: column;
  width: 28rem;
  max-height: 28rem;
  position: absolute;
  border-radius: ms.radius('2xl');
  box-shadow: ms.shadow('elevation-2xl');
  background: ms.color('surface-base-default');
  border: ms.border('thin') solid ms.color('border-base-default');
  bottom: 1.5rem;
  right: 2rem;
  z-index: 20;
  overflow: hidden;
  transition:
    width 250ms ease-in-out,
    max-height 250ms ease-in-out;

  @include ms.responsive-breakpoint('sm') {
    right: 0;
    bottom: 0;
    width: 100%;
    border-radius: ms.radius('2xl') ms.radius('2xl') ms.radius('none') ms.radius('none');
  }

  &-header {
    display: flex;
    align-items: center;
    height: fit-content;
    justify-content: space-between;
    width: 100%;
    padding: ms.spacing('padding-sm');
    background: ms.color('popover-header-background');
    color: ms.color('text-on-color-heading');
    cursor: pointer;

    &:hover {
      .upload-menu-header__title {
        text-decoration: underline;
      }
    }

    &__title {
      @include ms.font('heading-h5');
      margin-left: 1rem;
      height: stretch;
      display: flex;
      width: 100%;
      align-items: center;
    }
  }

  &-status {
    min-width: 22rem;
    height: fit-content;
    flex-shrink: 0;
  }

  &-tabs {
    display: flex;
    padding: ms.spacing('padding-xl') ms.spacing('padding-lg') ms.spacing('padding-xs');
    flex-shrink: 0;
    width: 100%;
    gap: ms.spacing('gap-lg');
    background: ms.color('surface-base-default');
    position: relative;
    align-items: center;
    box-shadow: ms.shadow('elevation-md');

    @include ms.responsive-breakpoint('sm') {
      padding: ms.spacing('padding-xl');
    }

    &__item {
      color: ms.color('text-neutral-default');
      border: ms.border('thin') solid ms.color('border-base-default');
      background: ms.color('surface-base-default');
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

      &:hover {
        background: ms.color('surface-neutral-default-subtle-hover');
        color: ms.color('text-brand-default-hover');
      }
    }
  }

  &-list {
    display: flex;
    flex-direction: column;
    padding: ms.spacing('padding-none');
    overflow-y: auto;
    height: 60vh;
    transition: all 250ms ease-in-out;
    padding-bottom: ms.spacing('padding-lg');
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
      padding: ms.spacing('padding-3xl');
      margin: auto;
      color: ms.color('text-base-description');

      &-text {
        @include ms.font('body-lg-regular');
      }
    }
  }
}

.menu-header-icons {
  display: flex;
  gap: ms.spacing('gap-lg');
  color: ms.color('icon-on-color-heading');
  font-size: 1.25rem;
  cursor: pointer;
  border-radius: ms.radius('lg');
  padding: ms.spacing('padding-lg');
  transition: transform 250ms ease-in-out;

  &:hover {
    background-color: ms.color('surface-neutral-on-color');
  }
}

.minimize {
  width: 2.75rem;
  max-height: 2.75rem;
  cursor: default;

  @include ms.responsive-breakpoint('sm') {
    right: 1.5rem;
    bottom: 1.5rem;
    border-radius: ms.radius('2xl');
  }

  .upload-menu-list {
    padding: ms.spacing('padding-none');
    margin: 0;
    overflow: hidden;
    height: 0;
  }

  .menu-header-icons {
    transform: rotate(180deg);
    margin-left: auto;
  }
}

.minimized-with-status {
  width: 28rem;
  max-height: 6rem;

  .upload-menu-header {
    padding: ms.spacing('padding-sm');
  }

  .menu-header-icons {
    padding: ms.spacing('padding-lg');
  }
}
</style>
