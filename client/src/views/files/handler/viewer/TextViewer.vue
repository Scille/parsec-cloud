<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper>
    <template #viewer>
      <ms-report-text
        v-if="error"
        :theme="MsReportTheme.Error"
      >
        {{ $msTranslate(error) }}
      </ms-report-text>
      <div
        class="text-container"
        ref="textContainer"
      >
        <ms-spinner v-show="loading" />
      </div>
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import { getFileContent } from '@/common/file';
import { closeFile, openFile, writeFile } from '@/parsec';
import { SaveState } from '@/views/files/handler/types';
import { FileViewerWrapper } from '@/views/files/handler/viewer';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import { MsReportText, MsReportTheme, MsSpinner } from 'megashark-lib';
import * as monaco from 'monaco-editor';
import { computed, onMounted, onUnmounted, ref, useTemplateRef } from 'vue';

const props = defineProps<{
  contentInfo: FileContentInfo;
  readOnly: boolean;
}>();

const SAVE_INTERVAL = 5000;

const loading = ref(true);
const error = ref('');
const containerRef = useTemplateRef<HTMLDivElement>('textContainer');
let editor: monaco.editor.IStandaloneCodeEditor | undefined = undefined;
let subscription: monaco.IDisposable | undefined = undefined;
let saveTimeout: any = undefined;

const isReadOnly = computed(() => {
  return Boolean(props.contentInfo.timestamp || props.readOnly);
});

const emits = defineEmits<{
  (event: 'onSaveStateChange', saveState: SaveState): void;
}>();

onMounted(async () => {
  loading.value = true;

  try {
    const raw = await getFileContent(props.contentInfo.workspaceHandle, props.contentInfo.path, props.contentInfo.timestamp);
    if (!raw || !containerRef.value) {
      throw new Error('failed to load content');
    }
    const content = new TextDecoder().decode(raw);
    editor = monaco.editor.create(containerRef.value, { value: content, language: detectLanguage(), readOnly: isReadOnly.value });
    if (!isReadOnly.value) {
      subscription = editor.onDidChangeModelContent((_event: monaco.editor.IModelContentChangedEvent) => {
        if (saveTimeout) {
          clearTimeout(saveTimeout);
          saveTimeout = undefined;
        }
        saveTimeout = setTimeout(() => {
          clearTimeout(saveTimeout);
          saveTimeout = undefined;
          saveDocument();
        }, SAVE_INTERVAL);
        emits('onSaveStateChange', SaveState.Unsaved);
      });
    }
  } catch (e: unknown) {
    window.nativeAPI.log('error', `Failed to load text file: ${e}`);
    error.value = 'fileViewers.text.loadDocumentError';
  } finally {
    loading.value = false;
  }
});

onUnmounted(async () => {
  if (saveTimeout) {
    clearTimeout(saveTimeout);
  }
  if (subscription) {
    subscription.dispose();
  }
  if (editor) {
    editor.dispose();
  }
});

async function saveDocument(): Promise<void> {
  if (!editor) {
    return;
  }
  emits('onSaveStateChange', SaveState.Saving);
  const openResult = await openFile(props.contentInfo.workspaceHandle, props.contentInfo.path, { write: true, truncate: true });

  if (!openResult.ok) {
    emits('onSaveStateChange', SaveState.Error);
    return;
  }
  try {
    const raw = new TextEncoder().encode(editor.getValue());
    const writeResult = await writeFile(props.contentInfo.workspaceHandle, openResult.value, 0, raw);
    if (!writeResult.ok) {
      emits('onSaveStateChange', SaveState.Error);
      return;
    }
    emits('onSaveStateChange', SaveState.Saved);
  } finally {
    await closeFile(props.contentInfo.workspaceHandle, openResult.value);
  }
}

function detectLanguage(): string | undefined {
  switch (props.contentInfo.extension) {
    case 'py':
      return 'python';
    default:
      undefined;
  }
}
</script>

<style scoped lang="scss">
.text-container {
  background-color: var(--parsec-color-light-secondary-premiere);
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: start;
  align-items: center;
  overflow-y: auto;
  gap: 2em;
  padding: 3em 0;
}
</style>
