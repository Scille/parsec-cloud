<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    v-if="shouldBeVisible || isImportManagerActive"
    class="upload-menu"
    :class="menuMinimized ? 'minimize' : ''"
  >
    <div class="upload-menu-header">
      <ion-text class="title-h4">{{ $t('FoldersPage.ImportFile.title') }}</ion-text>
      <div class="menu-header-icons">
        <ion-icon
          class="menu-header-icons__item"
          :icon="chevronDown"
          @click="toggleMenu()"
        />
        <ion-icon
          v-if="!isImportManagerActive"
          class="menu-header-icons__item"
          :icon="close"
          @click="shouldBeVisible = false"
        />
      </div>
    </div>
    <ion-list
      class="upload-menu-list"
      ref="uploadMenuList"
    >
      <file-upload-item
        v-for="importItem in imports"
        :key="importItem.data.id"
        :progress="importItem.progress"
        :state="importItem.state"
        :import-data="importItem.data"
        @import-cancel="cancelImport($event)"
      />
    </ion-list>
  </div>
</template>

<script setup lang="ts">
import FileUploadItem from '@/components/files/FileUploadItem.vue';
import { FileProgressStateData, ImportData, ImportManager, ImportManagerKey, ImportState, StateData } from '@/services/importManager';
import { IonIcon, IonList, IonText } from '@ionic/vue';
import { chevronDown, close } from 'ionicons/icons';
import { Ref, inject, onMounted, onUnmounted, ref } from 'vue';

interface ImportItem {
  data: ImportData;
  state: ImportState;
  progress: number;
}

const importManager: ImportManager = inject(ImportManagerKey)!;
const imports: Ref<Array<ImportItem>> = ref([]);
let dbId: string;
const menuMinimized = ref(false);
const isImportManagerActive = ref(false);
const shouldBeVisible = ref(false);
const uploadMenuList = ref();

function toggleMenu(): void {
  menuMinimized.value = !menuMinimized.value;
}

onMounted(async () => {
  dbId = await importManager.registerCallback(onImportEvent);
});

onUnmounted(async () => {
  await importManager.removeCallback(dbId);
});

function updateImportState(id: string, state: ImportState, progress?: number): void {
  const importItem = imports.value.find((importItem) => importItem.data.id === id);
  if (importItem) {
    importItem.state = state;
    if (progress !== undefined) {
      importItem.progress = progress;
    }
  }
}

async function cancelImport(importId: string): Promise<void> {
  await importManager.cancelImport(importId);
}

async function onImportEvent(state: ImportState, importData?: ImportData, stateData?: StateData): Promise<void> {
  switch (state) {
    case ImportState.ImportAllStarted:
      isImportManagerActive.value = true;
      shouldBeVisible.value = true;
      break;
    case ImportState.ImportAllFinished:
      isImportManagerActive.value = false;
      break;
    case ImportState.FileAdded:
      imports.value.unshift({
        data: importData as ImportData,
        state: state,
        progress: 0,
      });
      if (uploadMenuList.value?.$el) {
        uploadMenuList.value.$el.scrollTo({ top: 0, behavior: 'smooth' });
      }
      break;
    case ImportState.FileProgress:
      updateImportState((importData as ImportData).id, state, (stateData as FileProgressStateData).progress);
      break;
    case ImportState.FileImported:
      updateImportState((importData as ImportData).id, state, 100);
      break;
    case ImportState.CreateFailed:
      updateImportState((importData as ImportData).id, state, -1);
      break;
    case ImportState.Cancelled:
      updateImportState((importData as ImportData).id, state);
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
  .upload-menu-list {
    height: 0;
    padding: 0;
    margin: 0;
  }

  .menu-header-icons__item {
    transform: rotate(180deg);
  }
}
</style>
