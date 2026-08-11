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
import { ClientInfo } from '@/parsec';
import { currentRouteIs, getFileHandlerMode, getWorkspaceHandle, routerGoBack, Routes } from '@/router';
import {
  getOnlyOfficeDocumentType,
  OnlyOfficeDocumentType,
  OnlyOfficeError,
  OnlyOfficeErrorCodes,
  OnlyOfficeOpenModes,
  OnlyOfficeSession,
  openDocument,
} from '@/services/onlyoffice';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { longLocaleCodeToShort } from '@/services/translation';
import { EditorButtonAction, EditorErrorTitle, EditorIssueStatus, SaveState } from '@/views/files/handler/editor';
import EditorIssueModal from '@/views/files/handler/editor/EditorIssueModal.vue';
import { FileHandlerMode } from '@/views/files/handler/types';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import { IonButton, IonIcon, IonItem, IonList, IonText, modalController } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import { I18n, LogoIconGradient, MsImage, MsModalResult, MsSpinner } from 'megashark-lib';
import { onMounted, onUnmounted, ref, useTemplateRef } from 'vue';

// Time to wait for the document to be fully loaded (fonts, dictionaries, etc.) before offering
// the user the option to keep waiting or give up, see openTimeoutModal().
const READY_TIMEOUT_MS = 15000;

const editorFrame = useTemplateRef<HTMLIFrameElement>('editorFrame');
const documentType = ref<OnlyOfficeDocumentType>(OnlyOfficeDocumentType.Unsupported);
const error = ref('');
const showErrorTips = ref(false);
const loadFinished = ref(false);
let session: OnlyOfficeSession | undefined = undefined;
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

onMounted(async () => {
  documentType.value = getOnlyOfficeDocumentType(contentInfo.contentType);

  if (documentType.value === OnlyOfficeDocumentType.Unsupported) {
    error.value = EditorErrorTitle.UnsupportedFileType;
    await openIssueModal(EditorIssueStatus.UnsupportedFileType);
    return;
  }

  await loadEditor();
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

async function loadEditor(): Promise<void> {
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

  const content = await getFileContent(workspaceHandle, contentInfo.path, contentInfo.timestamp);
  if (!content) {
    emits('fileError');
    return;
  }
  session = await openDocument(
    {
      documentContent: content,
      documentName: contentInfo.fileName,
      documentType: documentType.value,
      key: crypto.randomUUID(),
      userName: userInfo ? userInfo.humanHandle.label : I18n.translate('UsersPage.anonymous'),
      userId: userInfo ? userInfo.userId : crypto.randomUUID(),
      mode: readOnly || contentInfo.timestamp ? OnlyOfficeOpenModes.View : OnlyOfficeOpenModes.Edit,
      locale: longLocaleCodeToShort(I18n.getLocale()),
    },
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
      onError: async (err: unknown): Promise<void> => {
        error.value = 'fileViewers.errors.titles.genericError';
        showErrorTips.value = true;

        if (err instanceof OnlyOfficeError) {
          window.nativeAPI.log('info', `Failed to load OnlyOffice: ${err}`);
          switch (err.code) {
            case OnlyOfficeErrorCodes.FrameLoadFailed:
            case OnlyOfficeErrorCodes.FrameNotLoaded:
              error.value = 'fileEditors.errors.titles.frameLoadFailed';
              break;
            case OnlyOfficeErrorCodes.EventError:
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

  // OnlyOffice's own assets (sdkjs, fonts, dictionaries) can take a while to load on first use,
  // give the user the option to keep waiting rather than silently doing nothing.
  readyTimeoutId = setTimeout(
    () => {
      if (!loadFinished.value) {
        openTimeoutModal();
      }
    },
    (window as any).TESTING === true ? 500 : READY_TIMEOUT_MS,
  );
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
  // POC: saving back to Parsec isn't implemented yet, see client/src/services/onlyoffice.ts.
  return true;
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
