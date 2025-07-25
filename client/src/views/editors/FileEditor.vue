<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <div class="container">
        <ms-spinner
          class="file-viewer"
          title="fileViewers.retrievingFileContent"
          v-show="!loaded"
        />
        <div
          v-if="loaded && contentInfo && documentType"
          @click.prevent="onClick"
          class="file-viewer"
        >
          <!-- file-viewer topbar -->
          <div class="file-viewer-topbar">
            <ms-image
              :image="getFileIcon(contentInfo.fileName)"
              class="file-icon"
            />
            <div class="file-viewer-topbar__title">
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
            <ion-buttons class="file-viewer-topbar-buttons">
              <ion-button
                class="file-viewer-topbar-buttons__item"
                @click="showDetails"
                v-if="isDesktop()"
              >
                <ion-icon :icon="informationCircle" />
                {{ $msTranslate('fileViewers.details') }}
              </ion-button>
              <ion-button
                class="file-viewer-topbar-buttons__item"
                @click="copyPath(contentInfo.path)"
                v-if="isWeb()"
              >
                <ion-icon :icon="link" />
                {{ $msTranslate('fileViewers.copyLink') }}
              </ion-button>
              <ion-button
                class="file-viewer-topbar-buttons__item"
                @click="downloadFile(contentInfo.path)"
                v-if="isWeb()"
              >
                <ms-image
                  :image="DownloadIcon"
                  class="download-icon"
                />
                {{ $msTranslate('fileViewers.download') }}
              </ion-button>
              <ion-button
                class="file-viewer-topbar-buttons__item"
                @click="openWithSystem(contentInfo.path)"
                v-show="isDesktop() && !atDateTime"
              >
                <ion-icon :icon="open" />
                {{ $msTranslate('fileViewers.openWithDefault') }}
              </ion-button>
              <ion-button
                class="file-viewer-topbar-buttons__item toggle-menu"
                @click="toggleMainHeader"
                :class="{ 'header-visible': isHeaderVisible }"
              >
                <ion-icon :icon="isHeaderVisible ? chevronUp : chevronDown" />
                {{ $msTranslate(isHeaderVisible ? 'fileViewers.hideMenu' : 'fileViewers.showMenu') }}
              </ion-button>
            </ion-buttons>
          </div>

          <!-- file-editor component -->
          <div
            class="file-editor"
            id="file-editor"
            ref="fileEditor"
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
  createReadStream,
  isDesktop,
  isWeb,
  WorkspaceHistory,
  DEFAULT_READ_SIZE,
  WorkspaceHandle,
  EntryName,
  getWorkspaceInfo,
  WorkspaceHistoryEntryStatFile,
  writeFile,
} from '@/parsec';
import { IonPage, IonContent, IonButton, IonText, IonIcon, IonButtons, modalController } from '@ionic/vue';
import { link, informationCircle, open, chevronDown, chevronUp } from 'ionicons/icons';
import { Base64, MsSpinner, MsImage, I18n, DownloadIcon } from 'megashark-lib';
import { ref, Ref, inject, onMounted, onUnmounted } from 'vue';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { currentRouteIs, getCurrentRouteQuery, getDocumentPath, getWorkspaceHandle, navigateTo, Routes, watchRoute } from '@/router';
import { DetectedFileType } from '@/common/fileTypes';
import { FileContentInfo } from '@/views/viewers/utils';
import { Env } from '@/services/environment';
import { DateTime } from 'luxon';
import { getFileIcon } from '@/common/file';
import { copyPathLinkToClipboard } from '@/components/files';
import { FileDetailsModal } from '@/views/files';
import { showSaveFilePicker } from 'native-file-system-adapter';
import { CryptpadDocumentType, Cryptpad, CryptpadConfig, getDocumentTypeFromExtension } from '@/services/cryptpad';
import { longLocaleCodeToShort } from '@/services/translation';
import useHeaderControl from '@/services/headerControl';

const informationManager: InformationManager = inject(InformationManagerKey)!;
const contentInfo: Ref<FileContentInfo | undefined> = ref(undefined);
const detectedFileType = ref<DetectedFileType | null>(null);
const loaded = ref(false);
const atDateTime: Ref<DateTime | undefined> = ref(undefined);
const fileEditor: Ref<HTMLDivElement | null> = ref(null);
const documentType = ref<CryptpadDocumentType | null>(null);
const cryptpadInstance = ref<Cryptpad | null>(null);
const fileInfoId = ref<string | undefined>(undefined);
const { isHeaderVisible, toggleHeader: toggleMainHeader, showHeader, hideHeader } = useHeaderControl();

const cancelRouteWatch = watchRoute(async () => {
  if (!currentRouteIs(Routes.Editor)) {
    return;
  }

  const query = getCurrentRouteQuery();
  const timestamp = Number.isNaN(Number(query.timestamp)) ? undefined : Number(query.timestamp);

  // Same file, no need to reload
  if (contentInfo.value && contentInfo.value.path === getDocumentPath() && atDateTime.value?.toMillis() === timestamp) {
    console.log('FileEditor: no need to reload, same file and timestamp');
    return;
  }
  await loadFile();
  await loadCryptpad();
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
      fileInfoId.value = statsResult.value.id;
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
    fileInfoId.value = statsResult.value.id;
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
  documentType.value = null;
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
    documentType.value = null;
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

  documentType.value = getDocumentTypeFromExtension(fileInfo.extension);

  if (!documentType.value) {
    window.electronAPI.log('error', `No document type for file with extension '${fileInfo.extension}'`);
    informationManager.present(
      new Information({
        message: 'fileViewers.genericError',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return;
  }
  contentInfo.value = info;
  if (timestamp) {
    atDateTime.value = DateTime.fromMillis(timestamp);
  }
  loaded.value = true;
}

async function getUrlFromFile(workspaceHandle: WorkspaceHandle, path: FsPath): Promise<string | undefined> {
  const statsResult = await entryStat(workspaceHandle, path);
  if (!statsResult.ok || !statsResult.value.isFile()) {
    return;
  }

  const openResult = await openFile(workspaceHandle, path, { read: true });

  if (!openResult.ok) {
    return undefined;
  }
  const fd = openResult.value;
  const content = new Uint8Array((statsResult.value as EntryStatFile).size);
  try {
    let loop = true;
    let offset = 0;
    while (loop) {
      const readResult = await readFile(workspaceHandle, openResult.value, offset, DEFAULT_READ_SIZE);
      if (!readResult.ok) {
        throw Error(JSON.stringify(readResult.error));
      }
      const buffer = new Uint8Array(readResult.value);
      content.set(buffer, offset);
      if (readResult.value.byteLength < DEFAULT_READ_SIZE) {
        loop = false;
      }
      offset += readResult.value.byteLength;
    }
    const urlObject = URL.createObjectURL(new Blob([content], { type: 'application/octet-stream' }));
    return urlObject;
  } catch (e: any) {
    window.electronAPI.log('error', `Can't read the file: ${e.toString()}`);
  } finally {
    await closeFile(workspaceHandle, fd);
  }
}

onMounted(async () => {
  // file viewer function
  await loadFile();
  await loadCryptpad();
  // Set header hidden by default when entering editor
  hideHeader();
});

async function loadCryptpad(): Promise<void> {
  // Cryptpad edition specific handling
  const fileUrl = await getUrlFromFile(getWorkspaceHandle()!, getDocumentPath());
  console.log('Cryptpad file URL:', fileUrl);
  if (!fileUrl) {
    window.electronAPI.log('error', 'Failed to retrieve file URL for Cryptpad edition');
    return;
  }
  const cryptpadConfig: CryptpadConfig = {
    document: {
      url: fileUrl,
      fileType: contentInfo.value?.extension || '',
      title: contentInfo.value?.fileName || '',
      key: fileInfoId.value,
    },
    documentType: documentType.value!,
    editorConfig: {
      lang: longLocaleCodeToShort(I18n.getLocale()),
    },
    autosave: 10,
    events: {
      // eslint-disable-next-line @typescript-eslint/explicit-function-return-type
      onSave: async (file: Blob, callback: () => void) => {
        console.log('Cryptpad onSave event triggered');
        // Handle save logic here
        const workspaceHandle = getWorkspaceHandle();
        const openResult = await openFile(workspaceHandle!, getDocumentPath(), { write: true, truncate: true });

        if (!openResult.ok) {
          return undefined;
        }
        const fd = openResult.value;
        const arrayBuffer = await file.arrayBuffer();
        await writeFile(workspaceHandle!, fd, 0, new Uint8Array(arrayBuffer));
        await closeFile(workspaceHandle!, fd);
        callback();
      },
      onHasUnsavedChanges: (unsaved: boolean) => {
        console.log('Cryptpad unsaved changes:', unsaved);
      },
    },
  };
  if (Env.isCryptpadEnabled()) {
    if (!cryptpadInstance.value) {
      cryptpadInstance.value = new Cryptpad(fileEditor.value as HTMLDivElement);
      console.log(cryptpadInstance.value);
      // await cryptpadInstance.value.close();
    }

    await cryptpadInstance.value.open(cryptpadConfig);
    loaded.value = true;
  }
}

onUnmounted(async () => {
  cancelRouteWatch();
  // Ensure header is visible when leaving editor
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
  if (event.target) {
    const target = event.target as Element;
    if (target.tagName.toLocaleLowerCase() === 'a') {
      const href = (target as HTMLLinkElement).href;
      if (href) {
        // await Env.Links.openLink(href);
      }
    }
  }
}

async function downloadFile(path: FsPath): Promise<void> {
  const workspaceHandle = getWorkspaceHandle();
  if (!workspaceHandle) {
    window.electronAPI.log('error', 'Failed to retrieve workspace handle');
    return;
  }

  if (!contentInfo.value) {
    window.electronAPI.log('error', 'No content info when trying to download a file');
    return;
  }

  try {
    const saveHandle = await showSaveFilePicker({
      _preferPolyfill: false,
      suggestedName: contentInfo.value.fileName,
    });

    const stream = await createReadStream(workspaceHandle, path);
    await stream.pipeTo(await saveHandle.createWritable());
  } catch (e: any) {
    window.electronAPI.log('error', `Failed to download file: ${e.toString()}`);
  }
}
</script>

<style scoped lang="scss">
.container {
  height: 100%;
  background-color: var(--parsec-color-light-secondary-premiere);

  .file-viewer,
  .file-editor {
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
          transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);

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

          &::after {
            content: '';
            position: absolute;
            left: 1.125rem;
            bottom: 0.25rem;
            width: 0;
            height: 1px;
            background: var(--parsec-color-light-secondary-text);
            transition: all 150ms linear;
          }

          &:hover {
            background: none;

            &::after {
              background: var(--parsec-color-light-secondary-text);
              width: calc(100% - 2.25rem);
            }
          }
        }
      }
    }
  }
}
</style>
