<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="folders-container-grid">
    <file-card
      class="folder-grid-item"
      v-for="folder in folders.getEntries()"
      :key="folder.id"
      :entry="folder"
      :show-checkbox="hasSelected()"
      @click="$emit('click', folder, $event)"
      @menu-click="(event, entry, onFinished) => $emit('menuClick', event, entry, onFinished)"
    />
    <file-card
      class="folder-grid-item"
      v-for="file in files.getEntries()"
      :key="file.id"
      :entry="file"
      :show-checkbox="hasSelected()"
      @click="$emit('click', file, $event)"
      @menu-click="(event, entry, onFinished) => $emit('menuClick', event, entry, onFinished)"
    />

    <file-card-importing
      v-for="fileImport in importing"
      :key="fileImport.data.id"
      :data="fileImport.data"
      :progress="fileImport.progress"
    />
  </div>
</template>

<script setup lang="ts">
import FileCard from '@/components/files/FileCard.vue';
import FileCardImporting from '@/components/files/FileCardImporting.vue';
import { EntryCollection, EntryModel, FileImportProgress, FileModel, FolderModel } from '@/components/files/types';
import { Groups, HotkeyManager, HotkeyManagerKey, Hotkeys, Modifiers, Platforms } from '@/services/hotkeyManager';
import { inject, onMounted, onUnmounted } from 'vue';

const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;

let hotkeys: Hotkeys | null = null;

onMounted(async () => {
  hotkeys = hotkeyManager.newHotkeys(Groups.Workspaces);
  hotkeys.add('a', Modifiers.Ctrl, Platforms.Desktop | Platforms.Web, async () => {
    props.files.selectAll(true);
    props.folders.selectAll(true);
  });
});

onUnmounted(async () => {
  if (hotkeys) {
    hotkeyManager.unregister(hotkeys);
  }
});

const props = defineProps<{
  importing: Array<FileImportProgress>;
  files: EntryCollection<FileModel>;
  folders: EntryCollection<FolderModel>;
}>();

defineEmits<{
  (e: 'click', entry: EntryModel, event: Event): void;
  (e: 'menuClick', event: Event, entry: EntryModel, onFinished: () => void): void;
}>();

function hasSelected(): boolean {
  return props.files.hasSelected() || props.folders.hasSelected();
}
</script>

<style scoped lang="scss">
.folders-container-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 1.5em;
  overflow-y: auto;
}
</style>
