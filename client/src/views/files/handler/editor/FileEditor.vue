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

    <div
      class="error-advices"
      v-show="showErrorTips"
    >
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
import { getFileContent } from '@/common/file';
import { ClientInfo, closeFile, openFile, WorkspaceHandle, writeFile } from '@/parsec';
import { currentRouteIs, getFileHandlerMode, getWorkspaceHandle, routerGoBack, Routes } from '@/router';
import {
  EditicsError,
  EditicsErrorCodes,
  EditicsHostSession,
  EditicsSaveState,
  getEditicsDocumentType,
  openDocument,
} from '@/services/editics';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { longLocaleCodeToShort } from '@/services/translation';
import { EditorButtonAction, EditorErrorTitle, EditorIssueStatus } from '@/views/files/handler/editor';
import EditorIssueModal from '@/views/files/handler/editor/EditorIssueModal.vue';
import { FileHandlerMode, SaveState } from '@/views/files/handler/types';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import type { EditicsDocumentTypes } from '@editics_parent_host_api';
import { IonButton, IonIcon, IonItem, IonList, IonText, modalController } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import { I18n, LogoIconGradient, MsImage, MsModalResult, MsSpinner } from 'megashark-lib';
import { onMounted, onUnmounted, ref, useTemplateRef } from 'vue';

// Time to wait for the document to be fully loaded (fonts, dictionaries, etc.) before offering
// the user the option to keep waiting or give up, see openTimeoutModal(). OnlyOffice's own assets
// (sdkjs, fonts, dictionaries) are heavy and can take a while to load on first use, so this is
// generous on purpose (the editor shows a loading state in the meantime).
const READY_TIMEOUT_MS = 60_000;

const editorFrame = useTemplateRef<HTMLIFrameElement>('editorFrame');
const error = ref('');
const showErrorTips = ref(false);
const loadFinished = ref(false);
let session: EditicsHostSession | undefined = undefined;
const frameReady = ref(false);
let readyTimeoutId: ReturnType<typeof setTimeout> | undefined;

const {
  contentInfo,
  readOnly,
  userInfo = undefined,
} = defineProps<{
  contentInfo: FileContentInfo;
  readOnly?: boolean;
  userInfo?: ClientInfo;
}>();

const emits = defineEmits<{
  (event: 'fileLoaded'): void;
  (event: 'fileError'): void;
  (event: 'onSaveStateChange', saveState: SaveState): void;
}>();

defineExpose({ save });

onMounted(async () => {
  // The document type is resolved from the file content type detected on this
  // side (see getEditicsDocumentType), so it is always known by the time
  // the open options are constructed.
  const documentType = getEditicsDocumentType(contentInfo.contentType);

  if (documentType === undefined) {
    error.value = EditorErrorTitle.UnsupportedFileType;
    await openIssueModal(EditorIssueStatus.UnsupportedFileType);
    return;
  }

  await loadEditor(documentType);
});

onUnmounted(() => {
  if (readyTimeoutId) {
    clearTimeout(readyTimeoutId);
  }
  if (session) {
    session.controller.abort();
    session = undefined;
  }
});

// Persists the original-format document bytes prepared by the offline host
// into the workspace file it was opened from. This is the workspace-access
// side of the offline editor: the iframe hosting the editor only gets to send
// us `save` requests (see services/onlyoffice.ts), never libparsec itself.
async function saveToWorkspace(workspaceHandle: WorkspaceHandle, data: Uint8Array): Promise<void> {
  if (!contentInfo) {
    throw new Error('missing content info');
  }
  const { path } = contentInfo;

  // Overwrite the file in place: opening with truncate keeps the same entry
  // (id, path, history), unlike the create-temporary-then-rename dance used
  // for new files.
  const fdResult = await openFile(workspaceHandle, path, { write: true, truncate: true });
  if (!fdResult.ok) {
    throw new Error(`failed to open the file for writing: ${JSON.stringify(fdResult.error)}`);
  }
  try {
    const writeResult = await writeFile(workspaceHandle, fdResult.value, 0, data);
    if (!writeResult.ok) {
      throw new Error(`failed to write the file: ${JSON.stringify(writeResult.error)}`);
    }
  } finally {
    await closeFile(workspaceHandle, fdResult.value);
  }
}

// Maps the save states reported by the OnlyOffice service onto the states
// displayed by the file handler topbar.
function mapSaveState(state: EditicsSaveState): SaveState {
  switch (state) {
    case EditicsSaveState.Unsaved:
      return SaveState.Unsaved;
    case EditicsSaveState.Saving:
      return SaveState.Saving;
    case EditicsSaveState.Saved:
      return SaveState.Saved;
    case EditicsSaveState.Error:
      return SaveState.Error;
  }
}

async function loadEditor(documentType: EditicsDocumentTypes): Promise<void> {
  const workspaceHandle = getWorkspaceHandle();

  if (!workspaceHandle) {
    window.nativeAPI.log('error', 'Cannot retrieve workspace handle');
    emits('fileError');
    return;
  }
  if (!editorFrame.value) {
    window.nativeAPI.log('error', 'Cannot get the iframe element');
    emits('fileError');
    return;
  }
  frameReady.value = false;
  if (session) {
    session.controller.abort();
    session = undefined;
  }

  const documentContent = await getFileContent(workspaceHandle, contentInfo.path, contentInfo.timestamp);
  if (!documentContent) {
    emits('fileError');
    return;
  }
  session = await openDocument(
    {
      documentName: contentInfo.fileName,
      documentExtension: contentInfo.extension,
      documentType,
      userName: userInfo ? userInfo.humanHandle.label : I18n.translate('UsersPage.anonymous'),
      userId: userInfo ? userInfo.userId : crypto.randomUUID(),
      mode: readOnly || contentInfo.timestamp ? 'view' : 'edit',
      locale: longLocaleCodeToShort(I18n.getLocale()),
    },
    documentContent,
    {
      onReady: (): void => {
        window.nativeAPI.log('info', 'OnlyOffice editor is ready and document loaded successfully');
        if (readyTimeoutId) {
          clearTimeout(readyTimeoutId);
          readyTimeoutId = undefined;
        }
        loadFinished.value = true;
        emits('fileLoaded');
      },
      // The editor iframe asks us to persist the document (it has no
      // workspace access itself): convert the OnlyOffice native serialization
      // back to the file's office format and write it to the workspace.
      onSave: async (data: Uint8Array): Promise<void> => {
        await saveToWorkspace(workspaceHandle, data);
      },
      onSaveStateChange: (state: EditicsSaveState): void => {
        emits('onSaveStateChange', mapSaveState(state));
      },
      onError: async (err: unknown): Promise<void> => {
        error.value = 'fileViewers.errors.titles.genericError';
        showErrorTips.value = true;

        if (err instanceof EditicsError) {
          window.nativeAPI.log('info', `Failed to load OnlyOffice: ${err}`);
          switch (err.code) {
            case EditicsErrorCodes.FrameLoadFailed:
            case EditicsErrorCodes.FrameNotLoaded:
              error.value = 'fileEditors.errors.titles.frameLoadFailed';
              break;
            case EditicsErrorCodes.EventError:
              window.nativeAPI.log('error', `Unhandled event error: ${err.details}`);
              break;
          }
        } else {
          window.nativeAPI.log('error', `Unhandled error: ${err}`);
        }
        emits('fileError');
        loadFinished.value = true;
      },
    },
    editorFrame.value,
  );
  frameReady.value = true;

  // Give the user the option to keep waiting rather than silently doing nothing if the
  // document takes too long to load. The e2e tests manage their own waits and skip the
  // timeout modal entirely (it would otherwise pop up while the editor assets are still
  // loading and intercept the test's clicks).
  if ((window as any).TESTING !== true) {
    readyTimeoutId = setTimeout(() => {
      if (!loadFinished.value) {
        openTimeoutModal();
      }
    }, READY_TIMEOUT_MS);
  }
}

async function openIssueModal(status: EditorIssueStatus, redirectAfterDismiss = true): Promise<MsModalResult> {
  // Safety check: only show modal if we're still on the file handler/editor route
  if (!currentRouteIs(Routes.FileHandler) || (currentRouteIs(Routes.FileHandler) && getFileHandlerMode() !== FileHandlerMode.Edit)) {
    window.nativeAPI.log('info', 'Skipping modal - user navigated away from editor');
    return MsModalResult.Cancel;
  }
  if (await modalController.getTop()) {
    window.nativeAPI.log('warn', 'A modal is already opened, skipping...');
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
  const WAIT_TIMEOUT = 15000;

  if (loadFinished.value) {
    return 'close';
  }
  const role = await openIssueModal(EditorIssueStatus.LoadingTimeout, false);

  // If user clicks primary button (close/dismiss)
  if (role === MsModalResult.Confirm) {
    if (loadFinished.value) {
      return 'close';
    } else {
      window.nativeAPI.log('info', `User chose to wait, ask them again in ${WAIT_TIMEOUT}ms`);
      setTimeout(() => {
        openTimeoutModal();
      }, WAIT_TIMEOUT);
      return 'wait';
    }
  } else if (role === MsModalResult.Cancel) {
    await routerGoBack();
    return 'close';
  }

  // Modal was dismissed (close button) - just close without navigating
  return 'close';
}

async function save(): Promise<boolean> {
  // Ask the editor to save now (its serialization goes through `onSave` →
  // saveToWorkspace above) and wait for the completion. Returns false when it
  // could not be saved, in which case the file handler asks the user whether
  // to discard the changes or stay (see FileHandler.vue checkSaved).
  if (!session) {
    return true;
  }
  return await session.save();
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
