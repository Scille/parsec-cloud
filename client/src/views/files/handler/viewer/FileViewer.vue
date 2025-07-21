<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <component
    :is="viewerComponent"
    :content-info="contentInfoRef"
  />
</template>

<script setup lang="ts">
import { ref, Ref, type Component, inject, shallowRef, onMounted, onUnmounted } from 'vue';
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

const informationManager: InformationManager = inject(InformationManagerKey)!;
const viewerComponent: Ref<Component | null> = shallowRef(null);
const contentInfoRef: Ref<FileContentInfo | undefined> = ref(undefined);

const { contentInfo, fileInfo } = defineProps<{
  contentInfo: FileContentInfo;
  fileInfo: DetectedFileType;
}>();

const emits = defineEmits<{
  (event: 'fileLoaded'): void;
}>();

onMounted(async () => {
  await onFileLoaded(contentInfo, fileInfo);
  emits('fileLoaded');
});

onUnmounted(() => {
  contentInfoRef.value = undefined;
  viewerComponent.value = null;
});

async function onFileLoaded(contentInfo: FileContentInfo, fileInfo: DetectedFileType): Promise<void> {
  contentInfoRef.value = contentInfo;

  const component = await getComponent(fileInfo);
  if (!component) {
    throwError(`No component for file with extension '${fileInfo.extension}'`);
  }

  viewerComponent.value = component;
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

function throwError(message: string): never {
  window.electronAPI.log('error', message);
  informationManager.present(
    new Information({
      message: 'fileViewers.genericError',
      level: InformationLevel.Error,
    }),
    PresentationMode.Toast,
  );
  throw new Error(message);
}
</script>

<style scoped lang="scss"></style>
