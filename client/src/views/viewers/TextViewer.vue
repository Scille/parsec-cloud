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
        <file-controls-fullscreen @click="toggleFullScreen" />
      </file-controls>
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import * as monaco from 'monaco-editor';
import { FileContentInfo } from '@/views/viewers/utils';
import { FileViewerWrapper } from '@/views/viewers';
import { FileControls, FileControlsFullscreen } from '@/components/viewers';
const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const container = ref();

onMounted(async () => {
  const text = new TextDecoder().decode(props.contentInfo.data);
  monaco.editor.create(container.value, {
    value: text,
    readOnly: true,
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

async function toggleFullScreen(): Promise<void> {
  await container.value?.requestFullscreen();
}
</script>

<style scoped lang="scss">
.text-container {
  width: 100%;
  height: 100%;
}
</style>
