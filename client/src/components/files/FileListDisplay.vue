<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div>
    <file-drop-zone
      ref="fileDropZoneRef"
      :current-path="currentPath"
      @files-added="$emit('filesAdded', $event)"
      :show-drop-message="true"
    >
      <ion-list class="list">
        <ion-list-header
          class="folder-list-header"
          lines="full"
        >
          <ion-label class="folder-list-header__label ion-text-nowrap label-selected">
            <ms-checkbox
              @change="selectAll"
              :checked="allSelected"
              :indeterminate="someSelected && !allSelected"
            />
          </ion-label>
          <ion-label class="folder-list-header__label cell-title ion-text-nowrap label-name">
            {{ $msTranslate('FoldersPage.listDisplayTitles.name') }}
          </ion-label>
          <ion-label
            class="folder-list-header__label cell-title ion-text-nowrap label-updatedBy"
            v-show="false"
          >
            {{ $msTranslate('FoldersPage.listDisplayTitles.updatedBy') }}
          </ion-label>
          <ion-label class="folder-list-header__label cell-title ion-text-nowrap label-lastUpdate">
            {{ $msTranslate('FoldersPage.listDisplayTitles.lastUpdate') }}
          </ion-label>
          <ion-label class="folder-list-header__label cell-title ion-text-nowrap label-size">
            {{ $msTranslate('FoldersPage.listDisplayTitles.size') }}
          </ion-label>
          <ion-label class="folder-list-header__label cell-title ion-text-nowrap label-space" />
        </ion-list-header>
        <div>
          <file-list-item
            v-for="folder in folders.getEntries()"
            :key="folder.id"
            :entry="folder"
            :show-checkbox="someSelected"
            @click="$emit('click', folder, $event)"
            @menu-click="(event, entry, onFinished) => $emit('menuClick', event, entry, onFinished)"
            @selected-change="onSelectedChange"
            @files-added="onFilesAdded"
          />
          <file-list-item
            v-for="file in files.getEntries()"
            :key="file.id"
            :entry="file"
            :show-checkbox="someSelected"
            @click="$emit('click', file, $event)"
            @menu-click="(event, entry, onFinished) => $emit('menuClick', event, entry, onFinished)"
            @selected-change="onSelectedChange"
            @files-added="onFilesAdded"
          />
          <file-list-item-importing
            v-for="fileImport in importing"
            :key="fileImport.data.id"
            :data="fileImport.data"
            :progress="fileImport.progress"
          />
        </div>
      </ion-list>
    </file-drop-zone>
  </div>
</template>

<script setup lang="ts">
import FileDropZone from '@/components/files/FileDropZone.vue';
import FileListItem from '@/components/files/FileListItem.vue';
import FileListItemImporting from '@/components/files/FileListItemImporting.vue';
import { EntryCollection, EntryModel, FileImportProgress, FileModel, FolderModel } from '@/components/files/types';
import { FileImportTuple } from '@/components/files/utils';
import { FsPath } from '@/parsec';
import { IonLabel, IonList, IonListHeader } from '@ionic/vue';
import { computed, ref } from 'vue';
import { MsCheckbox } from 'megashark-lib';

const props = defineProps<{
  importing: Array<FileImportProgress>;
  files: EntryCollection<FileModel>;
  folders: EntryCollection<FolderModel>;
  currentPath: FsPath;
}>();

const emits = defineEmits<{
  (e: 'click', entry: EntryModel, event: Event): void;
  (e: 'menuClick', event: Event, entry: EntryModel, onFinished: () => void): void;
  (e: 'filesAdded', imports: FileImportTuple[]): void;
}>();

const fileDropZoneRef = ref();

const allSelected = computed(() => {
  const selectedCount = props.files.selectedCount() + props.folders.selectedCount();
  return selectedCount > 0 && selectedCount === props.files.entriesCount() + props.folders.entriesCount();
});

const someSelected = computed(() => {
  return props.files.selectedCount() + props.folders.selectedCount() > 0;
});

async function onSelectedChange(_entry: EntryModel, _checked: boolean): Promise<void> {}

function onFilesAdded(imports: FileImportTuple[]): void {
  fileDropZoneRef.value.reset();
  emits('filesAdded', imports);
}

async function selectAll(selected: boolean): Promise<void> {
  console.log('Selected', selected);
  props.files.selectAll(selected);
  props.folders.selectAll(selected);
}
</script>

<style scoped lang="scss">
.folder-list-header {
  &__label {
    padding: 0.75rem 1rem;
  }
  .label-selected {
    display: flex;
    align-items: center;
  }

  .label-space {
    min-width: 4rem;
    flex-grow: 0;
    margin-left: auto;
  }
}
</style>
