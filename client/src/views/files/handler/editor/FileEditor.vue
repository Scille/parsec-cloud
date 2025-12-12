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
          {{ $msTranslate(EditorButtonAction.BackToFiles) }}
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
      </ion-list>
    </div>
  </div>
</template>

<script setup lang="ts">
import { DetectedFileType } from '@/common/fileTypes';
import { ClientInfo, closeFile, FileDescriptor, openFile, writeFile } from '@/parsec';
import { currentRouteIs, getFileHandlerMode, getWorkspaceHandle, routerGoBack, Routes } from '@/router';
import {
  Cryptpad,
  CryptpadAppMode,
  CryptpadDocumentType,
  CryptpadError,
  CryptpadErrorCode,
  getCryptpadDocumentType,
} from '@/services/cryptpad';
import { Env } from '@/services/environment';
import { EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';

import { longLocaleCodeToShort } from '@/services/translation';
import { EditorButtonAction, EditorErrorMessage, EditorErrorTitle, EditorIssueStatus, SaveState } from '@/views/files/handler/editor';
import EditorIssueModal from '@/views/files/handler/editor/EditorIssueModal.vue';
import { FileHandlerMode } from '@/views/files/handler/types';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import { IonButton, IonIcon, IonItem, IonList, IonText, modalController } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import { I18n, MsModalResult } from 'megashark-lib';
import { inject, onMounted, onUnmounted, ref, useTemplateRef, watch } from 'vue';

const fileEditorRef = useTemplateRef('fileEditor');
const documentType = ref<CryptpadDocumentType | null>(null);
const cryptpadInstance = ref<Cryptpad | null>(null);
const fileUrl = ref<string | null>(null);
const error = ref('');
const eventDistributor: EventDistributor = inject(EventDistributorKey)!;
let eventCbId: null | string = null;
const fileLoaded = ref(false);

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
    error.value = EditorErrorTitle.UnsupportedFileType;
    await openIssueModal(EditorIssueStatus.UnsupportedFileType);
    return;
  }
  if (!(await loadCryptpad())) {
    return;
  }
  const openResult = await openFileWithCryptpad();
  if (openResult) {
    emits('fileLoaded');
  } else {
    emits('fileError');
  }

  eventCbId = await eventDistributor.registerCallback(Events.Online | Events.Offline, async (event: Events) => {
    if (event === Events.Offline) {
      window.electronAPI.log('warn', 'Network connection lost while editing');
      emits('onSaveStateChange', SaveState.Offline);
      await openIssueModal(EditorIssueStatus.NetworkOffline, false);
    } else if (event === Events.Online) {
      window.electronAPI.log('info', 'Network connection restored');
      emits('onSaveStateChange', SaveState.None);
    }
  });
});

onUnmounted(() => {
  // Clean up CryptPad instance and event listeners
  if (cryptpadInstance.value) {
    cryptpadInstance.value = null;
  }
  if (fileUrl.value) {
    URL.revokeObjectURL(fileUrl.value);
  }

  if (eventCbId) {
    eventDistributor.removeCallback(eventCbId);
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
    if (e instanceof CryptpadError) {
      switch (e.code) {
        case CryptpadErrorCode.NotEnabled:
          await openIssueModal(EditorIssueStatus.EditionNotAvailable);
          break;
        case CryptpadErrorCode.ScriptElementCreationFailed:
        case CryptpadErrorCode.InitFailed:
          error.value = EditorErrorMessage.EditableOnlyOnSystem;
          break;
      }
    }
    return false;
  }
}

async function openIssueModal(status: EditorIssueStatus, redirectAfterDismiss = true): Promise<MsModalResult> {
  // Safety check: only show modal if we're still on the file handler/editor route
  if (!currentRouteIs(Routes.FileHandler) || getFileHandlerMode() !== FileHandlerMode.Edit) {
    window.electronAPI.log('info', 'Skipping modal - user navigated away from editor');
    return MsModalResult.Cancel;
  }

  const modal = await modalController.create({
    component: EditorIssueModal,
    cssClass: 'editor-issue-modal',
    componentProps: {
      status,
      fileLoaded,
    },
    backdropDismiss: false,
  });

  await modal.present();
  const { role } = await modal.onWillDismiss();

  // Handle redirection if requested (default is true)
  if (redirectAfterDismiss) {
    await routerGoBack();
    // TODO: find why we have router problems causing routerGoBack to stay on same page
    // https://github.com/Scille/parsec-cloud/issues/11749
    // Seems similar to the header back button double click issue
    // For now, we'll force navigation if page is still the same after modal
    if (currentRouteIs(Routes.FileHandler) && getFileHandlerMode() === FileHandlerMode.Edit) {
      await routerGoBack();
    }
  }

  return role as MsModalResult;
}

async function openTimeoutModal(): Promise<'wait' | 'close'> {
  const role = await openIssueModal(EditorIssueStatus.LoadingTimeout, false);

  // If user clicks primary button (close/dismiss)
  if (role === MsModalResult.Confirm) {
    if (fileLoaded.value) {
      return 'close';
    } else {
      return 'wait';
    }
  } else if (role === MsModalResult.Cancel) {
    await routerGoBack();
    // TODO: find why we have router problems causing routerGoBack to stay on same page
    // https://github.com/Scille/parsec-cloud/issues/11749
    if (currentRouteIs(Routes.FileHandler) && getFileHandlerMode() === FileHandlerMode.Edit) {
      await routerGoBack();
    }
    return 'close';
  }

  // Modal was dismissed (close button) - just close without navigating
  return 'close';
}

async function openFileWithCryptpad(): Promise<boolean> {
  const cryptpadApi = cryptpadInstance.value as Cryptpad;
  const workspaceHandle = getWorkspaceHandle();
  const documentPath = contentInfo.path;
  const user = userInfo ? { name: userInfo.humanHandle.label, id: userInfo.userId } : undefined;

  if (!workspaceHandle) {
    error.value = EditorErrorTitle.GenericError;
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
      onReady: (): void => {
        window.electronAPI.log('info', 'CryptPad editor is ready and document loaded successfully');
        fileLoaded.value = true;
      },
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
      onError: (errorData: { message: string; errorType: string; originalError: string }): void => {
        window.electronAPI.log('error', `CryptPad error [${errorData.errorType}]: ${errorData.message}`);
        error.value = 'fileViewers.errors.titles.genericError';
        emits('fileError');
      },
    },
  };

  try {
    // Start CryptPad (non-blocking)
    await cryptpadApi.open(cryptpadConfig);

    // Set up timeout for loading files
    // If the file does not open before timeout, a dialog is displayed
    // to ask the user if it wants to keep waiting or go back to files.
    const LOADING_TIMEOUT_MS = 30000;
    let shouldContinueWaiting = true;

    // TODO: Fix DOCX timeout false positives - OnlyOffice doesn't reliably call onReady for 'doc' type
    // See: https://github.com/Scille/parsec-cloud/issues/11753
    // For now, we ignore timeout for DOCX files to avoid false positive "corrupted file" modals
    const shouldSkipTimeout = documentType.value === CryptpadDocumentType.Doc;

    if (shouldSkipTimeout) {
      window.electronAPI.log('info', 'Skipping timeout for DOCX file');
      return true;
    }

    // Wait for file to load with timeout and restart capability
    while (!fileLoaded.value && shouldContinueWaiting) {
      // Wait for timeout or file load
      const timeoutOccurred = await new Promise<boolean>((resolve) => {
        const timeoutId = window.setTimeout(() => {
          resolve(true); // Timeout occurred
        }, LOADING_TIMEOUT_MS);

        // Watch for file loaded to resolve immediately
        const stopWatch = watch(fileLoaded, (loaded) => {
          if (loaded) {
            window.clearTimeout(timeoutId);
            stopWatch();
            resolve(false); // File loaded
          }
        });
      });

      // If file loaded, exit successfully
      if (!timeoutOccurred) {
        window.electronAPI.log('info', 'CryptPad editor initialized successfully');
        return true;
      }

      // Timeout occurred - show modal
      window.electronAPI.log('warn', 'CryptPad loading timeout - file appears to be corrupted or too large');
      const result = await openTimeoutModal();

      // Check if file loaded while modal was open
      if (fileLoaded.value) {
        window.electronAPI.log('info', 'File loaded while modal was open');
        return true;
      }

      // Continue waiting if user clicked wait, stop if they clicked close
      shouldContinueWaiting = result === 'wait';
      if (shouldContinueWaiting) {
        window.electronAPI.log('info', 'User chose to wait - restarting timeout');
      }
    }

    return fileLoaded.value;
  } catch (e: unknown) {
    // Handle initialization errors
    if (e instanceof CryptpadError) {
      emits('fileError');
      switch (e.code) {
        case CryptpadErrorCode.NotInitialized:
          error.value = EditorErrorMessage.EditableOnlyOnSystem;
          break;
        case CryptpadErrorCode.DocumentTypeNotEnabled:
          await openIssueModal(EditorIssueStatus.EditionNotAvailable);
          break;
      }
    }

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
      width: 100%;
      &:first-child {
        --background: var(--parsec-color-light-secondary-text);
        --color: var(--parsec-color-light-secondary-white);
        --border-color: var(--parsec-color-light-secondary-text);
        --color-hover: var(--parsec-color-light-secondary-text);
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
