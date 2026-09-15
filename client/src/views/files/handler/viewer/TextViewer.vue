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

// Used to save when exiting if needed
const isDirty = ref(false);
// Used as a lock to avoid concurrent saves
const saving = ref(false);

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
    editor = monaco.editor.create(containerRef.value, {
      value: content,
      language: detectLanguage(),
      readOnly: isReadOnly.value,
      automaticLayout: true,
      stickyScroll: { enabled: false },
      wordWrap: 'on',
      scrollBeyondLastLine: false,
      unicodeHighlight: { ambiguousCharacters: false, invisibleCharacters: false },
      minimap: { enabled: false },
      glyphMargin: false,
      codeLens: false,
      quickSuggestions: false,
    });
    if (!isReadOnly.value) {
      subscription = editor.onDidChangeModelContent((_event: monaco.editor.IModelContentChangedEvent) => {
        isDirty.value = true;
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
  if (isDirty.value) {
    await saveDocument();
  }
  if (subscription) {
    subscription.dispose();
  }
  if (editor) {
    editor.dispose();
  }
});

async function saveDocument(): Promise<void> {
  if (!editor || saving.value || !isDirty.value) {
    return;
  }
  saving.value = true;
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
    isDirty.value = false;
    saving.value = false;
  }
}

function detectLanguage(): string | undefined {
  switch (props.contentInfo.extension) {
    case 'xml':
      return 'xml';
    case 'json':
      return 'json';
    case 'js':
    case 'jsx':
    case 'mjs':
    case 'cjs':
      return 'javascript';
    case 'html':
    case 'htm':
    case 'xhtml':
      return 'html';
    case 'sh':
      return 'shell';
    case 'css':
      return 'css';
    case 'scss':
      return 'scss';
    case 'less':
      return 'less';
    case 'py':
      return 'python';
    case 'php':
      return 'php';
    case 'h':
    case 'c':
      return 'c';
    case 'hpp':
    case 'cpp':
      return 'cpp';
    case 'rs':
      return 'rust';
    case 'java':
      return 'java';
    case 'ts':
    case 'tsx':
      return 'typescript';
    case 'ini':
    case 'properties':
      return 'ini';
    case 'cs':
      return 'csharp';
    case 'vb':
    case 'vbs':
      return 'vb';
    case 'swift':
      return 'swift';
    case 'lua':
      return 'lua';
    case 'rb':
      return 'ruby';
    case 'md':
      return 'markdown';
    case 'rst':
      return 'restructuredtext';
    case 'kt':
      return 'kotlin';
    case 'yml':
    case 'yaml':
      return 'yaml';
    case 'go':
      return 'go';
    case 'dart':
      return 'dart';
    case 'sql':
      return 'sql';
    case 'ps1':
    case 'psm1':
    case 'psd1':
      return 'powershell';
    case 'pl':
      return 'perl';
    case 'm':
      return 'objective-c';
    case 'bat':
    case 'cmd':
      return 'bat';
    case 'graphql':
    case 'gql':
      return 'graphql';
    case 'proto':
      return 'proto';
    case 'fs':
    case 'fsx':
      return 'fsharp';
    case 'hbs':
      return 'handlebars';
    case 'tf':
    case 'tfvars':
      return 'hcl';
    case 'scala':
    case 'sbt':
      return 'scala';
    // No Monaco basic-language registered for these, falls back to plaintext:
    // csv, tex, txt, log, toml, po, vue, conf, cfg, diff, patch
    default:
      return undefined;
  }
}
</script>

<style scoped lang="scss">
.text-container {
  background-color: var(--parsec-color-light-secondary-premiere);
  width: 100%;
  height: 100%;
}
</style>
