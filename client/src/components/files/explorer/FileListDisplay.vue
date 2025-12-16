<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-drop-zone
    ref="fileDropZone"
    @files-added="$emit('filesAdded', $event)"
    :show-drop-message="true"
    :is-reader="ownRole === WorkspaceRole.Reader"
    @drop-as-reader="$emit('dropAsReader')"
    class="file-drop-zone"
  >
    <div
      class="container-scroll"
      ref="containerScroll"
      @contextmenu="onContextMenu"
    >
      <ion-list
        class="files-container-list"
        :class="{ 'file-list-mobile': isSmallDisplay }"
      >
        <ion-list-header
          class="folder-list-header"
          lines="full"
          v-if="isLargeDisplay"
        >
          <ion-label class="folder-list-header__label ion-text-nowrap header-label-selected">
            <ms-checkbox
              @change="selectAll"
              :checked="allSelected"
              :indeterminate="someSelected && !allSelected"
            />
          </ion-label>
          <ion-text
            class="folder-list-header__label cell-title ion-text-nowrap header-label-name"
            @click="onHeaderSortChange(SortProperty.Name)"
            :class="{ 'header-label-name-sorted': currentSortProperty === SortProperty.Name }"
          >
            <span class="header-label-name__text">{{ $msTranslate('FoldersPage.listDisplayTitles.name') }}</span>
            <ion-icon
              :icon="currentSortOrder ? arrowUp : arrowDown"
              class="header-label-name__sort-icon"
            />
          </ion-text>
          <ion-text
            class="folder-list-header__label cell-title ion-text-nowrap header-label-updated-by"
            v-if="ownProfile !== UserProfile.Outsider"
          >
            {{ $msTranslate('FoldersPage.listDisplayTitles.updatedBy') }}
          </ion-text>
          <ion-text
            class="folder-list-header__label cell-title ion-text-nowrap header-label-last-update"
            @click="onHeaderSortChange(SortProperty.LastUpdate)"
            :class="{ 'header-label-last-update-sorted': currentSortProperty === SortProperty.LastUpdate }"
          >
            <span class="header-label-last-update__text">{{ $msTranslate('FoldersPage.listDisplayTitles.lastUpdate') }}</span>
            <ion-icon
              :icon="currentSortOrder ? arrowUp : arrowDown"
              class="header-label-last-update__sort-icon"
            />
          </ion-text>
          <ion-text
            class="folder-list-header__label cell-title ion-text-nowrap header-label-creation-date"
            @click="onHeaderSortChange(SortProperty.CreationDate)"
            :class="{ 'header-label-creation-date-sorted': currentSortProperty === SortProperty.CreationDate }"
          >
            <span class="header-label-creation-date__text">{{ $msTranslate('FoldersPage.listDisplayTitles.creation') }}</span>
            <ion-icon
              :icon="currentSortOrder ? arrowUp : arrowDown"
              class="header-label-creation-date__sort-icon"
            />
          </ion-text>
          <ion-text
            class="folder-list-header__label cell-title ion-text-nowrap header-label-size"
            @click="onHeaderSortChange(SortProperty.Size)"
            :class="{ 'header-label-size-sorted': currentSortProperty === SortProperty.Size }"
          >
            <span class="header-label-size__text">{{ $msTranslate('FoldersPage.listDisplayTitles.size') }}</span>
            <ion-icon
              :icon="currentSortOrder ? arrowUp : arrowDown"
              class="header-label-size__sort-icon"
            />
          </ion-text>
          <ion-text class="folder-list-header__label cell-title ion-text-nowrap header-label-space" />
        </ion-list-header>
        <div>
          <file-list-item
            v-for="folder in folders.getEntries()"
            ref="folderItems"
            v-model="folder.isSelected"
            :key="folder.id"
            :entry="folder"
            :own-profile="ownProfile"
            :show-checkbox="someSelected || selectionEnabled === true"
            :is-workspace-reader="ownRole === WorkspaceRole.Reader"
            @open-item.stop="$emit('openItem', folder, $event)"
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
            :own-profile="ownProfile"
            :show-checkbox="someSelected || selectionEnabled === true"
            @open-item.stop="$emit('openItem', file, $event)"
            @menu-click="onMenuClick"
            @files-added="onFilesAdded"
            @drop-as-reader="$emit('dropAsReader')"
          />
          <file-list-item-processing
            v-for="op in fileOperations"
            :key="op.entryName"
            :operation="op"
            :profile="ownProfile"
          />
        </div>
      </ion-list>
    </div>
  </file-drop-zone>
</template>

<script setup lang="ts">
import { SortProperty } from '@/components/files';
import FileDropZone from '@/components/files/explorer/FileDropZone.vue';
import FileListItem from '@/components/files/explorer/FileListItem.vue';
import FileListItemProcessing from '@/components/files/explorer/FileListItemProcessing.vue';
import { EntryCollection, EntryModel, FileModel, FileOperationCurrentFolder, FolderModel } from '@/components/files/types';
import { EntryName, UserProfile, WorkspaceRole } from '@/parsec';
import { IonIcon, IonLabel, IonList, IonListHeader, IonText } from '@ionic/vue';
import { arrowDown, arrowUp } from 'ionicons/icons';
import { MsCheckbox, MsSorterChangeEvent, useWindowSize } from 'megashark-lib';
import { computed, useTemplateRef } from 'vue';

const { isLargeDisplay, isSmallDisplay } = useWindowSize();
const props = defineProps<{
  ownProfile: UserProfile;
  files: EntryCollection<FileModel>;
  folders: EntryCollection<FolderModel>;
  ownRole: WorkspaceRole;
  fileOperations: Array<FileOperationCurrentFolder>;
  selectionEnabled?: boolean;
  currentSortProperty: SortProperty;
  currentSortOrder?: boolean;
}>();

const emits = defineEmits<{
  (e: 'openItem', entry: EntryModel, event: Event): void;
  (e: 'sortChange', event: MsSorterChangeEvent): void;
  (e: 'menuClick', event: Event, entry: EntryModel, onFinished: () => void): void;
  (e: 'globalMenuClick', event: Event): void;
  (e: 'filesAdded', files: Array<File>, destinationFolder?: EntryName): void;
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

function onFilesAdded(files: Array<File>, destinationFolder?: EntryName): void {
  fileDropZoneRef.value?.reset();
  emits('filesAdded', files, destinationFolder);
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
.file-drop-zone {
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.container-scroll {
  flex-grow: 1;
  overflow: auto;
}

.header-label-selected {
  justify-content: flex-end;
}

.file-list-mobile {
  padding-top: 1rem;
}

.folder-list-header__label {
  padding: 0.75rem 1rem;
}
</style>
