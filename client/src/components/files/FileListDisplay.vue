<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div>
    <file-drop-zone
      ref="fileDropZoneRef"
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
              ref="folderItemsRef"
              :key="folder.id"
              :entry="folder"
              :show-checkbox="someSelected"
              @click="$emit('click', folder, $event)"
              @menu-click="(event, entry, onFinished) => $emit('menuClick', event, entry, onFinished)"
              @selected-change="onSelectedChange"
              @files-added="onFilesAdded"
              :is-workspace-reader="ownRole === WorkspaceRole.Reader"
              @drop-as-reader="$emit('dropAsReader')"
            />
            <file-list-item
              v-for="file in files.getEntries()"
              ref="fileItemsRef"
              :key="file.id"
              :entry="file"
              :show-checkbox="someSelected"
              @click="$emit('click', file, $event)"
              @menu-click="(event, entry, onFinished) => $emit('menuClick', event, entry, onFinished)"
              @selected-change="onSelectedChange"
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
  </div>
</template>

<script setup lang="ts">
import FileDropZone from '@/components/files/FileDropZone.vue';
import FileListItem from '@/components/files/FileListItem.vue';
import FileListItemProcessing from '@/components/files/FileListItemProcessing.vue';
import { EntryCollection, EntryModel, FileOperationProgress, FileModel, FolderModel } from '@/components/files/types';
import { FileImportTuple } from '@/components/files/utils';
import { FsPath, WorkspaceRole } from '@/parsec';
import { IonLabel, IonList, IonListHeader } from '@ionic/vue';
import { computed, ref } from 'vue';
import { MsCheckbox } from 'megashark-lib';

const props = defineProps<{
  operationsInProgress: Array<FileOperationProgress>;
  files: EntryCollection<FileModel>;
  folders: EntryCollection<FolderModel>;
  currentPath: FsPath;
  ownRole: WorkspaceRole;
}>();

const emits = defineEmits<{
  (e: 'click', entry: EntryModel, event: Event): void;
  (e: 'menuClick', event: Event, entry: EntryModel, onFinished: () => void): void;
  (e: 'globalMenuClick', event: Event): void;
  (e: 'filesAdded', imports: FileImportTuple[]): void;
  (e: 'dropAsReader'): void;
}>();

defineExpose({
  scrollToSelected,
});

const fileDropZoneRef = ref();
const folderItemsRef = ref<Array<typeof FileListItem>>();
const fileItemsRef = ref<Array<typeof FileListItem>>();
const containerScroll = ref();

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

async function onSelectedChange(_entry: EntryModel, _checked: boolean): Promise<void> {}

function onFilesAdded(imports: FileImportTuple[]): void {
  fileDropZoneRef.value.reset();
  emits('filesAdded', imports);
}

async function selectAll(selected: boolean): Promise<void> {
  props.files.selectAll(selected);
  props.folders.selectAll(selected);
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
      containerScroll.value.scrollTo({ top: offset - elHeight, behavior: 'smooth' });
    }
  }, 500);
}
</script>

<style scoped lang="scss">
.scroll {
  padding: 0;
  margin-bottom: 0;
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
