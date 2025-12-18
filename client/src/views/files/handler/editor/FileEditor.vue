<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <iframe
    class="file-editor"
    ref="editorFrame"
    v-if="!error"
    v-show="frameReady"
  />
  <div
    v-if="!frameReady"
    class="loading-container"
  >
    <div class="loading-content">
      <!-- prettier-ignore -->
      <ms-image
        :image="(ResourcesManager.instance().get(Resources.LogoIcon, LogoIconGradient) as string)"
        class="logo-img"
      />
      <ms-spinner title="fileEditors.loading" />
    </div>
  </div>
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
  CryptpadEditors,
  CryptpadError,
  CryptpadErrorCodes,
  CryptpadOpenModes,
  getCryptpadEditor,
  openDocument,
} from '@/services/cryptpad';
import { EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { longLocaleCodeToShort } from '@/services/translation';
import { EditorButtonAction, EditorErrorMessage, EditorErrorTitle, EditorIssueStatus, SaveState } from '@/views/files/handler/editor';
import EditorIssueModal from '@/views/files/handler/editor/EditorIssueModal.vue';
import { FileHandlerMode } from '@/views/files/handler/types';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import { IonButton, IonIcon, IonItem, IonList, IonText, modalController } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import { I18n, LogoIconGradient, MsImage, MsModalResult, MsSpinner } from 'megashark-lib';
import { inject, onMounted, onUnmounted, ref, useTemplateRef, watch } from 'vue';

const editorFrame = useTemplateRef<HTMLIFrameElement>('editorFrame');
const documentType = ref<CryptpadEditors>(CryptpadEditors.Unsupported);
const error = ref('');
const eventDistributor: EventDistributor = inject(EventDistributorKey)!;
let eventCbId: null | string = null;
const loadFinished = ref(false);
let abortController: AbortController | undefined = undefined;
const frameReady = ref(false);

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
  documentType.value = getCryptpadEditor(fileInfo.type);

  if (documentType.value === CryptpadEditors.Unsupported) {
    error.value = EditorErrorTitle.UnsupportedFileType;
    await openIssueModal(EditorIssueStatus.UnsupportedFileType);
    return;
  }

  await loadEditor();

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
  if (abortController) {
    abortController.abort();
    abortController = undefined;
  }

  if (eventCbId) {
    eventDistributor.removeCallback(eventCbId);
  }
});

async function loadEditor(): Promise<void> {
  const workspaceHandle = getWorkspaceHandle();

  if (!workspaceHandle) {
    window.electronAPI.log('error', 'Cannot retrieve workspace handle');
    return;
  }
  if (!editorFrame.value) {
    window.electronAPI.log('error', 'Cannot get the iframe element');
    return;
  }
  frameReady.value = false;
  if (abortController) {
    abortController.abort();
    abortController = undefined;
  }

  let isSaving = false;
  abortController = await openDocument(
    {
      documentContent: contentInfo.data,
      documentName: contentInfo.fileName,
      documentExtension: contentInfo.extension,
      cryptpadEditor: documentType.value as CryptpadEditors,
      key: crypto.randomUUID(),
      userName: userInfo ? userInfo.humanHandle.label : I18n.translate('UNKNOWN_USER'),
      userId: userInfo ? userInfo.userId : crypto.randomUUID(),
      autosaveInterval: 10,
      mode: readOnly ? CryptpadOpenModes.View : CryptpadOpenModes.Edit,
      locale: longLocaleCodeToShort(I18n.getLocale()),
    },
    {
      onReady: (): void => {
        window.electronAPI.log('info', 'CryptPad editor is ready and document loaded successfully');
        loadFinished.value = true;
        emits('fileLoaded');
      },
      onSave: async (file: Blob): Promise<void> => {
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
          const openResult = await openFile(workspaceHandle, contentInfo.path, { write: true, truncate: true });

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
            (window as any).TESTING === true ? 0 : Math.max(1000 - (end - start), 0),
          );
        }
      },
      onHasUnsavedChanges: (unsaved: boolean): void => {
        if (unsaved) {
          isSaving = false;
          emits('onSaveStateChange', SaveState.Unsaved);
        }
      },
      onError: async (err: unknown): Promise<void> => {
        emits('fileError');
        loadFinished.value = true;
        error.value = 'fileViewers.errors.titles.genericError';

        if (err instanceof CryptpadError) {
          switch (err.code) {
            case CryptpadErrorCodes.InitFailed:
              error.value = EditorErrorMessage.EditableOnlyOnSystem;
              break;
            case CryptpadErrorCodes.OpenFailed:
            case CryptpadErrorCodes.OpenInvalidConfig:
              error.value = 'fileEditors.errors.titles.openFailed';
              break;
            case CryptpadErrorCodes.FrameLoadFailed:
            case CryptpadErrorCodes.FrameNotLoaded:
              error.value = 'fileEditors.errors.titles.frameLoadFailed';
              break;
          }
        } else {
          window.electronAPI.log('error', `Unhandled error: ${err} `);
        }
      },
    },
    editorFrame.value,
  );
  frameReady.value = true;

  await handleTimeout();
}

async function handleTimeout(): Promise<void> {
  // Set up timeout for loading files
  // If the file does not open before timeout, a dialog is displayed
  // to ask the user if it wants to keep waiting or go back to files.
  const LOADING_TIMEOUT_MS = 30000;
  let shouldContinueWaiting = true;

  // TODO: Fix timeout false positives - OnlyOffice doesn't reliably call onReady in some cases
  // See: https://github.com/Scille/parsec-cloud/issues/11753
  // For now, we ignore timeout for some files to avoid false positive "corrupted file" modals
  const shouldSkipTimeout =
    documentType.value === CryptpadEditors.Doc ||
    documentType.value === CryptpadEditors.Presentation ||
    (documentType.value === CryptpadEditors.Sheet && readOnly);

  if (shouldSkipTimeout) {
    window.electronAPI.log('info', 'Skipping timeout for DOCX and PPTX files');
    return;
  }

  // Wait for file to load with timeout and restart capability
  while (!loadFinished.value && shouldContinueWaiting) {
    // Wait for timeout or file load
    const timeoutOccurred = await new Promise<boolean>((resolve) => {
      const timeoutId = window.setTimeout(() => {
        resolve(true); // Timeout occurred
      }, LOADING_TIMEOUT_MS);

      // Watch for file loaded to resolve immediately
      const stopWatch = watch(loadFinished, (loaded) => {
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
      return;
    }

    // Timeout occurred - show modal
    window.electronAPI.log('warn', 'CryptPad loading timeout - file appears to be corrupted or too large');
    const result = await openTimeoutModal();

    // Check if file loaded while modal was open
    if (loadFinished.value) {
      window.electronAPI.log('info', 'File loaded while modal was open');
      return;
    }

    // Continue waiting if user clicked wait, stop if they clicked close
    shouldContinueWaiting = result === 'wait';
    if (shouldContinueWaiting) {
      window.electronAPI.log('info', 'User chose to wait - restarting timeout');
    }
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
      loadFinished,
    },
    backdropDismiss: false,
  });

  await modal.present();
  const { role } = await modal.onWillDismiss();

  // Handle redirection if requested (default is true)
  if (redirectAfterDismiss) {
    await routerGoBack();
  }

  return role as MsModalResult;
}

async function openTimeoutModal(): Promise<'wait' | 'close'> {
  const role = await openIssueModal(EditorIssueStatus.LoadingTimeout, false);

  // If user clicks primary button (close/dismiss)
  if (role === MsModalResult.Confirm) {
    if (loadFinished.value) {
      return 'close';
    } else {
      return 'wait';
    }
  } else if (role === MsModalResult.Cancel) {
    await routerGoBack();
    return 'close';
  }

  // Modal was dismissed (close button) - just close without navigating
  return 'close';
}
</script>

<style scoped lang="scss">
.file-editor {
  height: 100%;
  background: var(--parsec-color-light-secondary-premiere);
  border: none;
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

.loading-container {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  width: 100%;
  height: 100%;
  user-select: none;
}

@keyframes LogoFadeIn {
  0% {
    opacity: 0;
  }
  100% {
    opacity: 1;
  }
}

.loading-content {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  width: fit-content;
  gap: 0.5rem;

  .logo-img {
    animation: LogoFadeIn 0.8s ease-in-out;
    width: 3.25rem;
    height: 3.25rem;
  }
}
</style>
