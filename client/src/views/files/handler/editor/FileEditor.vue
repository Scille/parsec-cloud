<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="file-editor"
    id="file-editor"
    ref="fileEditor"
  />
</template>

<script setup lang="ts">
import { closeFile, FsPath, openFile, writeFile } from '@/parsec';
import { I18n } from 'megashark-lib';
import { ref, Ref, inject, useTemplateRef, onMounted, onUnmounted } from 'vue';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { getDocumentPath, getWorkspaceHandle } from '@/router';
import { DetectedFileType } from '@/common/fileTypes';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import { Env } from '@/services/environment';
import { CryptpadDocumentType, Cryptpad, getDocumentTypeFromExtension } from '@/services/cryptpad';
import { longLocaleCodeToShort } from '@/services/translation';

const informationManager: InformationManager = inject(InformationManagerKey)!;
const contentInfoRef: Ref<FileContentInfo | undefined> = ref(undefined);
const fileEditorRef = useTemplateRef('fileEditor');
const documentType = ref<CryptpadDocumentType | null>(null);
const workspaceHandleRef = ref<number | undefined>(undefined);
const documentPathRef = ref<FsPath>();
const cryptpadInstance = ref<Cryptpad | null>(null);
const fileUrl = ref<string | null>(null);

const { contentInfo, fileInfo } = defineProps<{
  contentInfo: FileContentInfo;
  fileInfo: DetectedFileType;
}>();

const emits = defineEmits<{
  (event: 'fileLoaded'): void;
}>();

onMounted(async () => {
  try {
    await onFileLoaded(contentInfo, fileInfo);
    loadCryptpad();
    await openFileWithCryptpad();
    emits('fileLoaded');
  } catch (error) {
    console.error('Error during file editor initialization:', error);
    throw error;
  }
});

onUnmounted(() => {
  contentInfoRef.value = undefined;
  cryptpadInstance.value = null;
  documentType.value = null;
  workspaceHandleRef.value = undefined;
  documentPathRef.value = undefined;
  if (fileUrl.value) {
    URL.revokeObjectURL(fileUrl.value);
    fileUrl.value = null;
  }
});

async function onFileLoaded(contentInfo: FileContentInfo, fileInfo: DetectedFileType): Promise<void> {
  contentInfoRef.value = contentInfo;
  documentType.value = getDocumentTypeFromExtension(fileInfo.extension);
  workspaceHandleRef.value = getWorkspaceHandle();
  documentPathRef.value = getDocumentPath();

  if (!workspaceHandleRef.value) {
    throwError('Cannot initialize Cryptpad editor: missing workspace handle');
  }

  try {
    validateContentInfo(contentInfoRef.value);
  } catch (error) {
    throwError(`Cannot initialize Cryptpad editor: failed to validate content info: ${error}`);
  }

  if (!documentType.value) {
    throwError(`No document type for file with extension '${fileInfo.extension}'`);
  }
}

function loadCryptpad(): void {
  try {
    // Clear the DOM element before creating a new instance
    if (fileEditorRef.value) {
      fileEditorRef.value.innerHTML = '';
    }

    // Always create a new instance for each document to avoid conflicts
    cryptpadInstance.value = new Cryptpad(fileEditorRef.value as HTMLDivElement, Env.getDefaultCryptpadServer());
  } catch (error) {
    throwError(`Failed to load CryptPad editor: ${error}`);
  }
}

async function openFileWithCryptpad(): Promise<void> {
  const cryptpadApi = cryptpadInstance.value as Cryptpad;
  const contentInfo = contentInfoRef.value as FileContentInfo;
  const workspaceHandle = workspaceHandleRef.value as number;
  const documentPath = documentPathRef.value as FsPath;

  fileUrl.value = URL.createObjectURL(new Blob([contentInfo.data], { type: 'application/octet-stream' }));

  const cryptpadConfig = {
    document: {
      url: fileUrl.value,
      fileType: contentInfo.extension,
      title: contentInfo.fileName,
      key: contentInfo.fileId,
    },
    documentType: documentType.value as CryptpadDocumentType, // Safe since we checked for null earlier
    editorConfig: {
      lang: longLocaleCodeToShort(I18n.getLocale()),
    },
    autosave: 10,
    events: {
      onSave: async (file: Blob, callback: () => void): Promise<void> => {
        // Handle save logic here
        const openResult = await openFile(workspaceHandle, documentPath, { write: true, truncate: true });

        if (!openResult.ok) {
          return undefined;
        }
        const fd = openResult.value;
        const arrayBuffer = await file.arrayBuffer();
        await writeFile(workspaceHandle, fd, 0, new Uint8Array(arrayBuffer));
        await closeFile(workspaceHandle, fd);
        callback();
      },
      onHasUnsavedChanges: (unsaved: boolean): void => {
        console.log('Cryptpad unsaved changes:', unsaved);
      },
    },
  };

  try {
    await cryptpadApi.open(cryptpadConfig);
    window.electronAPI.log('info', 'CryptPad editor initialized successfully');
  } catch (error) {
    throwError(`Failed to open CryptPad editor: ${error}`);
  }
}

function validateContentInfo(contentInfo: FileContentInfo | undefined): void {
  if (!contentInfo) {
    throw new Error('Content info is not available');
  }

  if (!contentInfo.extension || contentInfo.extension.trim() === '') {
    throw new Error('File extension is missing or empty');
  }

  if (!contentInfo.fileName || contentInfo.fileName.trim() === '') {
    throw new Error('File name is missing or empty');
  }

  if (!contentInfo.fileId || contentInfo.fileId.trim() === '') {
    throw new Error('File ID is missing or empty');
  }

  if (!contentInfo.data || contentInfo.data.length === 0) {
    throw new Error('File data is missing or empty');
  }
}

function throwError(message: string): never {
  window.electronAPI.log('error', message);
  informationManager.present(
    new Information({
      message: 'fileViewers.genericError',
      level: InformationLevel.Error,
    }),
    PresentationMode.Toast,
  );
  throw new Error(message);
}
</script>

<style scoped lang="scss"></style>
