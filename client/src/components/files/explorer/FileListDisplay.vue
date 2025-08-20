<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-drop-zone
    ref="fileDropZone"
    :current-path="currentPath"
    @files-added="$emit('filesAdded', $event)"
    :show-drop-message="true"
    :is-reader="ownRole === WorkspaceRole.Reader"
    @drop-as-reader="$emit('dropAsReader')"
  >
    <div
      class="scroll"
      ref="containerScroll"
      @contextmenu="onContextMenu"
    >
      <ion-list
        class="list files-container-list"
        :class="{ 'file-list-mobile': isSmallDisplay }"
      >
        <ion-list-header
          class="folder-list-header"
          lines="full"
          v-if="isLargeDisplay"
        >
          <ion-label class="folder-list-header__label ion-text-nowrap label-selected">
            <ms-checkbox
              @change="selectAll"
              :checked="allSelected"
              :indeterminate="someSelected && !allSelected"
            />
          </ion-label>
          <ion-text
            class="folder-list-header__label cell-title ion-text-nowrap label-name"
            @click="onHeaderSortChange(SortProperty.Name)"
            :class="{ 'label-name-sorted': currentSortProperty === SortProperty.Name }"
          >
            {{ $msTranslate('FoldersPage.listDisplayTitles.name') }}
            <ion-icon
              :icon="currentSortOrder ? arrowUp : arrowDown"
              class="label-name__sort-icon"
            />
          </ion-text>
          <ion-text class="folder-list-header__label cell-title ion-text-nowrap label-updatedBy">
            {{ $msTranslate('FoldersPage.listDisplayTitles.updatedBy') }}
          </ion-text>
          <ion-text
            class="folder-list-header__label cell-title ion-text-nowrap label-lastUpdate"
            @click="onHeaderSortChange(SortProperty.LastUpdate)"
            :class="{ 'label-lastUpdate-sorted': currentSortProperty === SortProperty.LastUpdate }"
          >
            {{ $msTranslate('FoldersPage.listDisplayTitles.lastUpdate') }}
            <ion-icon
              :icon="currentSortOrder ? arrowUp : arrowDown"
              class="label-lastUpdate__sort-icon"
            />
          </ion-text>
          <ion-text
            class="folder-list-header__label cell-title ion-text-nowrap label-creationDate"
            @click="onHeaderSortChange(SortProperty.CreationDate)"
            :class="{ 'label-creationDate-sorted': currentSortProperty === SortProperty.CreationDate }"
          >
            {{ $msTranslate('FoldersPage.listDisplayTitles.creation') }}
            <ion-icon
              :icon="currentSortOrder ? arrowUp : arrowDown"
              class="label-creationDate__sort-icon"
            />
          </ion-text>
          <ion-text
            class="folder-list-header__label cell-title ion-text-nowrap label-size"
            @click="onHeaderSortChange(SortProperty.Size)"
            :class="{ 'label-size-sorted': currentSortProperty === SortProperty.Size }"
          >
            {{ $msTranslate('FoldersPage.listDisplayTitles.size') }}
            <ion-icon
              :icon="currentSortOrder ? arrowUp : arrowDown"
              class="label-size__sort-icon"
            />
          </ion-text>
          <ion-text class="folder-list-header__label cell-title ion-text-nowrap label-space" />
        </ion-list-header>
        <div>
          <file-list-item
            v-for="folder in folders.getEntries()"
            ref="folderItems"
            v-model="folder.isSelected"
            :key="folder.id"
            :entry="folder"
            :show-checkbox="someSelected || selectionEnabled === true"
            :is-workspace-reader="ownRole === WorkspaceRole.Reader"
            @open-item="$emit('openItem', folder, $event)"
            @open-item.stop
            @menu-click="onMenuClick"
            @files-added="onFilesAdded"
            @drop-as-reader="$emit('dropAsReader')"
          />
          <file-list-item
            v-for="file in files.getEntries()"
            ref="fileItems"
            v-model="file.isSelected"
            :key="file.id"
            :entry="file"
            :show-checkbox="someSelected || selectionEnabled === true"
            @open-item="$emit('openItem', file, $event)"
            @open-item.stop
            @menu-click="onMenuClick"
            @files-added="onFilesAdded"
            @drop-as-reader="$emit('dropAsReader')"
          />
          <file-list-item-processing
            v-for="op in operationsInProgress"
            :key="op.data.id"
            :data="op.data"
            :progress="op.progress"
          />
        </div>
      </ion-list>
    </div>
  </file-drop-zone>
</template>

<script setup lang="ts">
import FileDropZone from '@/components/files/explorer/FileDropZone.vue';
import FileListItem from '@/components/files/explorer/FileListItem.vue';
import FileListItemProcessing from '@/components/files/explorer/FileListItemProcessing.vue';
import { EntryCollection, EntryModel, FileOperationProgress, FileModel, FolderModel } from '@/components/files/types';
import { SortProperty } from '@/components/files';
import { FileImportTuple } from '@/components/files/utils';
import { FsPath, WorkspaceRole } from '@/parsec';
import { IonText, IonList, IonLabel, IonListHeader, IonIcon } from '@ionic/vue';
import { arrowUp, arrowDown } from 'ionicons/icons';
import { computed, useTemplateRef } from 'vue';
import { MsCheckbox, useWindowSize, MsSorterChangeEvent } from 'megashark-lib';

const { isLargeDisplay, isSmallDisplay } = useWindowSize();
const props = defineProps<{
  operationsInProgress: Array<FileOperationProgress>;
  files: EntryCollection<FileModel>;
  folders: EntryCollection<FolderModel>;
  currentPath: FsPath;
  ownRole: WorkspaceRole;
  selectionEnabled?: boolean;
  currentSortProperty: SortProperty;
  currentSortOrder?: boolean;
}>();

const emits = defineEmits<{
  (e: 'openItem', entry: EntryModel, event: Event): void;
  (e: 'sortChange', event: MsSorterChangeEvent): void;
  (e: 'menuClick', event: Event, entry: EntryModel, onFinished: () => void): void;
  (e: 'globalMenuClick', event: Event): void;
  (e: 'filesAdded', imports: FileImportTuple[]): void;
  (e: 'dropAsReader'): void;
}>();

defineExpose({
  scrollToSelected,
});

const fileDropZoneRef = useTemplateRef('fileDropZone');
const folderItemsRef = useTemplateRef<Array<typeof FileListItem>>('folderItems');
const fileItemsRef = useTemplateRef<Array<typeof FileListItem>>('fileItems');
const containerScrollRef = useTemplateRef('containerScroll');

const allSelected = computed(() => {
  const selectedCount = props.files.selectedCount() + props.folders.selectedCount();
  return selectedCount > 0 && selectedCount === props.files.entriesCount() + props.folders.entriesCount();
});

const someSelected = computed(() => {
  return props.files.selectedCount() + props.folders.selectedCount() > 0;
});

async function onContextMenu(event: Event): Promise<void> {
  event.preventDefault();
  emits('globalMenuClick', event);
}

async function onMenuClick(event: Event, entry: EntryModel, onFinished: () => void): Promise<void> {
  emits('menuClick', event, entry, onFinished);
}

function onFilesAdded(imports: FileImportTuple[]): void {
  fileDropZoneRef.value?.reset();
  emits('filesAdded', imports);
}

async function selectAll(selected: boolean): Promise<void> {
  props.files.selectAll(selected);
  props.folders.selectAll(selected);
}

async function onHeaderSortChange(property: SortProperty): Promise<void> {
  const newSortOrder = props.currentSortProperty === property ? !props.currentSortOrder : props.currentSortOrder;

  const sortEvent: MsSorterChangeEvent = {
    option: { key: property, label: '' },
    sortByAsc: newSortOrder,
  };

  emits('sortChange', sortEvent);
}

async function scrollToSelected(): Promise<void> {
  let selectedItem: any = undefined;

  for (const item of folderItemsRef.value ?? []) {
    if (item.props.entry.isSelected) {
      selectedItem = item;
      break;
    }
  }
  if (!selectedItem) {
    for (const item of fileItemsRef.value ?? []) {
      if (item.props.entry.isSelected) {
        selectedItem = item;
        break;
      }
    }
  }
  // Don't know how to handle it better.
  // If the component has been created recently, `selectedItem.offsetTop` doesn't have the
  // correct value yet.
  setTimeout(() => {
    if (selectedItem) {
      const offset = selectedItem.$el.offsetTop;
      const elHeight = selectedItem.$el.offsetHeight;
      containerScrollRef.value?.scrollTo({ top: offset - elHeight, behavior: 'smooth' });
    }
  }, 500);
}
</script>

<style scoped lang="scss">
.scroll {
  padding: 0;
  margin-bottom: 0;
}

.file-list-mobile {
  padding-top: 1rem;
}

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
