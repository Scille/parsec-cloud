<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div>
    <ion-list class="list">
      <ion-list-header
        class="folder-list-header"
        lines="full"
      >
        <ion-label class="folder-list-header__label ion-text-nowrap label-selected">
          <ion-checkbox
            aria-label=""
            class="checkbox"
            @ion-change="selectAll($event.detail.checked)"
            :checked="allSelected"
            :indeterminate="someSelected && !allSelected"
          />
        </ion-label>
        <ion-label class="folder-list-header__label cell-title ion-text-nowrap label-name">
          {{ $t('FoldersPage.listDisplayTitles.name') }}
        </ion-label>
        <ion-label class="folder-list-header__label cell-title ion-text-nowrap label-updatedBy">
          {{ $t('FoldersPage.listDisplayTitles.updatedBy') }}
        </ion-label>
        <ion-label class="folder-list-header__label cell-title ion-text-nowrap label-lastUpdate">
          {{ $t('FoldersPage.listDisplayTitles.lastUpdate') }}
        </ion-label>
        <ion-label class="folder-list-header__label cell-title ion-text-nowrap label-size">
          {{ $t('FoldersPage.listDisplayTitles.size') }}
        </ion-label>
        <ion-label class="folder-list-header__label cell-title ion-text-nowrap label-space" />
      </ion-list-header>
      <file-list-item
        v-for="folder in folders.getEntries()"
        :key="folder.id"
        :entry="folder"
        :show-checkbox="someSelected"
        @click="$emit('click', folder, $event)"
        @menu-click="(event, entry, onFinished) => $emit('menuClick', event, entry, onFinished)"
        @selected-change="onSelectedChange"
      />
      <file-list-item
        v-for="file in files.getEntries()"
        :key="file.id"
        :entry="file"
        :show-checkbox="someSelected"
        @click="$emit('click', file, $event)"
        @menu-click="(event, entry, onFinished) => $emit('menuClick', event, entry, onFinished)"
        @selected-change="onSelectedChange"
      />
      <file-list-item-importing
        v-for="fileImport in importing"
        :key="fileImport.data.id"
        :data="fileImport.data"
        :progress="fileImport.progress"
      />
    </ion-list>
  </div>
</template>

<script setup lang="ts">
import FileListItem from '@/components/files/FileListItem.vue';
import FileListItemImporting from '@/components/files/FileListItemImporting.vue';
import { EntryCollection, EntryModel, FileImportProgress, FileModel, FolderModel } from '@/components/files/types';
import { Groups, HotkeyManager, HotkeyManagerKey, Hotkeys, Modifiers, Platforms } from '@/services/hotkeyManager';
import { IonCheckbox, IonLabel, IonList, IonListHeader } from '@ionic/vue';
import { computed, inject, onMounted, onUnmounted } from 'vue';

const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;

let hotkeys: Hotkeys | null = null;

const props = defineProps<{
  importing: Array<FileImportProgress>;
  files: EntryCollection<FileModel>;
  folders: EntryCollection<FolderModel>;
}>();

defineEmits<{
  (e: 'click', entry: EntryModel, event: Event): void;
  (e: 'menuClick', event: Event, entry: EntryModel, onFinished: () => void): void;
}>();

onMounted(async () => {
  hotkeys = hotkeyManager.newHotkeys(Groups.Workspaces);
  hotkeys.add('a', Modifiers.Ctrl, Platforms.Desktop | Platforms.Web, async () => await selectAll(true));
});

onUnmounted(async () => {
  if (hotkeys) {
    hotkeyManager.unregister(hotkeys);
  }
});

const allSelected = computed(() => {
  const selectedCount = props.files.selectedCount() + props.folders.selectedCount();
  return selectedCount > 0 && selectedCount === props.files.entriesCount() + props.folders.entriesCount();
});

const someSelected = computed(() => {
  return props.files.selectedCount() + props.folders.selectedCount() > 0;
});

async function onSelectedChange(_entry: EntryModel, _checked: boolean): Promise<void> {}

async function selectAll(selected: boolean): Promise<void> {
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
