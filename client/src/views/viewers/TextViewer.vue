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
        <file-controls-group class="file-controls-font-size">
          <ms-dropdown
            class="dropdown"
            :options="fontOptions"
            :default-option-key="fontSize"
            @change="onFontSizeChange($event.option.key)"
          />
        </file-controls-group>
      </file-controls>
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import * as monaco from 'monaco-editor';
import { FileContentInfo } from '@/views/viewers/utils';
import { FileViewerWrapper } from '@/views/viewers';
import { FileControls, FileControlsGroup } from '@/components/viewers';
import { MsDropdown, MsOption, MsOptions } from 'megashark-lib';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const fontSize = ref(13);
const container = ref();
const editor = ref();
const fontOptions = new MsOptions(
  ((): MsOption[] => {
    const fontItems: MsOption[] = [];
    for (let i = 6; i <= 30; i++) {
      fontItems.push({ key: i, label: { key: 'fileViewers.fontLabel', data: { number: i } } });
    }
    return fontItems;
  })(),
);

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

function onFontSizeChange(value: number): void {
  fontSize.value = value;
  editor.value.updateOptions({ fontSize: fontSize.value });
}
</script>

<style scoped lang="scss">
.text-container {
  width: 100%;
  height: 100%;
}

.file-controls-font-size {
  margin: 0 0.5rem;
}
</style>
