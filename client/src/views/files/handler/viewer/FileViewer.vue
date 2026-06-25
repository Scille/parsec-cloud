<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <component
    :is="viewerComponent"
    :content-info="contentInfo"
  />
</template>

<script setup lang="ts">
import { FileContentType } from '@/common/fileTypes';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { AudioViewer, ImageViewer, PdfViewer, VideoViewer } from '@/views/files/handler/viewer';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import { inject, onMounted, onUnmounted, Ref, shallowRef, type Component } from 'vue';

const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
const viewerComponent: Ref<Component | null> = shallowRef(null);

const { contentInfo } = defineProps<{
  contentInfo: FileContentInfo;
}>();

const emits = defineEmits<{
  (event: 'fileLoaded'): void;
  (event: 'fileError'): void;
}>();

onMounted(async () => {
  await loadFile();
});

onUnmounted(() => {
  viewerComponent.value = null;
});

async function loadFile(): Promise<void> {
  const component = await getComponent();
  if (!component) {
    emitError(`No component for file with extension '${contentInfo.extension}'`);
    return;
  }
  viewerComponent.value = component;
  emits('fileLoaded');
}

async function getComponent(): Promise<Component | undefined> {
  switch (contentInfo.contentType) {
    case FileContentType.Image:
      return ImageViewer;
    case FileContentType.Video:
      return VideoViewer;
    case FileContentType.Audio:
      return AudioViewer;
    case FileContentType.PdfDocument:
      return PdfViewer;
  }
}

function emitError(message: string): void {
  window.electronAPI.log('error', message);
  informationManager.value.present(
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
