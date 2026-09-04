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
import {
  ClientInfo,
  entryStat,
  EntryStatTag,
  FsPath,
  getCurrentServerAddr,
  getUserInfoFromDeviceID,
  getWorkspaceInfo,
  parseParsecAddr,
  WorkspaceHandle,
} from '@/parsec';
import { currentRouteIs, getFileHandlerMode, getWorkspaceHandle, routerGoBack, Routes } from '@/router';
import {
  getOnlyOfficeDocumentType,
  OnlyOfficeDocumentType,
  OnlyOfficeError,
  OnlyOfficeErrorCodes,
  OnlyOfficeOpenModes,
  OnlyOfficeSession,
  openDocument,
  OpenDocumentOptions,
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

// Resolve the document's vlob version for the editics session (RFC 1030 §1.2).
//
// A freshly-created file has `baseVersion: 0` and `needSync: true` until the
// workspace has manifested it as a new vlob on the server (version 1). If two
// tabs open the same document around that moment, the first can capture
// `baseVersion: 0` (local-only) while the second reads `baseVersion: 1`
// (synced) -- the editics server would then reject the second as
// "modified outside the session" since its version is ahead of the session's
// `initial_version`.
//
// To make the collaborative join robust, wait until the file is synced
// (`needSync === false`) before reading `baseVersion`, polling for a bounded
// time. If it never syncs within the deadline, fall back to the last seen
// `baseVersion` (the session may still be created; the join-retry path handles
// the rest). This keeps both tabs on the same committed vlob version.
const VLOB_VERSION_SYNC_TIMEOUT_MS = 5000;
const VLOB_VERSION_SYNC_POLL_MS = 100;

async function resolveVlobVersion(
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
  initialStat: Awaited<ReturnType<typeof entryStat>>,
): Promise<number> {
  let stat = initialStat;
  const deadline = Date.now() + VLOB_VERSION_SYNC_TIMEOUT_MS;
  while (stat.ok && stat.value.tag === EntryStatTag.File && stat.value.needSync) {
    if (Date.now() >= deadline) {
      break;
    }
    await new Promise((resolve) => setTimeout(resolve, VLOB_VERSION_SYNC_POLL_MS));
    stat = await entryStat(workspaceHandle, path);
  }
  return stat.ok && stat.value.tag === EntryStatTag.File ? stat.value.baseVersion : 1;
}

async function buildEditicsConfig(workspaceHandle: WorkspaceHandle): Promise<OpenDocumentOptions['editics'] | undefined> {
  // Build the editics collaborative session config (RFC 1030) so the host page
  // connects to the Parsec server's SSE + RPC routes. We resolve:
  //   - baseUrl:        the server's HTTP origin (from the device's server addr).
  //   - organizationId: from the client info.
  //   - workspaceId:    the workspace (realm) VlobID.
  //   - vlobId:         the document's VlobID (contentInfo.fileId).
  //   - deviceId:       the client's DeviceID (hyphenated UUID string).
  //   - vlobVersion:    the file's current vlob version (baseVersion from stat).
  //   - editorType:     0=Word,1=Spreadsheet,2=Presentation,3=Visio.
  // The server is not trusted for user names: `resolveUserName` resolves a
  // DeviceID hex to a display name through libparsec (RFC §3.3 / todo §2).
  try {
    const info = userInfo;
    if (!info) {
      return undefined;
    }
    const serverResult = await getCurrentServerAddr();
    if (!serverResult.ok) {
      return undefined;
    }
    // The parsec server addr uses the `parsec3://` scheme (with a `no_ssl=true`
    // query when the server runs plain HTTP); the editics HTTP routes live at
    // the same host. Parse it properly to get the hostname/port/ssl flags
    // instead of string-mangling the scheme (which breaks on `?no_ssl=true`).
    const parsedAddrResult = await parseParsecAddr(serverResult.value);
    if (!parsedAddrResult.ok) {
      return undefined;
    }
    const parsed = parsedAddrResult.value;
    const scheme = parsed.useSsl ? 'https' : 'http';
    const baseUrl = parsed.isDefaultPort ? `${scheme}://${parsed.hostname}` : `${scheme}://${parsed.hostname}:${parsed.port}`;
    const wsInfoResult = await getWorkspaceInfo(workspaceHandle);
    if (!wsInfoResult.ok) {
      return undefined;
    }
    const statResult = await entryStat(workspaceHandle, contentInfo.path);
    const vlobVersion = await resolveVlobVersion(workspaceHandle, contentInfo.path, statResult);
    const editorType =
      documentType.value === OnlyOfficeDocumentType.Word
        ? 0
        : documentType.value === OnlyOfficeDocumentType.Cell
          ? 1
          : documentType.value === OnlyOfficeDocumentType.Slide
            ? 2
            : 3;
    return {
      baseUrl,
      organizationId: info.organizationId,
      workspaceId: wsInfoResult.value.id,
      vlobId: contentInfo.fileId,
      deviceId: info.deviceId,
      vlobVersion,
      editorType,
      resolveUserName: async (deviceId: string): Promise<string | undefined> => {
        const result = await getUserInfoFromDeviceID(deviceId);
        if (result.ok) {
          return result.value.humanHandle.label;
        }
        return undefined;
      },
      resolveUserId: async (deviceId: string): Promise<string | undefined> => {
        const result = await getUserInfoFromDeviceID(deviceId);
        if (result.ok) {
          return result.value.id;
        }
        return undefined;
      },
    };
  } catch (e) {
    window.nativeAPI.log('warn', `Failed to build editics config: ${String(e)}`);
    return undefined;
  }
}

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
  // POC: identify the document independently of the workspace handle (a per-session
  // local resource id, not shared across users/tabs). The editics session is
  // keyed on the server by the file's own VlobID (`contentInfo.fileId`, RFC
  // §1.3), not its path: a path can be reused (delete + recreate, rename)
  // across genuinely different files, so keying on path would let a brand-new
  // file silently resume a stale collaboration session (locks, accumulated
  // changes) from a past, unrelated file that happened to share the same name.
  const documentId = contentInfo.fileId;
  const editics = await buildEditicsConfig(workspaceHandle);
  session = await openDocument(
    {
      documentContent: content,
      documentName: contentInfo.fileName,
      documentExtension: contentInfo.extension,
      documentType: documentType.value,
      documentId,
      key: crypto.randomUUID(),
      userName: userInfo ? userInfo.humanHandle.label : I18n.translate('UsersPage.anonymous'),
      userId: userInfo ? userInfo.userId : crypto.randomUUID(),
      mode: readOnly || contentInfo.timestamp ? OnlyOfficeOpenModes.View : OnlyOfficeOpenModes.Edit,
      locale: longLocaleCodeToShort(I18n.getLocale()),
      editics,
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
