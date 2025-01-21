<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <div class="container">
        <ms-spinner v-show="!loaded" />
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
            <ion-text class="title-h3">
              {{ contentInfo.fileName }}
            </ion-text>
            <!-- Here we could put the file action buttons -->
            <ion-buttons class="file-viewer-topbar-buttons">
              <ion-button
                class="file-viewer-topbar-buttons__item"
                @click="openWithSystem(contentInfo.path)"
                v-show="isDesktop()"
              >
                <ion-icon :icon="open" />
                {{ $msTranslate('fileViewers.openWithDefault') }}
              </ion-button>
              <ion-button
                class="file-viewer-topbar-buttons__item"
                @click="showDetails"
              >
                <ion-icon :icon="informationCircle" />
                {{ $msTranslate('fileViewers.details') }}
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
  isDesktop,
  entryStatAt,
  openFileAt,
  readHistoryFile,
  closeHistoryFile,
  DEFAULT_READ_SIZE,
} from '@/parsec';
import { IonPage, IonContent, IonButton, IonText, IonIcon, IonButtons, modalController } from '@ionic/vue';
import { informationCircle, open } from 'ionicons/icons';
import { Base64, MsSpinner, MsImage } from 'megashark-lib';
import { ref, Ref, type Component, inject, onMounted, shallowRef } from 'vue';
import { ImageViewer, VideoViewer, SpreadsheetViewer, DocumentViewer, AudioViewer, TextViewer, PdfViewer } from '@/views/viewers';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { getCurrentRouteQuery, getDocumentPath, getWorkspaceHandle, navigateTo, Routes } from '@/router';
import { DetectedFileType, FileContentType } from '@/common/fileTypes';
import { FileContentInfo } from '@/views/viewers/utils';
import { Env } from '@/services/environment';
import { DateTime } from 'luxon';
import { getFileIcon } from '@/common/file';
import { FileDetailsModal } from '@/views/files';

const informationManager: InformationManager = inject(InformationManagerKey)!;
const viewerComponent: Ref<Component | null> = shallowRef(null);
const contentInfo: Ref<FileContentInfo | null> = ref(null);
const detectedFileType = ref<DetectedFileType | null>(null);
const loaded = ref(false);
const atDateTime: Ref<DateTime | null> = ref(null);

onMounted(async () => {
  loaded.value = false;
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
    return;
  }

  const timestamp = Number(getCurrentRouteQuery().timestamp);
  if (timestamp) {
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

  const result = await getSystemPath(workspaceHandle, path);

  if (!result.ok) {
    await informationManager.present(
      new Information({
        message: 'FoldersPage.open.fileFailed',
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

async function showDetails(): Promise<void> {
  const workspaceHandle = getWorkspaceHandle();
  if (workspaceHandle) {
    const entry = await entryStat(workspaceHandle, getDocumentPath());
    if (!entry.ok) {
      await informationManager.present(
        new Information({
          message: 'FoldersPage.open.fileFailed',
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
</script>

<style scoped lang="scss">
.container {
  height: 100%;
  height: -webkit-fill-available;
  height: -moz-available;
  height: stretch;
  margin: 0 1em 1em 1em;
  padding: 1.5em;
  border-radius: var(--parsec-radius-12);
  background-color: var(--parsec-color-light-secondary-background);

  .file-viewer {
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    gap: 1.5rem;

    &-topbar {
      display: flex;
      align-items: center;
      gap: 1rem;
      padding-bottom: 1rem;
      border-bottom: 1px solid var(--parsec-color-light-secondary-disabled);

      .file-icon {
        width: 1.5rem;
        height: 1.5rem;
      }

      .title-h3 {
        color: var(--parsec-color-light-primary-700);
      }

      &-buttons {
        margin-left: auto;
        display: flex;
        gap: 1rem;

        &:hover {
          border-color: var(--parsec-color-light-primary-100);
        }

        &__item {
          background: var(--parsec-color-light-primary-100);
          color: var(--parsec-color-light-primary-600);
          border-radius: var(--parsec-radius-32);
          transition: all 150ms linear;

          &::part(native) {
            --background-hover: none;
            border-radius: var(--parsec-radius-32);
            padding: 0.5rem 1rem;
          }

          &:hover {
            background: var(--parsec-color-light-primary-200);
            color: var(--parsec-color-light-primary-700);
            scale: 1.01;
            box-shadow: var(--parsec-shadow-light);
          }

          ion-icon {
            font-size: 1.25rem;
            margin-right: 0.5rem;
          }
        }
      }
    }
  }
}
</style>
