<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-handler @file-loaded="onFileLoaded">
    <!-- file-viewer component -->
    <component
      :is="viewerComponent"
      :content-info="contentInfoRef"
    />
  </file-handler>
</template>

<script setup lang="ts">
import { ref, Ref, type Component, inject, shallowRef } from 'vue';
import {
  ImageViewer,
  VideoViewer,
  SpreadsheetViewer,
  DocumentViewer,
  AudioViewer,
  TextViewer,
  PdfViewer,
} from '@/views/files/handler/viewer';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { DetectedFileType, FileContentType } from '@/common/fileTypes';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import FileHandler from '@/views/files/handler/FileHandler.vue';

const informationManager: InformationManager = inject(InformationManagerKey)!;
const viewerComponent: Ref<Component | null> = shallowRef(null);
const contentInfoRef: Ref<FileContentInfo | undefined> = ref(undefined);

const emits = defineEmits<{
  (e: 'handlerReady'): void;
}>();

async function onFileLoaded(contentInfo: FileContentInfo, fileInfo: DetectedFileType): Promise<void> {
  contentInfoRef.value = contentInfo;

  const component = await getComponent(fileInfo);
  if (!component) {
    window.electronAPI.log('error', `No component for file with extension '${fileInfo.extension}'`);
    informationManager.present(
      new Information({
        message: 'fileViewers.genericError',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return;
  }

  viewerComponent.value = component;
  emits('handlerReady');
}

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
</script>

<style scoped lang="scss"></style>
