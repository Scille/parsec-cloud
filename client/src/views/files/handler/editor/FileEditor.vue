<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="file-editor"
    id="file-editor"
    ref="fileEditor"
  >
    <div
      v-if="error"
      class="file-editor-error"
    >
      <div class="error-content">
        <div class="error-content-text">
          <ion-text class="error-content-text__title title-h3">{{ $msTranslate('fileEditors.globalTitle') }}</ion-text>
          <ion-text class="error-content-text__message body-lg">{{ $msTranslate(error) }}</ion-text>
        </div>
        <div class="error-content-buttons">
          <ion-button
            class="error-content-buttons__item button-default"
            @click="routerGoBack()"
          >
            {{ $msTranslate('fileEditors.actions.backToFiles') }}
          </ion-button>
        </div>
      </div>

      <div class="error-advices">
        <ion-text class="error-advices__title title-h4">{{ $msTranslate('fileEditors.advices.title') }}</ion-text>
        <ion-list class="error-advices-list ion-no-padding">
          <ion-item class="error-advices-list__item ion-no-padding body">
            <ion-icon
              class="item-icon"
              :icon="checkmarkCircle"
            />
            {{ $msTranslate('fileEditors.advices.advice1') }}
          </ion-item>
          <ion-item class="error-advices-list__item ion-no-padding body">
            <ion-icon
              class="item-icon"
              :icon="checkmarkCircle"
            />
            {{ $msTranslate('fileEditors.advices.advice2') }}
          </ion-item>
          <ion-item class="error-advices-list__item ion-no-padding body">
            <ion-icon
              class="item-icon"
              :icon="checkmarkCircle"
            />
            {{ $msTranslate('fileEditors.advices.advice3') }}
          </ion-item>
        </ion-list>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IonButton, IonIcon, IonList, IonItem, IonText } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import { closeFile, FsPath, openFile, writeFile } from '@/parsec';
import { I18n, Translatable } from 'megashark-lib';
import { ref, Ref, inject, useTemplateRef, onMounted, onUnmounted } from 'vue';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { getDocumentPath, getWorkspaceHandle, routerGoBack } from '@/router';
import { DetectedFileType } from '@/common/fileTypes';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import { Env } from '@/services/environment';
import { CryptpadDocumentType, Cryptpad, getDocumentTypeFromExtension, CryptpadError, CryptpadErrorCode } from '@/services/cryptpad';
import { longLocaleCodeToShort } from '@/services/translation';

const informationManager: InformationManager = inject(InformationManagerKey)!;
const contentInfoRef: Ref<FileContentInfo | undefined> = ref(undefined);
const fileEditorRef = useTemplateRef('fileEditor');
const documentType = ref<CryptpadDocumentType | null>(null);
const workspaceHandleRef = ref<number | undefined>(undefined);
const documentPathRef = ref<FsPath>();
const cryptpadInstance = ref<Cryptpad | null>(null);
const fileUrl = ref<string | null>(null);
const error = ref('');

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

async function loadCryptpad(): Promise<void> {
  try {
    // Clear the DOM element before creating a new instance
    if (fileEditorRef.value) {
      fileEditorRef.value.innerHTML = '';
    }

    // Always create a new instance for each document to avoid conflicts
    cryptpadInstance.value = new Cryptpad(fileEditorRef.value as HTMLDivElement, Env.getDefaultCryptpadServer());
    await cryptpadInstance.value.init();
  } catch (e: unknown) {
    if (e instanceof CryptpadError) {
      let title: Translatable = 'fileViewers.errors.titles.genericError';
      let message: Translatable = 'fileViewers.errors.informationEditDownload';
      const level: InformationLevel = InformationLevel.Info;

      switch (e.code) {
        case CryptpadErrorCode.NotEnabled:
          title = 'fileViewers.errors.titles.editionNotAvailable';
          message = 'fileViewers.errors.informationEditDownload';
          break;
        case CryptpadErrorCode.ScriptElementCreationFailed:
          title = 'fileViewers.errors.titles.genericError';
          message = 'fileViewers.errors.informationEditDownload';
          break;
        case CryptpadErrorCode.InitFailed:
          title = 'fileViewers.errors.titles.genericError';
          message = 'fileViewers.errors.informationEditDownload';
      }

      await informationManager.present(
        new Information({
          title,
          message,
          level,
        }),
        PresentationMode.Modal,
      );
    }
  }
}

async function openFileWithCryptpad(): Promise<void> {
  const cryptpadApi = cryptpadInstance.value as Cryptpad;
  const contentInfo = contentInfoRef.value as FileContentInfo;
  const workspaceHandle = workspaceHandleRef.value as number;
  const documentPath = documentPathRef.value as FsPath;

  fileUrl.value = URL.createObjectURL(new Blob([contentInfo.data] as Uint8Array<ArrayBuffer>[], { type: 'application/octet-stream' }));

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
  } catch (e: unknown) {
    if (e instanceof CryptpadError) {
      let title: Translatable = 'fileViewers.errors.titles.genericError';
      let message: Translatable = 'fileViewers.errors.informationEditDownload';
      const level: InformationLevel = InformationLevel.Info;

      switch (e.code) {
        case CryptpadErrorCode.NotInitialized:
          title = 'fileViewers.errors.titles.genericError';
          message = 'fileViewers.errors.informationEditDownload';
          break;
        case CryptpadErrorCode.DocumentTypeNotEnabled:
          title = 'fileViewers.errors.titles.editionNotAvailable';
          message = 'fileViewers.errors.informationEditDownload';
      }

      await informationManager.present(
        new Information({
          title,
          message,
          level,
        }),
        PresentationMode.Modal,
      );
    }
  }
}

function validateContentInfo(contentInfo: FileContentInfo | undefined): void {
  if (!contentInfo) {
    error.value = 'fileEditors.errors.contentInfoMissing';
  } else {
    if (!contentInfo.extension || contentInfo.extension.trim() === '') {
      error.value = 'fileEditors.errors.fileExtensionMissing';
    }

    if (!contentInfo.fileName || contentInfo.fileName.trim() === '') {
      error.value = 'fileEditors.errors.fileNameMissing';
    }

    if (!contentInfo.fileId || contentInfo.fileId.trim() === '') {
      error.value = 'fileEditors.errors.fileIdMissing';
    }

    if (!contentInfo.data || contentInfo.data.length === 0) {
      error.value = 'fileEditors.errors.fileDataMissing';
    }
  }
}

function throwError(message: string): never {
  window.electronAPI.log('error', message);
  informationManager.present(
    new Information({
      message: 'fileViewers.errors.titles.genericError',
      level: InformationLevel.Error,
    }),
    PresentationMode.Toast,
  );
  throw new Error(message);
}
</script>

<style scoped lang="scss">
.file-editor {
  height: 100%;
  background: var(--parsec-color-light-secondary-premiere);
}

.file-editor-error {
  display: flex;
  flex-direction: column;
  gap: 2rem;
  max-width: 32rem;
  margin: auto;
  justify-content: center;
  align-items: center;
  height: 100%;
}

.error-content {
  display: flex;
  flex-direction: column;
  width: 100%;
  gap: 1.5rem;
  background: var(--parsec-color-light-secondary-white);
  padding: 1.5rem;
  border-radius: var(--parsec-radius-12);
  box-shadow: var(--parsec-shadow-light);

  &-text {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;

    &__title {
      color: var(--parsec-color-light-secondary-text);
    }

    &__message {
      color: var(--parsec-color-light-secondary-hard-grey);
    }
  }

  &-buttons {
    display: flex;
    justify-content: flex-end;
    gap: 1rem;

    &__item {
      &:first-child {
        --background: none;
        --color: var(--parsec-color-light-secondary-text);
        --border-color: var(--parsec-color-light-secondary-text);
        --color-hover: var(--parsec-color-light-secondary-text);
        --background-hover: var(--parsec-color-light-secondary-medium);
      }

      &:last-child {
        --color: var(--parsec-color-light-secondary-white);
        --background: var(--parsec-color-light-secondary-text);
        --background-hover: var(--parsec-color-light-secondary-contrast);
      }
    }
  }
}

.error-advices {
  border-top: 1px solid var(--parsec-color-light-secondary-disabled);
  padding: 2rem 1rem 1rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;

  &__title {
    color: var(--parsec-color-light-secondary-text);
  }

  &-list {
    padding-left: 0.5rem;
    list-style-type: circle;
    background: none;

    &__item {
      margin-bottom: 0.5rem;
      display: flex;
      align-items: center;
      gap: 0.5rem;
      color: var(--parsec-color-light-secondary-soft-text);
      --background: none;
      font-size: 0.9375rem;

      .item-icon {
        color: var(--parsec-color-light-secondary-grey);
        flex-shrink: 0;
        margin-right: 0.5rem;
        font-size: 1rem;
      }
    }
  }
}
</style>
