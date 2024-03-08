<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="$t('FoldersPage.importModal.title')"
    :close-button="{ visible: true }"
  >
    <div class="modal-content inner-content">
      <file-drop-zone @files-drop="onFilesDrop">
        <file-import @files-import="onFilesImport" />
      </file-drop-zone>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import { Answer, askQuestion, MsModal } from '@/components/core';
import { FileDropZone, FileImport } from '@/components/files';
import { entryStat, Path, WorkspaceHandle, WorkspaceID } from '@/parsec';
import { ImportManager, ImportManagerKey } from '@/services/importManager';
import { translate } from '@/services/translation';
import { inject } from 'vue';

const importManager = inject(ImportManagerKey) as ImportManager;

const props = defineProps<{
  currentPath: string;
  workspaceHandle: WorkspaceHandle;
  workspaceId: WorkspaceID;
}>();

interface FileImportTuple {
  file: File;
  path: string;
}

async function importFiles(imports: FileImportTuple[]): Promise<void> {
  const existing: FileImportTuple[] = [];

  for (const imp of imports) {
    const fullPath = await Path.join(imp.path, imp.file.name);
    const result = await entryStat(props.workspaceHandle, fullPath);
    if (result.ok && result.value.isFile()) {
      existing.push(imp);
    }
  }

  if (existing.length > 0) {
    const answer = await askQuestion(
      translate('FoldersPage.importModal.replaceTitle', {}, 1),
      translate('FoldersPage.importModal.replaceQuestion', { file: existing[0].file.name, count: existing.length }),
      {
        yesText: translate('FoldersPage.importModal.replaceText', {}, 1),
        noText: translate('FoldersPage.importModal.skipText', {}, 1),
      },
    );
    if (answer === Answer.No) {
      imports = imports.filter((imp) => {
        return (
          existing.find((ex) => {
            return ex.file.name === imp.file.name && ex.path === imp.path;
          }) === undefined
        );
      });
    }
  }
  for (const imp of imports) {
    await importManager.importFile(props.workspaceHandle, props.workspaceId, imp.file, imp.path);
  }
}

async function convertEntryToFile(fsEntry: FileSystemEntry): Promise<File> {
  return new Promise((resolve, reject) => {
    (fsEntry as FileSystemFileEntry).file(
      (file) => {
        resolve(file);
      },
      () => {
        reject();
      },
    );
  });
}

async function getEntries(fsEntry: FileSystemDirectoryEntry): Promise<FileSystemEntry[]> {
  return new Promise((resolve, reject) => {
    const reader = fsEntry.createReader();
    reader.readEntries(
      (entries) => {
        resolve(entries);
      },
      () => {
        reject();
      },
    );
  });
}

async function unwindEntry(currentPath: string, fsEntry: FileSystemEntry): Promise<FileImportTuple[]> {
  const parsecPath = await Path.join(currentPath, fsEntry.name);
  const imports: FileImportTuple[] = [];

  if (fsEntry.isDirectory) {
    const entries = await getEntries(fsEntry as FileSystemDirectoryEntry);
    for (const entry of entries) {
      const result = await unwindEntry(parsecPath, entry);
      imports.push(...result);
    }
  } else {
    const result = await convertEntryToFile(fsEntry);
    imports.push({ file: result, path: currentPath });
  }
  return imports;
}

async function onFilesDrop(entries: FileSystemEntry[]): Promise<void> {
  const imports: FileImportTuple[] = [];
  for (const entry of entries) {
    const result = await unwindEntry(props.currentPath, entry);
    imports.push(...result);
  }
  await importFiles(imports);
}

async function onFilesImport(files: File[]): Promise<void> {
  const imports: FileImportTuple[] = files.map((file): FileImportTuple => {
    return { file: file, path: props.currentPath };
  });
  await importFiles(imports);
}
</script>

<style scoped lang="scss"></style>
