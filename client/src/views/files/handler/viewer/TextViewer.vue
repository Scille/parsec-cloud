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
      <div class="text-layout">
        <div
          class="text-container"
          ref="textContainer"
          @pointerenter="activePane = 'editor'"
          @pointerdown="activePane = 'editor'"
        >
          <ms-spinner v-show="loading" />
        </div>
        <!-- eslint-disable vue/no-v-html -->
        <!--
          Fine here: we're converting from a .md (with html tags disabled),
          and then sanitizing the output with DOMPurify.
          Injecting something here would require some big bugs in both the conversion
          AND the sanitization.
        -->
        <div
          class="preview"
          v-if="markdownIt && isLargeDisplay"
          ref="preview"
          tabindex="0"
          v-html="markdownPreview"
          @pointerenter="activePane = 'preview'"
          @pointerdown="activePane = 'preview'"
          @keydown="activePane = 'preview'"
          @scroll="activePane === 'preview' && syncEditorScroll()"
        />
        <!-- eslint-enable vue/no-v-html -->
      </div>
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import ImagePlaceholder from '@/assets/images/image.svg';
import { getFileContent } from '@/common/file';
import { closeFile, openFile, writeFile } from '@/parsec';
import { SaveState } from '@/views/files/handler/types';
import { FileViewerWrapper } from '@/views/files/handler/viewer';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import DOMPurify from 'dompurify';
import MarkdownIt, { type MarkdownIt as MarkdownItType } from 'markdown-it';
import { I18n, MsReportText, MsReportTheme, MsSpinner, useWindowSize } from 'megashark-lib';
import * as monaco from 'monaco-editor';
import { computed, nextTick, onMounted, onUnmounted, ref, useTemplateRef } from 'vue';

const props = defineProps<{
  contentInfo: FileContentInfo;
  readOnly: boolean;
}>();

const SAVE_INTERVAL = 5000;
const PREVIEW_DEBOUNCE = 500;

const { isLargeDisplay } = useWindowSize();

const loading = ref(true);
const error = ref('');
const containerRef = useTemplateRef<HTMLDivElement>('textContainer');
const previewRef = useTemplateRef<HTMLDivElement>('preview');

let editor: monaco.editor.IStandaloneCodeEditor | undefined = undefined;

let subscription: monaco.IDisposable | undefined = undefined;
let scrollSub: monaco.IDisposable | undefined = undefined;
let keyDownSub: monaco.IDisposable | undefined = undefined;

let saveTimeout: any = undefined;
let previewTimeout: any = undefined;

const markdownIt = ref<MarkdownItType | undefined>(undefined);
const markdownPreview = ref<string>('');

// The pane the user is interacting with
const activePane = ref<'editor' | 'preview'>('editor');

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

defineExpose({
  saveDocument,
});

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
      contextmenu: false,
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
    if (props.contentInfo.extension === 'md') {
      markdownIt.value = new MarkdownIt({
        html: false, // Explicitly not authorizing HTML tags
      });
      markdownIt.value.renderer.rules.image = (): string => {
        const placeholder = I18n.translate('fileViewers.text.imagePlaceholder');
        return `<img class="image-placeholder" src="${ImagePlaceholder}" alt="${placeholder}" title="${placeholder}" />`;
      };
      markdownIt.value.core.ruler.push('source_line', (state) => {
        for (const token of state.tokens) {
          // Opening block tokens (paragraph, heading, list item, table row, fence, hr...)
          if (token.map && token.nesting !== -1) {
            token.attrSet('data-line', String(token.map[0]));
          }
        }
      });
      await buildPreview();
      scrollSub = editor.onDidScrollChange((e) => e.scrollTopChanged && activePane.value === 'editor' && syncPreviewScroll());
      keyDownSub = editor.onKeyDown(() => (activePane.value = 'editor'));
    }

    if (!isReadOnly.value) {
      subscription = editor.onDidChangeModelContent((_event: monaco.editor.IModelContentChangedEvent) => {
        isDirty.value = true;
        if (previewTimeout) {
          clearTimeout(previewTimeout);
          previewTimeout = undefined;
        }
        if (saveTimeout) {
          clearTimeout(saveTimeout);
          saveTimeout = undefined;
        }
        previewTimeout = setTimeout(() => {
          clearTimeout(previewTimeout);
          previewTimeout = undefined;
          buildPreview();
        }, PREVIEW_DEBOUNCE);
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
  if (previewTimeout) {
    clearTimeout(previewTimeout);
  }
  if (isDirty.value) {
    await saveDocument();
  }
  if (subscription) {
    subscription.dispose();
  }
  if (scrollSub) {
    scrollSub.dispose();
  }
  if (keyDownSub) {
    keyDownSub.dispose();
  }
  if (editor) {
    editor.dispose();
  }
});

async function buildPreview(): Promise<void> {
  if (!markdownIt.value || !editor) {
    return;
  }
  markdownPreview.value = DOMPurify.sanitize(markdownIt.value.render(editor.getValue()));
  await nextTick();
  syncPreviewScroll();
}

// Elements of the preview tagged with the source line they come from, in document order
function getAnchors(preview: HTMLElement): Array<{ line: number; top: number }> {
  const previewTop = preview.getBoundingClientRect().top - preview.scrollTop;
  return Array.from(preview.querySelectorAll<HTMLElement>('[data-line]')).map((el) => {
    return { line: Number(el.dataset.line), top: el.getBoundingClientRect().top - previewTop };
  });
}

function isEditorAtBottom(editor: monaco.editor.IStandaloneCodeEditor): boolean {
  return editor.getScrollTop() + editor.getLayoutInfo().height >= editor.getScrollHeight() - 1;
}

function isPreviewAtBottom(preview: HTMLElement): boolean {
  return preview.scrollTop + preview.clientHeight >= preview.scrollHeight - 1;
}

function syncPreviewScroll(): void {
  if (!editor || !previewRef.value) {
    return;
  }
  const preview = previewRef.value;
  const scrollTop = editor.getScrollTop();
  if (scrollTop === 0) {
    preview.scrollTop = 0;
    return;
  }
  // Interpolation never matches the end exactly, snap the ends together
  if (isEditorAtBottom(editor)) {
    preview.scrollTop = preview.scrollHeight;
    return;
  }
  // Fractional source line at the top of the editor, 0-based like token.map.
  // getTopForLineNumber accounts for word wrap.
  const line = editor.getVisibleRanges()[0]?.startLineNumber ?? 1;
  const lineTop = editor.getTopForLineNumber(line);
  const lineHeight = editor.getTopForLineNumber(line + 1) - lineTop || 1;
  const sourceLine = line - 1 + (scrollTop - lineTop) / lineHeight;

  const anchors = getAnchors(preview);
  const nextIndex = anchors.findIndex((anchor) => anchor.line > sourceLine);
  const previous = nextIndex === -1 ? anchors.at(-1) : anchors[nextIndex - 1];
  const next = nextIndex === -1 ? undefined : anchors[nextIndex];
  if (!previous) {
    preview.scrollTop = 0;
  } else if (!next) {
    preview.scrollTop = previous.top;
  } else {
    const ratio = (sourceLine - previous.line) / (next.line - previous.line);
    preview.scrollTop = previous.top + (next.top - previous.top) * ratio;
  }
}

function syncEditorScroll(): void {
  if (!editor || !previewRef.value) {
    return;
  }
  const preview = previewRef.value;
  const scrollTop = preview.scrollTop;
  if (scrollTop === 0) {
    editor.setScrollTop(0, monaco.editor.ScrollType.Immediate);
    return;
  }
  if (isPreviewAtBottom(preview)) {
    editor.setScrollTop(editor.getScrollHeight(), monaco.editor.ScrollType.Immediate);
    return;
  }
  // Fractional source line at the top of the preview
  const anchors = getAnchors(preview);
  const nextIndex = anchors.findIndex((anchor) => anchor.top > scrollTop);
  const previous = nextIndex === -1 ? anchors.at(-1) : anchors[nextIndex - 1];
  const next = nextIndex === -1 ? undefined : anchors[nextIndex];
  if (!previous) {
    editor.setScrollTop(0, monaco.editor.ScrollType.Immediate);
    return;
  }
  let sourceLine = previous.line;
  // Several anchors can share the same position (a list item and its paragraph)
  if (next && next.top > previous.top) {
    sourceLine += ((next.line - previous.line) * (scrollTop - previous.top)) / (next.top - previous.top);
  }
  // Back to 1-based Monaco lines, keeping the fractional part.
  // getTopForLineNumber accounts for word wrap.
  const line = Math.floor(sourceLine) + 1;
  const lineTop = editor.getTopForLineNumber(line);
  const lineHeight = editor.getTopForLineNumber(line + 1) - lineTop;
  editor.setScrollTop(lineTop + (sourceLine - Math.floor(sourceLine)) * lineHeight, monaco.editor.ScrollType.Immediate);
}

async function saveDocument(): Promise<boolean> {
  if (!editor || saving.value || !isDirty.value) {
    return true;
  }
  saving.value = true;
  emits('onSaveStateChange', SaveState.Saving);
  const openResult = await openFile(props.contentInfo.workspaceHandle, props.contentInfo.path, { write: true, truncate: true });

  if (!openResult.ok) {
    emits('onSaveStateChange', SaveState.Error);
    return false;
  }
  try {
    const raw = new TextEncoder().encode(editor.getValue());
    const writeResult = await writeFile(props.contentInfo.workspaceHandle, openResult.value, 0, raw);
    if (!writeResult.ok) {
      emits('onSaveStateChange', SaveState.Error);
      return false;
    }
    emits('onSaveStateChange', SaveState.Saved);
    return true;
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
.text-layout {
  display: flex;
  width: 100%;
  height: 100%;
}

.text-container {
  background-color: var(--parsec-color-light-secondary-premiere);
  flex: 1 1 0;
  min-width: 0;
  height: 100%;
}

.preview {
  background-color: var(--parsec-color-light-secondary-premiere);
  flex: 1 1 0;
  min-width: 0;
  height: 100%;
  overflow-y: auto;
  padding: 1rem 1.5rem;
  border-left: 1px solid var(--parsec-color-light-secondary-medium);
  box-sizing: border-box;

  color: var(--parsec-color-light-secondary-text);
  line-height: 1.5;

  // Scoped styles don't reach v-html content, hence :deep
  :deep(h1),
  :deep(h2) {
    padding-bottom: 0.25rem;
    border-bottom: 1px solid var(--parsec-color-light-secondary-medium);
  }

  :deep(a) {
    color: var(--parsec-color-light-primary-600);
  }

  :deep(code) {
    font-family: monospace;
    font-size: 0.9em;
    padding: 0.125rem 0.25rem;
    border-radius: var(--parsec-radius-4);
    background-color: var(--parsec-color-light-secondary-medium);
  }

  :deep(pre) {
    overflow-x: auto;
    padding: 0.75rem 1rem;
    border-radius: var(--parsec-radius-8);
    background-color: var(--parsec-color-light-secondary-medium);

    // Already styled by the `pre`
    code {
      padding: 0;
      background-color: transparent;
    }
  }

  :deep(blockquote) {
    margin: 1rem 0;
    padding: 0 1rem;
    border-left: 4px solid var(--parsec-color-light-secondary-light);
    color: var(--parsec-color-light-secondary-grey);
  }

  :deep(table) {
    border-collapse: collapse;
    margin: 1rem 0;
  }

  :deep(th),
  :deep(td) {
    padding: 0.375rem 0.75rem;
    border: 1px solid var(--parsec-color-light-secondary-medium);
  }

  :deep(th) {
    background-color: var(--parsec-color-light-secondary-background);
  }

  :deep(hr) {
    border: none;
    border-top: 1px solid var(--parsec-color-light-secondary-medium);
  }

  :deep(.image-placeholder) {
    width: 4rem;
    height: 4rem;
  }
}
</style>
