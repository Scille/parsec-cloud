<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    ref="container"
    class="text-container"
  />
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import * as monaco from 'monaco-editor';
import { FileContentInfo } from '@/views/viewers/utils';

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
</script>

<style scoped lang="scss">
.text-container {
  width: 100%;
  height: 680px;
}
</style>
