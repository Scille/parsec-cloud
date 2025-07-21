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
              <ion-text class="title-h3">
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
            <ion-buttons class="file-handler-topbar-buttons">
              <ion-button
                class="file-handler-topbar-buttons__item"
                @click="showDetails"
                v-if="isDesktop()"
                :disabled="!handlerReadyRef"
              >
                <ion-icon :icon="informationCircle" />
                {{ $msTranslate('fileViewers.details') }}
              </ion-button>
              <ion-button
                class="file-handler-topbar-buttons__item"
                @click="copyPath(contentInfo.path)"
                v-if="isWeb()"
                :disabled="!handlerReadyRef"
              >
                <ion-icon :icon="link" />
                {{ $msTranslate('fileViewers.copyLink') }}
              </ion-button>
              <ion-button
                class="file-handler-topbar-buttons__item"
                @click="downloadFile"
                v-if="isWeb()"
                :disabled="!handlerReadyRef"
              >
                <ms-image
                  :image="DownloadIcon"
                  class="download-icon"
                />
                {{ $msTranslate('fileViewers.download') }}
              </ion-button>
              <ion-button
                class="file-handler-topbar-buttons__item"
                @click="openWithSystem(contentInfo.path)"
                v-show="isDesktop() && !atDateTime"
                :disabled="!handlerReadyRef"
              >
                <ion-icon :icon="open" />
                {{ $msTranslate('fileViewers.openWithDefault') }}
              </ion-button>
              <ion-button
                class="file-handler-topbar-buttons__item toggle-menu"
                @click="toggleMainHeader"
                :class="{ 'header-visible': isHeaderVisible() }"
              >
                {{ $msTranslate(isHeaderVisible() ? 'fileViewers.hideMenu' : 'fileViewers.showMenu') }}
                <ion-icon :icon="isHeaderVisible() ? chevronUp : chevronDown" />
              </ion-button>
            </ion-buttons>
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
} from '@/parsec';
import HeaderBackButton from '@/components/header/HeaderBackButton.vue';
import { IonPage, IonContent, IonButton, IonText, IonIcon, IonButtons, modalController } from '@ionic/vue';
import { link, informationCircle, open, chevronDown, chevronUp } from 'ionicons/icons';
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
} from 'megashark-lib';
import { ref, Ref, inject, onMounted, onUnmounted, type Component, shallowRef } from 'vue';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import {
  currentRouteIs,
  getCurrentRouteParams,
  getCurrentRouteQuery,
  getDocumentPath,
  getWorkspaceHandle,
  navigateTo,
  Routes,
  watchRoute,
} from '@/router';
import { DetectedFileType } from '@/common/fileTypes';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import { Env } from '@/services/environment';
import { DateTime } from 'luxon';
import useSidebarMenu from '@/services/sidebarMenu';
import { getFileIcon } from '@/common/file';
import { copyPathLinkToClipboard } from '@/components/files';
import { askDownloadConfirmation, downloadEntry, FileDetailsModal } from '@/views/files';
import useHeaderControl from '@/services/headerControl';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { FileOperationManager, FileOperationManagerKey } from '@/services/fileOperationManager';
import FileEditor from '@/views/files/handler/editor/FileEditor.vue';
import FileViewer from '@/views/files/handler/viewer/FileViewer.vue';
import { FileHandlerMode } from '@/views/files/handler';

const storageManager: StorageManager = inject(StorageManagerKey)!;
const fileOperationManager: FileOperationManager = inject(FileOperationManagerKey)!;
const { isLargeDisplay } = useWindowSize();
const informationManager: InformationManager = inject(InformationManagerKey)!;
const contentInfo: Ref<FileContentInfo | undefined> = ref(undefined);
const detectedFileType = ref<DetectedFileType | null>(null);
const loaded = ref(false);
const atDateTime: Ref<DateTime | undefined> = ref(undefined);
const { isHeaderVisible, toggleHeader: toggleMainHeader, showHeader, hideHeader } = useHeaderControl();
const handlerReadyRef = ref(false);
const handlerComponent: Ref<Component | null> = shallowRef(null);
const { isVisible: isSidebarMenuVisible, reset: resetSidebarMenu, hide: hideSidebarMenu } = useSidebarMenu();

const cancelRouteWatch = watchRoute(async () => {
  if (!currentRouteIs(Routes.FileHandler)) {
    return;
  }

  const query = getCurrentRouteQuery();
  const timestamp = Number.isNaN(Number(query.timestamp)) ? undefined : Number(query.timestamp);

  // Same file, no need to reload
  if (contentInfo.value && contentInfo.value.path === getDocumentPath() && atDateTime.value?.toMillis() === timestamp) {
    return;
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
  const routeParams = getCurrentRouteParams() as { mode: FileHandlerMode };
  switch (routeParams.mode) {
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
  await loadFile();
  loadComponent();
  // Set header hidden by default when entering handler
  hideHeader();
  hideSidebarMenu();
});

onUnmounted(async () => {
  cancelRouteWatch();
  // Ensure header is visible when leaving handler
  resetSidebarMenu();
  showHeader();
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
          message: 'fileViewers.statsFailed',
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
      },
    });
    await modal.present();
    await modal.onWillDismiss();
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

  const config = await storageManager.retrieveConfig();
  if (!config.disableDownloadWarning) {
    const { result, noReminder } = await askDownloadConfirmation();

    if (noReminder) {
      config.disableDownloadWarning = true;
      await storageManager.storeConfig(config);
    }
    if (result !== MsModalResult.Confirm) {
      return;
    }
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
        --fill-color: var(--parsec-color-light-secondary-grey);
        padding: 0.625rem;
        border-radius: var(--parsec-radius-12);
        cursor: pointer;

        &:hover {
          background: var(--parsec-color-light-secondary-premiere);
          --fill-color: var(--parsec-color-light-secondary-hard-grey);
        }
      }

      .file-icon {
        width: 2rem;
        height: 2rem;
        min-width: 2rem;
        min-height: 2rem;
        margin-left: 0.5rem;
      }

      &__title {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        overflow: hidden;

        .title-h3 {
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

        &:hover {
          border-color: var(--parsec-color-light-primary-100);
        }

        &__item {
          background: none;
          color: var(--parsec-color-light-secondary-text);
          border-radius: var(--parsec-radius-8);
          transition: all 150ms linear;

          &::part(native) {
            --background-hover: none;
            padding: 0.75rem 1.125rem;
          }

          &:hover {
            background: var(--parsec-color-light-secondary-medium);
            color: var(--parsec-color-light-secondary-text);
          }

          ion-icon {
            font-size: 1rem;
            margin-right: 0.5rem;
          }
        }

        .toggle-menu {
          position: relative;
          margin-left: 0.5rem;

          &::part(native) {
            --background-hover: none;
            padding: 0;
          }

          ion-icon {
            margin-inline: 0.5rem 0;
          }

          &::before {
            content: '';
            position: absolute;
            top: 50%;
            left: -1.5rem;
            transform: translateY(-50%);
            width: 1px;
            height: 1.5rem;
            background: var(--parsec-color-light-secondary-disabled);
          }

          &::after {
            content: '';
            position: absolute;
            left: 0;
            bottom: -0.25rem;
            width: 0;
            height: 1px;
            background: var(--parsec-color-light-secondary-text);
            transition: all 150ms linear;
          }

          &:hover {
            background: none;

            &::after {
              background: var(--parsec-color-light-secondary-text);
              width: calc(100% - 1.5rem);
            }
          }
        }
      }
    }
  }
}
</style>
