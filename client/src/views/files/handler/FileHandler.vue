<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <div class="container">
        <ms-spinner
          class="file-handler"
          title="fileViewers.retrievingFileContent"
          v-show="!loaded"
        />
        <div
          v-if="loaded && contentInfo"
          @click.prevent="onClick"
          class="file-handler"
        >
          <!-- file-handler topbar -->
          <div class="file-handler-topbar">
            <!-- icon visible when menu is hidden -->
            <ms-image
              v-if="!isMobile() && isLargeDisplay && !isHeaderVisible()"
              slot="start"
              id="trigger-toggle-menu-button"
              :image="SidebarToggle"
              @click="isSidebarMenuVisible() ? hideSidebarMenu() : resetSidebarMenu()"
            />
            <div
              class="topbar-left-content"
              ref="backBlock"
              v-if="!isHeaderVisible()"
            >
              <header-back-button
                :short="true"
                class="file-handler-topbar__back-button"
              />
            </div>
            <ms-image
              :image="getFileIcon(contentInfo.fileName)"
              class="file-icon"
            />
            <div class="file-handler-topbar__title">
              <ion-text class="subtitles-normal">
                {{ contentInfo.fileName }}
              </ion-text>
              <ion-text
                class="subtitles-sm"
                v-if="atDateTime"
              >
                {{ $msTranslate(I18n.formatDate(atDateTime)) }}
              </ion-text>
            </div>
            <!-- Here we could put the file action buttons -->
            <div
              class="file-handler-topbar-buttons"
              v-if="isLargeDisplay"
            >
              <ion-button
                class="file-handler-topbar-buttons__item"
                id="file-handler-details"
                @click="showDetails"
                v-if="isDesktop()"
                :disabled="!handlerReadyRef"
              >
                <ion-icon
                  :icon="informationCircle"
                  class="item-icon"
                />
                {{ $msTranslate('fileViewers.details') }}
              </ion-button>
              <ion-button
                class="file-handler-topbar-buttons__item"
                id="file-handler-copy-link"
                @click="copyPath(contentInfo.path)"
                v-if="isWeb()"
                :disabled="!handlerReadyRef"
              >
                <ion-icon
                  :icon="link"
                  class="item-icon"
                />
                {{ $msTranslate('fileViewers.copyLink') }}
              </ion-button>
              <ion-button
                class="file-handler-topbar-buttons__item"
                id="file-handler-open-editor"
                @click="openEditor(contentInfo.path)"
                v-if="!atDateTime && handlerMode === FileHandlerMode.View && Env.isEditicsEnabled()"
              >
                <ion-icon
                  :icon="create"
                  class="item-icon"
                />
                {{ $msTranslate('fileViewers.openInEditor') }}
              </ion-button>
              <ion-button
                class="file-handler-topbar-buttons__item"
                @click="downloadFile"
                v-if="isWeb()"
                :disabled="!handlerReadyRef"
              >
                <ms-image
                  :image="DownloadIcon"
                  class="item-icon"
                />
                {{ $msTranslate('fileViewers.download') }}
              </ion-button>
              <ion-button
                class="file-handler-topbar-buttons__item"
                id="file-handler-open-with-system"
                @click="openWithSystem(contentInfo.path)"
                v-show="isDesktop() && !atDateTime"
                :disabled="!handlerReadyRef"
              >
                <ion-icon
                  :icon="open"
                  class="item-icon"
                />
                {{ $msTranslate('fileViewers.openWithDefault') }}
              </ion-button>
              <ion-button
                class="file-handler-topbar-buttons__item toggle-menu"
                @click="toggleMainHeader"
                :class="{ 'header-visible': isHeaderVisible() }"
              >
                <ion-icon :icon="isHeaderVisible() ? chevronUp : chevronDown" />
                <span v-if="windowWidth > WindowSizeBreakpoints.MD">
                  {{ $msTranslate(isHeaderVisible() ? 'fileViewers.hideMenu' : 'fileViewers.showMenu') }}
                </span>
              </ion-button>
            </div>
            <ion-button
              class="file-handler-topbar-buttons__item action-menu"
              @click="openSmallDisplayActionMenu"
              v-else
            >
              <ion-icon :icon="ellipsisHorizontal" />
            </ion-button>
          </div>

          <!-- file-handler sub-component -->
          <component
            :is="handlerComponent"
            :content-info="contentInfo"
            :file-info="detectedFileType"
            @file-loaded="handlerReadyRef = true"
          />
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  closeFile,
  entryStat,
  EntryStatFile,
  FsPath,
  openFile,
  Path,
  readFile,
  getSystemPath,
  isDesktop,
  isWeb,
  WorkspaceHistory,
  DEFAULT_READ_SIZE,
  WorkspaceHandle,
  EntryName,
  getWorkspaceInfo,
  isMobile,
  WorkspaceHistoryEntryStatFile,
  ClientInfo,
  getClientInfo,
} from '@/parsec';
import HeaderBackButton from '@/components/header/HeaderBackButton.vue';
import {
  Base64,
  MsSpinner,
  MsImage,
  I18n,
  DownloadIcon,
  askQuestion,
  Answer,
  MsModalResult,
  useWindowSize,
  SidebarToggle,
  WindowSizeBreakpoints,
} from 'megashark-lib';
import { ref, Ref, inject, onMounted, onUnmounted, type Component, shallowRef } from 'vue';
import { IonPage, IonContent, IonButton, IonText, IonIcon, modalController } from '@ionic/vue';
import { link, informationCircle, open, chevronUp, chevronDown, ellipsisHorizontal, create } from 'ionicons/icons';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import {
  currentRouteIs,
  getCurrentRouteQuery,
  getDocumentPath,
  getFileHandlerMode,
  getWorkspaceHandle,
  navigateTo,
  Routes,
  watchRoute,
} from '@/router';
import { DetectedFileType } from '@/common/fileTypes';
import SmallDisplayViewerActionMenu from '@/views/files/handler/SmallDisplayViewerActionMenu.vue';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import { DateTime } from 'luxon';
import { getFileIcon } from '@/common/file';
import { copyPathLinkToClipboard } from '@/components/files';
import { downloadEntry, FileDetailsModal, FileHandlerAction, openDownloadConfirmationModal } from '@/views/files';
import useHeaderControl from '@/services/headerControl';
import { Env } from '@/services/environment';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { FileOperationManager, FileOperationManagerKey } from '@/services/fileOperationManager';
import FileEditor from '@/views/files/handler/editor/FileEditor.vue';
import FileViewer from '@/views/files/handler/viewer/FileViewer.vue';
import useSidebarMenu from '@/services/sidebarMenu';
import { openPath } from '@/services/fileOpener';
import { FileHandlerMode } from '@/views/files/handler';

const { isLargeDisplay, windowWidth } = useWindowSize();
const storageManager: StorageManager = inject(StorageManagerKey)!;
const fileOperationManager: FileOperationManager = inject(FileOperationManagerKey)!;
const informationManager: InformationManager = inject(InformationManagerKey)!;
const contentInfo: Ref<FileContentInfo | undefined> = ref(undefined);
const detectedFileType = ref<DetectedFileType | null>(null);
const loaded = ref(false);
const atDateTime: Ref<DateTime | undefined> = ref(undefined);
const { isHeaderVisible, toggleHeader: toggleMainHeader, showHeader, hideHeader } = useHeaderControl();
const { isVisible: isSidebarMenuVisible, reset: resetSidebarMenu, hide: hideSidebarMenu, show: showSidebarMenu } = useSidebarMenu();
const handlerReadyRef = ref(false);
const handlerComponent: Ref<Component | null> = shallowRef(null);
const handlerMode = ref<FileHandlerMode | undefined>(undefined);
const sidebarMenuVisibleOnMounted = ref(false);
const userInfo: Ref<ClientInfo | null> = ref(null);

const cancelRouteWatch = watchRoute(async () => {
  if (!currentRouteIs(Routes.FileHandler)) {
    return;
  }

  const query = getCurrentRouteQuery();
  const timestamp = Number.isNaN(Number(query.timestamp)) ? undefined : Number(query.timestamp);

  // Same file, no need to reload
  if (contentInfo.value && contentInfo.value.path === getDocumentPath() && atDateTime.value?.toMillis() === timestamp) {
    const fileHandlerMode = getFileHandlerMode();
    if (fileHandlerMode === handlerMode.value) {
      return;
    }
    handlerMode.value = getFileHandlerMode();
  }

  await loadFile();
});

async function _getFileInfoAt(
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
  at: DateTime,
  fileInfo: DetectedFileType,
  fileName: EntryName,
): Promise<FileContentInfo | undefined> {
  const infoResult = await getWorkspaceInfo(workspaceHandle);
  if (!infoResult.ok) {
    return;
  }
  const history = new WorkspaceHistory(infoResult.value.id);
  try {
    await history.start(at);
    const statsResult = await history.entryStat(path);

    if (!statsResult.ok || !statsResult.value.isFile()) {
      return;
    }
    const openResult = await history.openFile(path);
    if (!openResult.ok) {
      return;
    }
    const info: FileContentInfo = {
      data: new Uint8Array(Number((statsResult.value as WorkspaceHistoryEntryStatFile).size)),
      extension: fileInfo.extension,
      contentType: fileInfo.type,
      fileName: fileName,
      path: path,
      fileId: statsResult.value.id,
    };
    const fd = openResult.value;
    try {
      let loop = true;
      let offset = 0;
      while (loop) {
        const readResult = await history.readFile(openResult.value, offset, DEFAULT_READ_SIZE);
        if (!readResult.ok) {
          throw Error(JSON.stringify(readResult.error));
        }
        const buffer = new Uint8Array(readResult.value);
        info.data.set(buffer, offset);
        if (readResult.value.byteLength < DEFAULT_READ_SIZE) {
          loop = false;
        }
        offset += readResult.value.byteLength;
      }
      return info;
    } catch (e: any) {
      window.electronAPI.log('error', `Can't view the file: ${e.toString()}`);
    } finally {
      await history.closeFile(fd);
    }
  } catch (e: any) {
    window.electronAPI.log('error', `Can't view the file: ${e.toString()}`);
  } finally {
    await history.stop();
  }
}

async function _getFileInfo(
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
  fileInfo: DetectedFileType,
  fileName: EntryName,
): Promise<FileContentInfo | undefined> {
  const statsResult = await entryStat(workspaceHandle, path);
  if (!statsResult.ok || !statsResult.value.isFile()) {
    return;
  }

  const openResult = await openFile(workspaceHandle, path, { read: true });
  if (!openResult.ok) {
    return;
  }
  const info: FileContentInfo = {
    data: new Uint8Array(Number((statsResult.value as EntryStatFile).size)),
    extension: fileInfo.extension,
    contentType: fileInfo.type,
    fileName: fileName,
    path: path,
    fileId: statsResult.value.id,
  };

  const fd = openResult.value;
  try {
    let loop = true;
    let offset = 0;
    while (loop) {
      const readResult = await readFile(workspaceHandle, openResult.value, offset, DEFAULT_READ_SIZE);
      if (!readResult.ok) {
        throw Error(JSON.stringify(readResult.error));
      }
      const buffer = new Uint8Array(readResult.value);
      info.data.set(buffer, offset);
      if (readResult.value.byteLength < DEFAULT_READ_SIZE) {
        loop = false;
      }
      offset += readResult.value.byteLength;
    }
    return info;
  } catch (e: any) {
    window.electronAPI.log('error', `Can't view the file: ${e.toString()}`);
  } finally {
    await closeFile(workspaceHandle, fd);
  }
}

async function loadFile(): Promise<void> {
  loaded.value = false;
  contentInfo.value = undefined;
  detectedFileType.value = null;
  atDateTime.value = undefined;
  handlerComponent.value = null;
  handlerReadyRef.value = false;
  const workspaceHandle = getWorkspaceHandle();
  if (!workspaceHandle) {
    window.electronAPI.log('error', 'Failed to retrieve workspace handle');
    return;
  }
  const path = getDocumentPath();
  if (!path) {
    window.electronAPI.log('error', 'Failed to retrieve document path');
  }
  const fileName = await Path.filename(path);
  if (!fileName) {
    window.electronAPI.log('error', 'Failed to retrieve the file name');
    return;
  }

  const timestamp = Number(getCurrentRouteQuery().timestamp);
  if (!Number.isNaN(timestamp)) {
    atDateTime.value = DateTime.fromMillis(timestamp);
  }

  const fileInfoSerialized = getCurrentRouteQuery().fileTypeInfo;
  if (!fileInfoSerialized) {
    window.electronAPI.log('error', 'Failed to retrieve file type info');
    return;
  }
  const fileInfo: DetectedFileType = Base64.toObject(fileInfoSerialized) as DetectedFileType;
  detectedFileType.value = fileInfo;

  const info = timestamp
    ? await _getFileInfoAt(workspaceHandle, path, DateTime.fromMillis(timestamp), fileInfo, fileName)
    : await _getFileInfo(workspaceHandle, path, fileInfo, fileName);

  if (!info) {
    contentInfo.value = undefined;
    handlerComponent.value = null;
    handlerReadyRef.value = false;
    informationManager.present(
      new Information({
        message: 'fileViewers.genericError',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    if (!timestamp) {
      await openWithSystem(path);
    }
    await navigateTo(Routes.Documents, { query: { workspaceHandle: workspaceHandle, documentPath: await Path.parent(path) } });
    return;
  }

  contentInfo.value = info;
  if (timestamp) {
    atDateTime.value = DateTime.fromMillis(timestamp);
  }

  // Load the appropriate component after file content is ready
  loadComponent();

  loaded.value = true;
}

function loadComponent(): void {
  // Set the handler component based on the file type
  switch (handlerMode.value) {
    case FileHandlerMode.Edit:
      handlerComponent.value = FileEditor;
      break;
    case FileHandlerMode.View:
      handlerComponent.value = FileViewer;
      break;
    default:
      handlerComponent.value = null;
  }
  if (!handlerComponent.value) {
    handlerReadyRef.value = false;
    throw new Error(`No component for file with extension '${detectedFileType.value!.extension}'`);
  }
}
onMounted(async () => {
  handlerMode.value = getFileHandlerMode();
  await loadFile();
  // Set header hidden by default when entering handler
  hideHeader();
  if (isSidebarMenuVisible()) {
    hideSidebarMenu();
    sidebarMenuVisibleOnMounted.value = true;
  }

  const clientInfoResult = await getClientInfo();
  if (clientInfoResult.ok) {
    userInfo.value = clientInfoResult.value;
  } else {
    window.electronAPI.log('error', `Failed to retrieve user info ${JSON.stringify(clientInfoResult.error)}`);
  }
});

onUnmounted(async () => {
  cancelRouteWatch();
  // Ensure header is visible when leaving handler
  showHeader();
  if (sidebarMenuVisibleOnMounted.value) {
    showSidebarMenu();
  }
});

async function openWithSystem(path: FsPath): Promise<boolean> {
  if (!isDesktop()) {
    return false;
  }

  const workspaceHandle = getWorkspaceHandle();
  if (!workspaceHandle) {
    window.electronAPI.log('error', 'Failed to retrieve workspace handle');
    return false;
  }

  const fileName = (await Path.filename(path)) ?? '';
  const result = await getSystemPath(workspaceHandle, path);

  if (!result.ok) {
    await informationManager.present(
      new Information({
        message: fileName ? { key: 'FoldersPage.open.fileFailed', data: { name: fileName } } : 'FoldersPage.open.fileFailedGeneric',
        level: InformationLevel.Error,
      }),
      PresentationMode.Modal,
    );
    return false;
  } else {
    window.electronAPI.openFile(result.value);
    return true;
  }
}

async function copyPath(path: FsPath): Promise<void> {
  const workspaceHandle = getWorkspaceHandle();
  if (!workspaceHandle) {
    window.electronAPI.log('error', 'Failed to retrieve workspace handle');
    return;
  }
  copyPathLinkToClipboard(path, workspaceHandle, informationManager);
}

async function showDetails(): Promise<void> {
  const workspaceHandle = getWorkspaceHandle();
  if (workspaceHandle) {
    const entry = await entryStat(workspaceHandle, getDocumentPath());
    if (!entry.ok) {
      await informationManager.present(
        new Information({
          message: 'fileViewers.errors.statsFailed',
          level: InformationLevel.Error,
        }),
        PresentationMode.Modal,
      );
      return;
    }
    const modal = await modalController.create({
      component: FileDetailsModal,
      cssClass: 'file-details-modal',
      componentProps: {
        entry: entry.value,
        workspaceHandle: workspaceHandle,
        ownProfile: userInfo.value ? userInfo.value.currentProfile : undefined,
      },
    });
    await modal.present();
    await modal.onWillDismiss();
  }
}

async function openEditor(path: FsPath): Promise<void> {
  const workspaceHandle = getWorkspaceHandle();
  if (workspaceHandle) {
    await openPath(workspaceHandle, path, informationManager, fileOperationManager, { useEditor: true });
  }
}

async function onClick(event: MouseEvent): Promise<void> {
  event.preventDefault();
  if (!event.target) {
    return;
  }
  const target = event.target as Element;
  let linkTag!: HTMLAnchorElement;
  if (target.tagName === 'A') {
    linkTag = target as HTMLAnchorElement;
  } else if (target.tagName === 'SPAN' && target.parentElement && target.parentElement.tagName === 'A') {
    linkTag = target.parentElement as HTMLAnchorElement;
  } else {
    return;
  }
  // `.href` get resolved, `getAttribute('href') is what actually in the href`
  const link = linkTag.href;
  const href = linkTag.getAttribute('href') ?? '';
  // We exclude invalid links or anchors
  if (!link || href.startsWith('#') || !URL.canParse(link)) {
    return;
  }
  const answer = await askQuestion('fileViewers.openLink.title', 'fileViewers.openLink.question', { yesText: 'fileViewers.openLink.yes' });
  if (answer === Answer.Yes) {
    Env.Links.openUrl(link);
  }
}

async function downloadFile(): Promise<void> {
  const result = await openDownloadConfirmationModal(storageManager);
  if (result === MsModalResult.Cancel) {
    return;
  }

  const workspaceHandle = getWorkspaceHandle();
  if (!workspaceHandle) {
    window.electronAPI.log('error', 'Failed to retrieve workspace handle');
    return;
  }
  const workspaceInfoResult = await getWorkspaceInfo(workspaceHandle);
  if (!workspaceInfoResult.ok) {
    window.electronAPI.log(
      'error',
      `Failed to retrieve workspace info: ${workspaceInfoResult.error.tag} (${workspaceInfoResult.error.error})`,
    );
    return;
  }

  if (!contentInfo.value) {
    window.electronAPI.log('error', 'No content info when trying to download a file');
    return;
  }

  await downloadEntry({
    name: contentInfo.value.fileName,
    workspaceHandle: workspaceHandle,
    workspaceId: workspaceInfoResult.value.id,
    path: contentInfo.value.path,
    informationManager: informationManager,
    fileOperationManager: fileOperationManager,
  });
}

async function openSmallDisplayActionMenu(): Promise<void> {
  const modal = await modalController.create({
    component: SmallDisplayViewerActionMenu,
    cssClass: 'viewer-action-menu-modal',
    showBackdrop: true,
    breakpoints: [0, 0.5, 1],
    expandToScroll: false,
    initialBreakpoint: 0.5,
    componentProps: {},
  });
  await modal.present();
  const { data } = await modal.onDidDismiss();
  if (data !== undefined) {
    switch (data.action) {
      case FileHandlerAction.Details:
        await showDetails();
        break;
      case FileHandlerAction.CopyPath:
        if (contentInfo.value) {
          await copyPath(contentInfo.value.path);
        }
        break;
      case FileHandlerAction.Download:
        await downloadFile();
        break;
      case FileHandlerAction.OpenWithSystem:
        if (contentInfo.value) {
          await openWithSystem(contentInfo.value.path);
        }
        break;
      default:
        console.warn('No ViewerAction match found');
    }
  }
}
</script>

<style scoped lang="scss">
.container {
  height: 100%;
  background-color: var(--parsec-color-light-secondary-premiere);

  .file-handler {
    display: flex;
    flex-direction: column;
    height: 100%;
    justify-content: center;

    &-topbar {
      display: flex;
      align-items: center;
      gap: 1rem;
      padding: 1rem 2rem;
      border-bottom: 1px solid var(--parsec-color-light-secondary-disabled);
      background: var(--parsec-color-light-secondary-white);

      #trigger-toggle-menu-button {
        flex-shrink: 0;
        --fill-color: var(--parsec-color-light-secondary-grey);
        padding: 0.625rem;
        border-radius: var(--parsec-radius-12);
        cursor: pointer;
        &:hover {
          background: var(--parsec-color-light-secondary-premiere);
          --fill-color: var(--parsec-color-light-secondary-hard-grey);
        }
      }

      @include ms.responsive-breakpoint('sm') {
        padding: 0.75rem 1rem;
      }

      .file-icon {
        width: 2rem;
        height: 2rem;
        min-width: 2rem;
        min-height: 2rem;
        margin-left: 0.5rem;

        @include ms.responsive-breakpoint('sm') {
          width: 1.75rem;
          height: 1.75rem;
          min-width: 1.75rem;
          min-height: 1.75rem;
        }
      }

      &__title {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        overflow: hidden;

        ion-text {
          color: var(--parsec-color-light-secondary-text);
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
        }

        .subtitles-sm {
          color: var(--parsec-color-light-secondary-grey);
        }
      }

      &-buttons {
        margin-left: auto;
        display: flex;
        gap: 0.5rem;
        flex-shrink: 0;

        &:hover {
          border-color: var(--parsec-color-light-primary-100);
        }

        &__item:not(.toggle-menu) {
          color: var(--parsec-color-light-secondary-text);
          border-radius: var(--parsec-radius-8);
          transition: all 150ms linear;

          &::part(native) {
            --background: none;
            --background-hover: none;
            padding: 0.625rem 1rem;

            @include ms.responsive-breakpoint('sm') {
              padding: 0.5rem;
            }
          }

          &:hover {
            background: var(--parsec-color-light-secondary-medium);
            color: var(--parsec-color-light-secondary-text);
          }

          .item-icon {
            width: 1rem;
            font-size: 1rem;
            margin-right: 0.5rem;
            --fill-color: var(--parsec-color-light-secondary-text);

            @include ms.responsive-breakpoint('sm') {
              min-width: 1.125rem;
              width: 1.125rem;
              font-size: 1.125rem;
              margin-right: 0;
            }
          }
        }

        .action-menu {
          font-size: 1.125rem;
          margin-right: 0;

          @include ms.responsive-breakpoint('sm') {
            font-size: 1.25rem;
          }
        }

        .toggle-menu {
          margin-right: 0.5rem;
          position: relative;
          margin-left: 0.5rem;
          --background: none;
          color: var(--parsec-color-light-secondary-hard-grey);

          &::part(native) {
            --background: none;
            --background-hover: none;
          }

          ion-icon {
            font-size: 1rem;
            margin-right: 0.5rem;
            color: var(--parsec-color-light-secondary-hard-grey);

            @include ms.responsive-breakpoint('md') {
              margin-right: 0;
            }
          }

          @include ms.responsive-breakpoint('sm') {
            font-size: 1.125rem;
            margin-right: 0;
          }

          &:hover {
            color: var(--parsec-color-light-secondary-soft-text);

            ion-icon {
              color: var(--parsec-color-light-secondary-soft-text);
            }
          }

          &::before {
            content: '';
            position: absolute;
            top: 50%;
            left: -0.5rem;
            transform: translateY(-50%);
            width: 1px;
            height: 1.5rem;
            background: var(--parsec-color-light-secondary-disabled);
          }
        }
      }
    }
  }
}
</style>
