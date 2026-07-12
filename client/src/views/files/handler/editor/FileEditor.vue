<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <!--
  Cryptpad editor is going to to be loaded in this Iframe.
  After that we will communicate back and forth with it through
  messages to pass it the document to open, then to receive the
  updated document every time Cryptpad decides we should save it.
  -->
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
import { ClientInfo, cryptpadSessionSaveAndSyncFile } from '@/parsec';
import { currentRouteIs, getFileHandlerMode, getWorkspaceHandle, routerGoBack, Routes } from '@/router';
import {
  CryptpadEditor,
  CryptpadError,
  CryptpadErrorCode,
  CryptpadOpenMode,
  CryptpadSession,
  derivePocSessionKey,
  derivePocViewOnlyKey,
  getCryptpadEditor,
  openDocument,
} from '@/services/cryptpad';
import { EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { longLocaleCodeToShort } from '@/services/translation';
import { EditorButtonAction, EditorErrorTitle, EditorIssueStatus, SaveState } from '@/views/files/handler/editor';
import EditorIssueModal from '@/views/files/handler/editor/EditorIssueModal.vue';
import { FileHandlerMode } from '@/views/files/handler/types';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import { IonButton, IonIcon, IonItem, IonList, IonText, modalController } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import { I18n, LogoIconGradient, MsImage, MsModalResult, MsSpinner } from 'megashark-lib';
import { DateTime } from 'luxon';
import { inject, onMounted, onUnmounted, Ref, ref, useTemplateRef } from 'vue';

const editorFrame = useTemplateRef<HTMLIFrameElement>('editorFrame');
const documentType = ref<CryptpadEditor>(CryptpadEditor.Unsupported);
const error = ref('');
const showErrorTips = ref(false);
const eventDistributor: Ref<EventDistributor> = inject(EventDistributorKey)!;
let eventCbId: null | string = null;
const loadFinished = ref(false);
let session: CryptpadSession | undefined = undefined;
const frameReady = ref(false);
let pendingSaveResolve: ((success: boolean) => void) | null = null;
// How long to wait, after the document is reported modified, before actually requesting a save.
// No save is requested at all while the document isn't modified (see `onHasUnsavedChanges` below):
// CryptPad's own autosave is always disabled, saving is entirely driven from here.
const SAVE_DEBOUNCE_MS = ((window as any).TESTING_EDITICS_SAVE_TIMEOUT as number | undefined) ?? 10_000;
// Zero jitter in test mode so tests are deterministic; up to 3 s in production so that
// collaborators' debounce timers fire at slightly different times, giving the
// ISAVE leader-election enough separation to elect a single writer without stalling.
const SAVE_DEBOUNCE_JITTER_MAX_MS = (window as any).TESTING_EDITICS_SAVE_TIMEOUT !== undefined ? 0 : 3_000;
let saveDebounceTimer: ReturnType<typeof setTimeout> | undefined;
// Mirrors CryptPad's own `onHasUnsavedChanges` signal so `save()` (called unconditionally by
// `FileHandler.vue` when leaving the editor) can skip the round-trip when there is nothing to
// persist, instead of always re-saving identical content.
let hasUnsavedChanges = false;

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

onMounted(async () => {
  documentType.value = getCryptpadEditor(contentInfo.contentType);

  if (documentType.value === CryptpadEditor.Unsupported) {
    error.value = EditorErrorTitle.UnsupportedFileType;
    await openIssueModal(EditorIssueStatus.UnsupportedFileType);
    return;
  }

  await loadEditor();

  eventCbId = await eventDistributor.value.registerCallback([Events.Online, Events.Offline], async (event: Events) => {
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
  clearSaveDebounceTimer();

  if (session) {
    session.controller.abort();
    session = undefined;
  }

  if (eventCbId) {
    eventDistributor.value.removeCallback(eventCbId);
  }
});

function clearSaveDebounceTimer(): void {
  if (saveDebounceTimer !== undefined) {
    clearTimeout(saveDebounceTimer);
    saveDebounceTimer = undefined;
  }
}

async function loadEditor(): Promise<void> {
  const workspaceHandle = getWorkspaceHandle();

  if (!workspaceHandle) {
    window.electronAPI.log('error', 'Cannot retrieve workspace handle');
    emits('fileError');
    return;
  }
  if (!editorFrame.value) {
    window.electronAPI.log('error', 'Cannot get the iframe element');
    emits('fileError');
    return;
  }
  frameReady.value = false;
  if (session) {
    session.controller.abort();
    session = undefined;
  }
  hasUnsavedChanges = false;
  clearSaveDebounceTimer();

  let isSaving = false;
  const content = await getFileContent(workspaceHandle, contentInfo.path, contentInfo.timestamp);
  if (!content) {
    emits('fileError');
    return;
  }
  // TODO: Key should be obtained from the Parsec server and contain actual cryptography
  // (real per-session read-only/read-write keys)! For now, derive a valid-shaped key from the
  // file id so collaborators opening the same file land on the same CryptPad channel. Readers
  // must use the corresponding *view* key, not the edit key itself (see `derivePocViewOnlyKey`).
  const editKey = await derivePocSessionKey(contentInfo.fileId);
  const key = readOnly ? await derivePocViewOnlyKey(editKey) : editKey;
  session = await openDocument(
    {
      documentContent: content,
      documentName: contentInfo.fileName,
      documentExtension: contentInfo.extension,
      cryptpadEditor: documentType.value as CryptpadEditor,
      key,
      // TODO: `userInfo` should be some kind of promise so that we always provide
      // the correct value (note `userInfo` can always be obtained since it doesn't
      // require sending the server a request)
      userName: userInfo ? userInfo.humanHandle.label : I18n.translate('UsersPage.anonymous'),
      userId: userInfo ? userInfo.userId : crypto.randomUUID(),
      mode: readOnly || contentInfo.timestamp ? CryptpadOpenMode.View : CryptpadOpenMode.Edit,
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
        const start = Date.now();
        try {
          if (readOnly) {
            return;
          }
          isSaving = true;
          emits('onSaveStateChange', SaveState.Saving);
          // Handle save logic here
          const arrayBuffer = await file.arrayBuffer();
          // TODO: `channelId` should be the actual Cryptpad channel identifier once the
          // server exposes real per-session channel management (see the `derivePocSessionKey`
          // TODO above). For now, derive it from the file id so every collaborator saving
          // the same file agrees on the same channel.
          const saveResult = await cryptpadSessionSaveAndSyncFile(
            workspaceHandle,
            contentInfo.fileId,
            contentInfo.fileId,
            DateTime.now(),
            new Uint8Array(arrayBuffer),
          );
          if (!saveResult.ok) {
            hasError = true;
            window.electronAPI.log('error', `Failed to save file: ${saveResult.error.tag} (${saveResult.error.error})`);
          }
        } catch (e: any) {
          window.electronAPI.log('error', `Failed to save file: ${e.toString()}`);
          hasError = true;
        } finally {
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
                // Resolve pending manual save promise
                if (pendingSaveResolve) {
                  pendingSaveResolve(!hasError);
                  pendingSaveResolve = null;
                }
              }
            },
            (window as any).TESTING === true ? 0 : Math.max(1000 - (end - start), 0),
          );
        }
      },
      onHasUnsavedChanges: (unsaved: boolean): void => {
        hasUnsavedChanges = unsaved;
        if (unsaved) {
          // The indicator must appear as soon as the document is modified, regardless of
          // when the actual save is going to happen.
          emits('onSaveStateChange', SaveState.Unsaved);
          if (!readOnly && saveDebounceTimer === undefined) {
            saveDebounceTimer = setTimeout(
              () => {
                saveDebounceTimer = undefined;
                save();
              },
              SAVE_DEBOUNCE_MS + Math.random() * SAVE_DEBOUNCE_JITTER_MAX_MS,
            );
          }
        } else {
          // Nothing left to save (e.g. right after loading/joining a session, or once a
          // save has caught up with the latest edits): don't let a stale timer trigger a
          // needless save.
          clearSaveDebounceTimer();
        }
      },
      onError: async (err: unknown): Promise<void> => {
        error.value = 'fileViewers.errors.titles.genericError';
        showErrorTips.value = true;

        if (err instanceof CryptpadError) {
          window.electronAPI.log('info', `Failed to load Cryptpad: ${err}`);
          switch (err.code) {
            case CryptpadErrorCode.OpenDocumentFailed:
            case CryptpadErrorCode.OpenDocumentInvalidConfig:
              error.value = 'fileEditors.errors.titles.openFailed';
              break;
            case CryptpadErrorCode.FrameLoadFailed:
            case CryptpadErrorCode.FrameNotLoaded:
              error.value = 'fileEditors.errors.titles.frameLoadFailed';
              break;
            case CryptpadErrorCode.EventError:
              if (err.details && err.details.toString() === 'ready-timeout') {
                error.value = '';
                await openTimeoutModal();
                // Don't process it as a normal error
                return;
              } else {
                window.electronAPI.log('error', `Unhandled event error: ${err.details}`);
              }
              break;
            case CryptpadErrorCode.NotAvailable:
              showErrorTips.value = false;
              error.value = 'fileEditors.errors.titles.cryptpadNotAvailable';
              break;
          }
        } else {
          window.electronAPI.log('error', `Unhandled error: ${err}`);
        }
        emits('fileError');
        loadFinished.value = true;
      },
    },
    editorFrame.value,
  );
  frameReady.value = true;
}

async function openIssueModal(status: EditorIssueStatus, redirectAfterDismiss = true): Promise<MsModalResult> {
  // Safety check: only show modal if we're still on the file handler/editor route
  if (!currentRouteIs(Routes.FileHandler) || (currentRouteIs(Routes.FileHandler) && getFileHandlerMode() !== FileHandlerMode.Edit)) {
    window.electronAPI.log('info', 'Skipping modal - user navigated away from editor');
    return MsModalResult.Cancel;
  }
  if (await modalController.getTop()) {
    window.electronAPI.log('warn', 'A modal is already opened, skipping...');
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
      window.electronAPI.log('info', `User chose to wait, ask them again in ${WAIT_TIMEOUT}ms`);
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
  clearSaveDebounceTimer();
  if (!session) {
    return false;
  }
  if (!hasUnsavedChanges) {
    return true;
  }
  // If a save is already pending, resolve it as failed before starting a new one
  if (pendingSaveResolve) {
    pendingSaveResolve(false);
    pendingSaveResolve = null;
  }
  return new Promise<boolean>((resolve) => {
    const SAVE_TIMEOUT_MS = 5000;
    pendingSaveResolve = resolve;
    session!.save();
    // Timeout in case save never completes
    setTimeout(() => {
      if (pendingSaveResolve === resolve) {
        pendingSaveResolve = null;
        resolve(false);
      }
    }, SAVE_TIMEOUT_MS);
  });
}

defineExpose({ save });
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
