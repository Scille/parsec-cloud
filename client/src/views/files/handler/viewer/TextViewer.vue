<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper>
    <template #viewer>
      <ms-spinner
        :title="loadingLabel"
        v-show="loading"
      />
      <ms-report-text
        class="text-error"
        :theme="MsReportTheme.Error"
        v-if="error"
      >
        {{ $msTranslate(error) }}
      </ms-report-text>
      <div
        ref="textContainer"
        v-show="!loading && !error"
        class="text-container"
      />
    </template>
    <template #controls>
      <file-controls v-show="!loading && !error">
        <file-controls-zoom
          @change="onZoomLevelChange"
          ref="zoomControl"
        />
        <file-controls-group>
          <file-controls-button
            @click="toggleFullScreen"
            :icon="scan"
          />
        </file-controls-group>
      </file-controls>
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import { FileControls, FileControlsButton, FileControlsGroup, FileControlsZoom } from '@/components/files/handler/viewer';
import { FileViewerWrapper } from '@/views/files/handler/viewer';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import { scan } from 'ionicons/icons';
import { MsReportText, MsReportTheme, MsSpinner, Translatable } from 'megashark-lib';
import * as monaco from 'monaco-editor';
import { onMounted, ref, useTemplateRef } from 'vue';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const textContainerRef = useTemplateRef<HTMLDivElement>('textContainer');
const editor = ref<monaco.editor.IStandaloneCodeEditor>();
const zoomLevel = ref(1);
const loading = ref(true);
const error = ref<Translatable>('');
const loadingLabel = ref<Translatable>('');

onMounted(async () => {
  zoomLevel.value = 1;
  loading.value = true;
  loadingLabel.value = 'fileViewers.text.loadingDocument';

  try {
    const text = new TextDecoder().decode(props.contentInfo.data);
    createEditor(text);
  } catch (_err: any) {
    error.value = 'fileViewers.text.loadDocumentError';
  } finally {
    loading.value = false;
  }
  loadingLabel.value = '';
  updateEditorZoomLevel();
});

function createEditor(text: string): void {
  if (textContainerRef.value) {
    editor.value = monaco.editor.create(textContainerRef.value, {
      value: text,
      readOnly: true,
      fontSize: 16,
      theme: 'msEditorTheme',
      automaticLayout: true,
      language: getLanguage(),
      scrollbar: {
        vertical: 'auto',
        horizontal: 'auto',
        verticalScrollbarSize: 10,
        horizontalScrollbarSize: 10,
      },
    });
  }
}

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

async function toggleFullScreen(): Promise<void> {
  if (textContainerRef.value) {
    await textContainerRef.value.requestFullscreen();
  }
}
</script>

<style scoped lang="scss">
.text-container {
  padding: 1rem;
  box-shadow: var(--parsec-shadow-light);
  width: 100%;
  height: 100%;
}

.text-error {
  width: 100%;
}
</style>
