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
    <ion-list class="upload-menu-tabs">
      <ion-item
        class="upload-menu-tabs__item body"
        :class="{ active: currentTab === Tabs.InProgress }"
        @click="currentTab = Tabs.InProgress"
      >
        <ion-label>{{ $t('FoldersPage.ImportFile.tabs.inProgress') }}</ion-label>
      </ion-item>

      <ion-item
        class="upload-menu-tabs__item body"
        :class="{ active: currentTab === Tabs.Done }"
        @click="currentTab = Tabs.Done"
      >
        <ion-label>{{ $t('FoldersPage.ImportFile.tabs.done') }}</ion-label>
      </ion-item>

      <ion-item
        class="upload-menu-tabs__item body"
        :class="{ active: currentTab === Tabs.Error }"
        @click="currentTab = Tabs.Error"
      >
        <ion-label>{{ $t('FoldersPage.ImportFile.tabs.failed') }}</ion-label>
      </ion-item>
    </ion-list>

    <ion-list class="upload-menu-list">
      <template v-if="currentTab === Tabs.InProgress">
        <file-upload-item
          v-for="item in importingItems"
          :key="item.data.id"
          :progress="item.progress"
          :state="item.state"
          :import-data="item.data"
          @import-cancel="cancelImport($event)"
        />
        <div
          class="upload-menu-list__empty"
          v-if="importingItems.length === 0"
        >
          <ms-image :image="NoImportInProgress" />
          <ion-text class="body-lg">
            {{ $t('FoldersPage.ImportFile.noImportInProgress') }}
          </ion-text>
        </div>
      </template>
      <template v-if="currentTab === Tabs.Done">
        <file-upload-item
          v-for="item in doneItems"
          :key="item.data.id"
          :progress="item.progress"
          :state="item.state"
          :import-data="item.data"
          @import-cancel="cancelImport($event)"
        />
        <div
          class="upload-menu-list__empty"
          v-if="doneItems.length === 0"
        >
          <ms-image :image="NoImportDone" />
          <ion-text class="body-lg">
            {{ $t('FoldersPage.ImportFile.noImportDone') }}
          </ion-text>
        </div>
      </template>
      <template v-if="currentTab === Tabs.Error">
        <file-upload-item
          v-for="item in errorItems"
          :key="item.data.id"
          :progress="item.progress"
          :state="item.state"
          :import-data="item.data"
          @import-cancel="cancelImport($event)"
        />
        <div
          class="upload-menu-list__empty"
          v-if="errorItems.length === 0"
        >
          <ms-image :image="NoImportError" />
          <ion-text class="body-lg">
            {{ $t('FoldersPage.ImportFile.noImportFailed') }}
          </ion-text>
        </div>
      </template>
    </ion-list>
  </div>
</template>

<script setup lang="ts">
import { MsImage, NoImportDone, NoImportError, NoImportInProgress } from '@/components/core/ms-image';
import FileUploadItem from '@/components/files/FileUploadItem.vue';
import { FileProgressStateData, ImportData, ImportManager, ImportManagerKey, ImportState, StateData } from '@/services/importManager';
import { IonIcon, IonItem, IonLabel, IonList, IonText } from '@ionic/vue';
import { chevronDown, close } from 'ionicons/icons';
import { Ref, computed, inject, onMounted, onUnmounted, ref } from 'vue';

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

enum Tabs {
  InProgress,
  Done,
  Error,
}

const currentTab = ref(Tabs.InProgress);

const importingItems = computed(() => {
  return imports.value.filter((importItem) => [ImportState.FileProgress, ImportState.FileAdded].includes(importItem.state));
});

const doneItems = computed(() => {
  return imports.value.filter((importItem) => [ImportState.FileImported, ImportState.Cancelled].includes(importItem.state));
});

const errorItems = computed(() => {
  return imports.value.filter((importItem) => importItem.state === ImportState.CreateFailed);
});

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

      ion-label {
        margin: 0 auto;
        text-align: center;
      }

      &:hover {
        color: var(--parsec-color-light-primary-500);
      }

      &.active {
        box-shadow: var(--parsec-shadow-light);
        --background: var(--parsec-color-light-secondary-white);
        color: var(--parsec-color-light-primary-500);
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
