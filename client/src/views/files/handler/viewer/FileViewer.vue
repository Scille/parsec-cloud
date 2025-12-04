<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <component
    :is="viewerComponent"
    :content-info="contentInfoRef"
  />
</template>

<script setup lang="ts">
import { DetectedFileType, FileContentType } from '@/common/fileTypes';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import {
  AudioViewer,
  DocumentViewer,
  ImageViewer,
  PdfViewer,
  SpreadsheetViewer,
  TextViewer,
  VideoViewer,
} from '@/views/files/handler/viewer';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import { inject, onMounted, onUnmounted, ref, Ref, shallowRef, type Component } from 'vue';

const informationManager: InformationManager = inject(InformationManagerKey)!;
const viewerComponent: Ref<Component | null> = shallowRef(null);
const contentInfoRef: Ref<FileContentInfo | undefined> = ref(undefined);

const { contentInfo, fileInfo } = defineProps<{
  contentInfo: FileContentInfo;
  fileInfo: DetectedFileType;
}>();

const emits = defineEmits<{
  (event: 'fileLoaded'): void;
  (event: 'fileError'): void;
}>();

onMounted(async () => {
  await onFileLoaded(contentInfo, fileInfo);
});

onUnmounted(() => {
  contentInfoRef.value = undefined;
  viewerComponent.value = null;
});

async function onFileLoaded(contentInfo: FileContentInfo, fileInfo: DetectedFileType): Promise<void> {
  contentInfoRef.value = contentInfo;

  const component = await getComponent(fileInfo);
  if (!component) {
    emitError(`No component for file with extension '${fileInfo.extension}'`);
    return;
  }

  viewerComponent.value = component;
  emits('fileLoaded');
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

function emitError(message: string): void {
  window.electronAPI.log('error', message);
  informationManager.present(
    new Information({
      message: 'fileViewers.errors.titles.genericError',
      level: InformationLevel.Error,
    }),
    PresentationMode.Toast,
  );
  emits('fileError');
}
</script>

<style scoped lang="scss"></style>
