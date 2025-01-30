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
    ['xml', 'xml'],
    ['json', 'json'],
    ['js', 'javascript'],
    ['html', 'html'],
    ['htm', 'html'],
    ['xhtml', 'html'],
    ['sh', 'shell'],
    ['php', 'php'],
    ['css', 'css'],
    ['tex', 'latex'],
    ['txt', 'plaintext'],
    ['h', 'c'],
    ['hpp', 'cpp'],
    ['c', 'c'],
    ['cpp', 'cpp'],
    ['rs', 'rust'],
    ['java', 'java'],
    ['ini', 'ini'],
    ['ts', 'typescript'],
    ['cs', 'csharp'],
    ['vb', 'vb'],
    ['vbs', 'vb'],
    ['swift', 'swift'],
    ['kt', 'kotlin'],
    ['lua', 'lua'],
    ['rb', 'ruby'],
    ['md', 'markdown'],
    ['log', 'plaintext'],
    ['rst', 'restructuredtext'],
    ['toml', 'toml'],
    ['po', 'plaintext'],
    ['ylm', 'yaml'],
  ]);
  return langs.get(props.contentInfo.extension);
}
</script>

<style scoped lang="scss">
.text-container {
  width: 100%;
  height: 100%;
}
</style>
