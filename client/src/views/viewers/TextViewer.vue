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
        <file-controls-zoom @change="onZoomLevelChange" />
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

const fontSize = ref(13);
const container = ref();
const editor = ref();

onMounted(async () => {
  const text = new TextDecoder().decode(props.contentInfo.data);
  editor.value = monaco.editor.create(container.value, {
    value: text,
    readOnly: true,
    fontSize: fontSize.value,
    theme: 'vs-dark',
    language: getLanguage(),
  });
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
  switch (value) {
    case 5:
      fontSize.value = 6;
      break;
    case 10:
      fontSize.value = 6.5;
      break;
    case 20:
      fontSize.value = 7;
      break;
    case 30:
      fontSize.value = 7.5;
      break;
    case 40:
      fontSize.value = 8;
      break;
    case 50:
      fontSize.value = 8.5;
      break;
    case 60:
      fontSize.value = 9;
      break;
    case 70:
      fontSize.value = 10;
      break;
    case 80:
      fontSize.value = 11;
      break;
    case 90:
      fontSize.value = 12;
      break;
    case 100:
      fontSize.value = 13;
      break;
    case 125:
      fontSize.value = 14;
      break;
    case 150:
      fontSize.value = 15;
      break;
    case 175:
      fontSize.value = 16;
      break;
    case 200:
      fontSize.value = 17;
      break;
    case 250:
      fontSize.value = 20;
      break;
    case 300:
      fontSize.value = 22;
      break;
    case 400:
      fontSize.value = 24;
      break;
    case 500:
      fontSize.value = 28;
      break;
    default:
      fontSize.value = 13;
  }
  editor.value.updateOptions({'fontSize': fontSize.value});
}
</script>

<style scoped lang="scss">
.text-container {
  width: 100%;
  height: 100%;
}
</style>
