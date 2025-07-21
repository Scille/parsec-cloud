<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-handler @file-loaded="onFileLoaded">
    <div
      class="file-editor"
      id="file-editor"
      ref="fileEditor"
    />
  </file-handler>
</template>

<script setup lang="ts">
import { closeFile, entryStat, EntryStatFile, FsPath, openFile, readFile, DEFAULT_READ_SIZE, WorkspaceHandle, writeFile } from '@/parsec';
import { I18n } from 'megashark-lib';
import { ref, Ref, inject, useTemplateRef } from 'vue';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { getDocumentPath, getWorkspaceHandle } from '@/router';
import { DetectedFileType } from '@/common/fileTypes';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import { Env } from '@/services/environment';
import { CryptpadDocumentType, Cryptpad, getDocumentTypeFromExtension } from '@/services/cryptpad';
import { longLocaleCodeToShort } from '@/services/translation';
import FileHandler from '@/views/files/handler/FileHandler.vue';

const informationManager: InformationManager = inject(InformationManagerKey)!;
const contentInfoRef: Ref<FileContentInfo | undefined> = ref(undefined);
const fileEditorRef = useTemplateRef('fileEditor');
const documentType = ref<CryptpadDocumentType | null>(null);
const cryptpadInstance = ref<Cryptpad | null>(null);
const fileInfoIdRef = ref<string | undefined>(undefined);

const emits = defineEmits<{
  (e: 'handlerReady'): void;
}>();

async function onFileLoaded(contentInfo: FileContentInfo, fileInfo: DetectedFileType, fileInfoId: string): Promise<void> {
  contentInfoRef.value = contentInfo;
  fileInfoIdRef.value = fileInfoId;
  documentType.value = getDocumentTypeFromExtension(fileInfo.extension);

  if (!documentType.value) {
    window.electronAPI.log('error', `No document type for file with extension '${fileInfo.extension}'`);
    informationManager.present(
      new Information({
        message: 'fileViewers.genericError',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
  await loadCryptpad();
  emits('handlerReady');
}

async function getUrlFromFile(workspaceHandle: WorkspaceHandle, path: FsPath): Promise<string | undefined> {
  const statsResult = await entryStat(workspaceHandle, path);
  if (!statsResult.ok || !statsResult.value.isFile()) {
    return;
  }

  const openResult = await openFile(workspaceHandle, path, { read: true });

  if (!openResult.ok) {
    return undefined;
  }
  const fd = openResult.value;
  const content = new Uint8Array((statsResult.value as EntryStatFile).size);
  try {
    let loop = true;
    let offset = 0;
    while (loop) {
      const readResult = await readFile(workspaceHandle, openResult.value, offset, DEFAULT_READ_SIZE);
      if (!readResult.ok) {
        throw Error(JSON.stringify(readResult.error));
      }
      const buffer = new Uint8Array(readResult.value);
      content.set(buffer, offset);
      if (readResult.value.byteLength < DEFAULT_READ_SIZE) {
        loop = false;
      }
      offset += readResult.value.byteLength;
    }
    const urlObject = URL.createObjectURL(new Blob([content], { type: 'application/octet-stream' }));
    return urlObject;
  } catch (e: any) {
    window.electronAPI.log('error', `Can't read the file: ${e.toString()}`);
  } finally {
    await closeFile(workspaceHandle, fd);
  }
}

async function loadCryptpad(): Promise<void> {
  // Cryptpad edition specific handling
  const fileUrl = await getUrlFromFile(getWorkspaceHandle()!, getDocumentPath());
  console.log('Cryptpad file URL:', fileUrl);
  if (!fileUrl) {
    window.electronAPI.log('error', 'Failed to retrieve file URL for Cryptpad edition');
    return;
  }
  const cryptpadConfig = {
    document: {
      url: fileUrl,
      fileType: contentInfoRef.value?.extension || '',
      title: contentInfoRef.value?.fileName || '',
      key: fileInfoIdRef.value,
    },
    documentType: documentType.value!,
    editorConfig: {
      lang: longLocaleCodeToShort(I18n.getLocale()),
    },
    autosave: 10,
    events: {
      // eslint-disable-next-line @typescript-eslint/explicit-function-return-type
      onSave: async (file: Blob, callback: () => void) => {
        console.log('Cryptpad onSave event triggered');
        // Handle save logic here
        const workspaceHandle = getWorkspaceHandle();
        const openResult = await openFile(workspaceHandle!, getDocumentPath(), { write: true, truncate: true });

        if (!openResult.ok) {
          return undefined;
        }
        const fd = openResult.value;
        const arrayBuffer = await file.arrayBuffer();
        await writeFile(workspaceHandle!, fd, 0, new Uint8Array(arrayBuffer));
        await closeFile(workspaceHandle!, fd);
        callback();
      },
      onHasUnsavedChanges: (unsaved: boolean): void => {
        console.log('Cryptpad unsaved changes:', unsaved);
      },
    },
  };
  if (Env.isEditicsEnabled()) {
    if (!cryptpadInstance.value) {
      cryptpadInstance.value = new Cryptpad(fileEditorRef.value as HTMLDivElement, Env.getDefaultCryptpadServer());
      console.log(cryptpadInstance.value);
    }

    await cryptpadInstance.value.open(cryptpadConfig);
  }
}
</script>

<style scoped lang="scss"></style>
