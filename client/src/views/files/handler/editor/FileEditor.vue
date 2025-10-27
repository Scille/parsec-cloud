<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="file-editor"
    id="file-editor"
    ref="fileEditor"
    v-if="!error"
  />
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
</template>

<script setup lang="ts">
import { DetectedFileType } from '@/common/fileTypes';
import { ClientInfo, closeFile, FileDescriptor, openFile, writeFile } from '@/parsec';
import { getWorkspaceHandle, routerGoBack } from '@/router';
import {
  Cryptpad,
  CryptpadAppMode,
  CryptpadDocumentType,
  CryptpadError,
  CryptpadErrorCode,
  getCryptpadDocumentType,
} from '@/services/cryptpad';
import { Env } from '@/services/environment';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { longLocaleCodeToShort } from '@/services/translation';
import { SaveState } from '@/views/files/handler/editor';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import { IonButton, IonIcon, IonItem, IonList, IonText } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import { I18n, Translatable } from 'megashark-lib';
import { inject, onMounted, onUnmounted, ref, useTemplateRef } from 'vue';

const informationManager: InformationManager = inject(InformationManagerKey)!;
const fileEditorRef = useTemplateRef('fileEditor');
const documentType = ref<CryptpadDocumentType | null>(null);
const cryptpadInstance = ref<Cryptpad | null>(null);
const fileUrl = ref<string | null>(null);
const error = ref('');

const {
  contentInfo,
  fileInfo,
  readOnly,
  userInfo = undefined,
} = defineProps<{
  contentInfo: FileContentInfo;
  fileInfo: DetectedFileType;
  readOnly?: boolean;
  userInfo?: ClientInfo;
}>();

const emits = defineEmits<{
  (event: 'fileLoaded'): void;
  (event: 'fileError'): void;
  (event: 'onSaveStateChange', saveState: SaveState): void;
}>();

onMounted(async () => {
  documentType.value = getCryptpadDocumentType(fileInfo.type);

  if (documentType.value === CryptpadDocumentType.Unsupported) {
    error.value = 'fileViewers.errors.titles.unsupportedFileType';
    await informationManager.present(
      new Information({
        title: 'fileViewers.errors.titles.unsupportedFileType',
        message: 'fileViewers.errors.informationEditDownload',
        level: InformationLevel.Info,
      }),
      PresentationMode.Modal,
    );
    return;
  }
  if (!(await loadCryptpad())) {
    return;
  }
  if (await openFileWithCryptpad()) {
    emits('fileLoaded');
  } else {
    emits('fileError');
  }
});

onUnmounted(() => {
  cryptpadInstance.value = null;
  if (fileUrl.value) {
    URL.revokeObjectURL(fileUrl.value);
  }
});

async function loadCryptpad(): Promise<boolean> {
  try {
    // Clear the DOM element before creating a new instance
    if (fileEditorRef.value) {
      fileEditorRef.value.innerHTML = '';
    }

    // Always create a new instance for each document to avoid conflicts
    cryptpadInstance.value = new Cryptpad(fileEditorRef.value as HTMLDivElement, Env.getDefaultCryptpadServer());
    await cryptpadInstance.value.init();
    return true;
  } catch (e: unknown) {
    let title: Translatable = 'fileViewers.errors.titles.genericError';
    const message: Translatable = 'fileViewers.errors.informationEditDownload';
    const level: InformationLevel = InformationLevel.Info;

    if (e instanceof CryptpadError) {
      switch (e.code) {
        case CryptpadErrorCode.NotEnabled:
          title = 'fileViewers.errors.titles.editionNotAvailable';
          break;
        case CryptpadErrorCode.ScriptElementCreationFailed:
        case CryptpadErrorCode.InitFailed:
          title = 'fileViewers.errors.titles.genericError';
      }
    }
    await informationManager.present(
      new Information({
        title,
        message,
        level,
      }),
      PresentationMode.Modal,
    );
    return false;
  }
}

async function openFileWithCryptpad(): Promise<boolean> {
  const cryptpadApi = cryptpadInstance.value as Cryptpad;
  const workspaceHandle = getWorkspaceHandle();
  const documentPath = contentInfo.path;
  const user = userInfo ? { name: userInfo.humanHandle.label, id: userInfo.userId } : undefined;

  if (!workspaceHandle) {
    error.value = 'fileViewers.errors.genericError';
    await informationManager.present(
      new Information({
        message: 'fileViewers.errors.titles.genericError',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return false;
  }
  fileUrl.value = URL.createObjectURL(new Blob([contentInfo.data as Uint8Array<ArrayBuffer>], { type: 'application/octet-stream' }));
  let isSaving = false;

  const cryptpadConfig = {
    document: {
      url: fileUrl.value,
      fileType: contentInfo.extension,
      title: contentInfo.fileName,
      // TODO: replace UUID with collaborative session key
      // key: contentInfo.fileId,
      key: crypto.randomUUID(),
    },
    documentType: documentType.value as CryptpadDocumentType, // Safe since we checked for null earlier
    editorConfig: {
      lang: longLocaleCodeToShort(I18n.getLocale()),
      user,
    },
    autosave: (window as any).TESTING_EDITICS_SAVE_TIMEOUT ?? 10,
    mode: readOnly ? CryptpadAppMode.View : CryptpadAppMode.Edit,
    events: {
      onSave: async (file: Blob, callback: () => void): Promise<void> => {
        let hasError = false;
        let fd: FileDescriptor | undefined = undefined;
        const start = Date.now();
        try {
          if (readOnly) {
            return;
          }
          isSaving = true;
          emits('onSaveStateChange', SaveState.Saving);
          // Handle save logic here
          const openResult = await openFile(workspaceHandle, documentPath, { write: true, truncate: true });

          if (!openResult.ok) {
            window.electronAPI.log('error', `Failed to open file: ${openResult.error.tag} (${openResult.error.error})`);
            hasError = true;
            return;
          }
          fd = openResult.value;
          const arrayBuffer = await file.arrayBuffer();
          const writeResult = await writeFile(workspaceHandle, fd, 0, new Uint8Array(arrayBuffer));
          if (!writeResult.ok) {
            hasError = true;
            window.electronAPI.log('error', `Failed to write file: ${writeResult.error.tag} (${writeResult.error.error})`);
          }
        } catch (e: any) {
          window.electronAPI.log('error', `Failed to save file: ${e.toString()}`);
          hasError = true;
        } finally {
          if (fd) {
            await closeFile(workspaceHandle, fd);
          }
          callback();
          const end = Date.now();
          setTimeout(
            () => {
              if (isSaving === true) {
                isSaving = false;
                if (!hasError) {
                  emits('onSaveStateChange', SaveState.Saved);
                } else {
                  emits('onSaveStateChange', SaveState.Error);
                }
              }
            },
            Math.max(1000 - (end - start), 0),
          );
        }
      },
      onHasUnsavedChanges: (unsaved: boolean): void => {
        if (unsaved) {
          isSaving = false;
          emits('onSaveStateChange', SaveState.Unsaved);
        }
      },
    },
  };

  try {
    await cryptpadApi.open(cryptpadConfig);
    window.electronAPI.log('info', 'CryptPad editor initialized successfully');
    return true;
  } catch (e: unknown) {
    let title: Translatable = 'fileViewers.errors.titles.genericError';
    const message: Translatable = 'fileViewers.errors.informationEditDownload';
    const level: InformationLevel = InformationLevel.Info;

    if (e instanceof CryptpadError) {
      switch (e.code) {
        case CryptpadErrorCode.NotInitialized:
          title = 'fileViewers.errors.titles.genericError';
          break;
        case CryptpadErrorCode.DocumentTypeNotEnabled:
          title = 'fileViewers.errors.titles.editionNotAvailable';
      }
    }

    await informationManager.present(
      new Information({
        title,
        message,
        level,
      }),
      PresentationMode.Modal,
    );
    return false;
  }
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
