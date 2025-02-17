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
          v-if="loaded && viewerComponent && contentInfo"
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
                  :image="Download"
                  class="file-icon download-icon"
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
                v-show="false"
                class="file-viewer-topbar-buttons__item toggle-menu"
              >
                {{ $msTranslate('fileViewers.hideMenu') }}
              </ion-button>
            </ion-buttons>
          </div>

          <!-- file-viewer component -->
          <component
            :is="viewerComponent"
            :content-info="contentInfo"
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
  entryStatAt,
  openFileAt,
  readHistoryFile,
  closeHistoryFile,
  DEFAULT_READ_SIZE,
} from '@/parsec';
import { IonPage, IonContent, IonButton, IonText, IonIcon, IonButtons, modalController } from '@ionic/vue';
import { link, informationCircle, open } from 'ionicons/icons';
import { Base64, MsSpinner, MsImage, I18n, Download } from 'megashark-lib';
import { ref, Ref, type Component, inject, onMounted, shallowRef, onUnmounted } from 'vue';
import { ImageViewer, VideoViewer, SpreadsheetViewer, DocumentViewer, AudioViewer, TextViewer, PdfViewer } from '@/views/viewers';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { currentRouteIs, getCurrentRouteQuery, getDocumentPath, getWorkspaceHandle, navigateTo, Routes, watchRoute } from '@/router';
import { DetectedFileType, FileContentType } from '@/common/fileTypes';
import { FileContentInfo } from '@/views/viewers/utils';
import { Env } from '@/services/environment';
import { DateTime } from 'luxon';
import { getFileIcon } from '@/common/file';
import { copyPathLinkToClipboard } from '@/components/files';
import { FileDetailsModal } from '@/views/files';
import { showSaveFilePicker } from 'native-file-system-adapter';

const informationManager: InformationManager = inject(InformationManagerKey)!;
const viewerComponent: Ref<Component | null> = shallowRef(null);
const contentInfo: Ref<FileContentInfo | null> = ref(null);
const detectedFileType = ref<DetectedFileType | null>(null);
const loaded = ref(false);
const atDateTime: Ref<DateTime | undefined> = ref(undefined);

const cancelRouteWatch = watchRoute(async () => {
  if (!currentRouteIs(Routes.Viewer)) {
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

async function loadFile(): Promise<void> {
  loaded.value = false;
  contentInfo.value = null;
  detectedFileType.value = null;
  atDateTime.value = undefined;
  viewerComponent.value = null;
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

  let statsResult;

  if (!atDateTime.value) {
    statsResult = await entryStat(workspaceHandle, path);
  } else {
    statsResult = await entryStatAt(workspaceHandle, path, atDateTime.value);
  }
  if (!statsResult.ok || !statsResult.value.isFile()) {
    window.electronAPI.log('error', 'Failed to stat the entry or entry is not a file');
    return;
  }

  const component = await getComponent(fileInfo);
  if (!component) {
    window.electronAPI.log('error', `No component for file of type ${fileInfo.mimeType}`);
    return;
  }
  viewerComponent.value = component;

  let openResult;

  if (!atDateTime.value) {
    openResult = await openFile(workspaceHandle, path, { read: true });
  } else {
    openResult = await openFileAt(workspaceHandle, path, atDateTime.value);
  }
  if (!openResult.ok) {
    await openWithSystem(path);
    return;
  }
  contentInfo.value = {
    data: new Uint8Array((statsResult.value as EntryStatFile).size),
    extension: fileInfo.extension,
    mimeType: fileInfo.mimeType,
    fileName: fileName,
    path: path,
  };
  const fd = openResult.value;
  try {
    let loop = true;
    let offset = 0;
    while (loop) {
      let readResult;
      if (!atDateTime.value) {
        readResult = await readFile(workspaceHandle, openResult.value, offset, DEFAULT_READ_SIZE);
      } else {
        readResult = await readHistoryFile(workspaceHandle, openResult.value, offset, DEFAULT_READ_SIZE);
      }
      if (!readResult.ok) {
        throw Error(JSON.stringify(readResult.error));
      }
      const buffer = new Uint8Array(readResult.value);
      contentInfo.value?.data.set(buffer, offset);
      if (readResult.value.byteLength < DEFAULT_READ_SIZE) {
        loop = false;
      }
      offset += readResult.value.byteLength;
    }
    loaded.value = true;
  } catch (e: any) {
    window.electronAPI.log('error', `Can't view the file: ${e.toString()}`);
    informationManager.present(
      new Information({
        message: 'fileViewers.genericError',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    contentInfo.value = null;
    viewerComponent.value = null;
    if (!(await openWithSystem(path))) {
      await navigateTo(Routes.Documents, { query: { workspaceHandle: workspaceHandle, documentPath: await Path.parent(path) } });
    }
  } finally {
    if (!atDateTime.value) {
      await closeFile(workspaceHandle, fd);
    } else {
      await closeHistoryFile(workspaceHandle, fd);
    }
  }
}

onMounted(async () => {
  await loadFile();
});

onUnmounted(async () => {
  cancelRouteWatch();
});

async function getComponent(fileInfo: DetectedFileType): Promise<Component | undefined> {
  switch (fileInfo.type) {
    case FileContentType.Image:
      return ImageViewer;
    case FileContentType.Video:
      return VideoViewer;
    case FileContentType.Spreadsheet:
      return SpreadsheetViewer;
    case FileContentType.Audio:
      return AudioViewer;
    case FileContentType.Document:
      return DocumentViewer;
    case FileContentType.Text:
      return TextViewer;
    case FileContentType.PdfDocument:
      return PdfViewer;
  }
}

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
        await Env.Links.openLink(href);
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

  .file-viewer {
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: space-between;

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
        margin-left: 0.5rem;
      }

      &__title {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;

        .title-h3 {
          color: var(--parsec-color-light-secondary-text);
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
