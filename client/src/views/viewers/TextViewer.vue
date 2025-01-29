<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper>
    <template #viewer>
      <div
        ref="container"
        class="text-container"
      />
    </template>
    <template #controls>
      <file-controls>
        <file-controls-zoom
          @change="onZoomLevelChange"
          ref="zoomControl"
        />
      </file-controls>
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import * as monaco from 'monaco-editor';
import { FileContentInfo } from '@/views/viewers/utils';
import { FileViewerWrapper } from '@/views/viewers';
import { FileControls, FileControlsZoom } from '@/components/viewers';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const container = ref();
const editor = ref<monaco.editor.IStandaloneCodeEditor>();
const zoomControl = ref();
const zoomLevel = ref(1);

onMounted(async () => {
  const text = new TextDecoder().decode(props.contentInfo.data);
  editor.value = monaco.editor.create(container.value, {
    value: text,
    readOnly: true,
    fontSize: 16,
    theme: 'vs-dark',
    automaticLayout: true,
    language: getLanguage(),
    scrollbar: {
      vertical: 'auto',
      horizontal: 'auto',
    },
  });
  updateEditorZoomLevel();
});

function getLanguage(): string | undefined {
  const langs = new Map<string, string>([
    ['py', 'python'],
    ['cpp', 'cpp'],
    ['c', 'c'],
    ['rs', 'rust'],
  ]);
  return langs.get(props.contentInfo.extension);
}

function onZoomLevelChange(value: number): void {
  zoomLevel.value = value / 100;
  updateEditorZoomLevel();
}

function updateEditorZoomLevel(): void {
  // Monaco editor use 'editorZoomLevelMultiplier = 1 + editorZoomLevel * 0.1' to calculate zoom level.
  // Here we update the value we send to minimize the effect of the specific calculation.
  const editorZoomLevel = (zoomLevel.value - 1) / 0.2;
  monaco.editor.EditorZoom.setZoomLevel(editorZoomLevel);
}
</script>

<style scoped lang="scss">
.text-container {
  width: 100%;
  height: 100%;
}
</style>
