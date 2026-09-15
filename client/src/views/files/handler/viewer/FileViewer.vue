<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <component
    :is="viewerComponent"
    :content-info="contentInfo"
    v-bind="viewerComponent === TextViewer ? { readOnly: readOnly } : {}"
    v-on="viewerComponent === TextViewer ? { onSaveStateChange: onSaveStateChange } : {}"
  />
</template>

<script setup lang="ts">
import { FileContentType } from '@/common/fileTypes';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { SaveState } from '@/views/files/handler/types';
import { AudioViewer, ImageViewer, PdfViewer, TextViewer, VideoViewer } from '@/views/files/handler/viewer';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import { inject, onMounted, onUnmounted, Ref, shallowRef } from 'vue';

type ViewerType = typeof AudioViewer | typeof ImageViewer | typeof PdfViewer | typeof TextViewer | typeof VideoViewer;

const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
const viewerComponent: Ref<ViewerType | undefined> = shallowRef(undefined);

const { contentInfo } = defineProps<{
  contentInfo: FileContentInfo;
  readOnly: boolean;
}>();

const emits = defineEmits<{
  (event: 'fileLoaded'): void;
  (event: 'fileError'): void;
  (event: 'onSaveStateChange', saveState: SaveState): void;
}>();

defineExpose({ save });

onMounted(async () => {
  await loadFile();
});

onUnmounted(() => {
  viewerComponent.value = undefined;
});

function onSaveStateChange(state: SaveState): void {
  emits('onSaveStateChange', state);
}

async function loadFile(): Promise<void> {
  viewerComponent.value = await getComponent();
  if (!viewerComponent.value) {
    emitError(`No component for file with extension '${contentInfo.extension}'`);
    return;
  }
  emits('fileLoaded');
}

async function getComponent(): Promise<ViewerType | undefined> {
  switch (contentInfo.contentType) {
    case FileContentType.Image:
      return ImageViewer;
    case FileContentType.Video:
      return VideoViewer;
    case FileContentType.Audio:
      return AudioViewer;
    case FileContentType.PdfDocument:
      return PdfViewer;
    case FileContentType.Text:
      return TextViewer;
  }
}

async function save(): Promise<boolean> {
  if (viewerComponent.value?.saveDocument) {
    window.nativeAPI.log('debug', 'Forced save on a viewer');
    return await viewerComponent.value.saveDocument();
  }
  return true;
}

function emitError(message: string): void {
  window.nativeAPI.log('error', message);
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
