<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper>
    <template #viewer>
      <ms-report-text
        class="spreadsheet-error"
        :theme="MsReportTheme.Error"
        v-if="error"
      >
        {{ $msTranslate(error) }}
      </ms-report-text>
      <ms-spinner v-show="loading" />
      <div
        ref="spreadsheet"
        v-show="!loading"
        class="spreadsheet-content"
        v-html="htmlContent"
      />
    </template>
    <template #controls>
      <file-controls v-show="!loading && workbook">
        <file-controls-button
          v-for="(action, key) in actions"
          :key="key"
          @click="action.handler"
          :label="action.text"
        />
        <file-controls-button
          @click="toggleFullScreen"
          :icon="scan"
        />
      </file-controls>
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import { MsSpinner, I18n, Translatable, MsReportText, MsReportTheme } from 'megashark-lib';
import { onBeforeMount, onMounted, Ref, ref } from 'vue';
import XLSX from 'xlsx';
import { FileControls, FileControlsButton } from '@/components/viewers';
import { FileViewerWrapper } from '@/views/viewers';
import { FileContentInfo } from '@/views/viewers/utils';
import { scan } from 'ionicons/icons';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

let workbook: XLSX.WorkBook | null = null;
const htmlContent = ref('');
const pages = ref<Array<string>>([]);
const currentPage = ref('');
const loading = ref(true);
const error = ref('');
let worker: Worker | null = null;
const actions: Ref<Array<{ icon?: string; text?: Translatable; handler: () => void }>> = ref([]);
const spreadsheet = ref();

onMounted(async () => {
  try {
    workbook = XLSX.read(props.contentInfo.data, { type: 'array' });
    pages.value = workbook.SheetNames;
    await switchToPage(workbook.SheetNames[0]);
    for (const page of pages.value) {
      actions.value.push({ text: I18n.valueAsTranslatable(page), handler: () => switchToPage(page) });
    }
  } catch (e: any) {
    window.electronAPI.log('error', `Failed to load spreadsheet document: ${e}`);
    error.value = 'fileViewers.spreadsheet.loadDocumentError';
    loading.value = false;
  }
});

onBeforeMount(async () => {
  if (worker) {
    worker.terminate();
  }
});

async function switchToPage(page: string): Promise<void> {
  if (!workbook) {
    return;
  }
  const ws = workbook.Sheets[page];
  if (!ws) {
    error.value = 'fileViewers.spreadsheet.loadSheetError';
    currentPage.value = '';
    htmlContent.value = '';
    return;
  }
  loading.value = true;
  error.value = '';

  if (worker) {
    worker.terminate();
  }

  worker = new Worker(new URL('@/views/viewers/workers/spreadsheet_converter.ts', import.meta.url));
  worker.onmessage = async function (e: MessageEvent): Promise<void> {
    currentPage.value = page;
    htmlContent.value = e.data as string;
    loading.value = false;
    worker = null;
  };
  worker.postMessage(ws);
}

async function toggleFullScreen(): Promise<void> {
  await spreadsheet.value.requestFullscreen();
}
</script>

<style scoped lang="scss">
.spreadsheet-content {
  width: 100%;
  max-height: 100%;
  overflow: auto;

  :deep(table) {
    width: 100%;

    tr {
      background-color: #eeeeee;

      td {
        border: 1px solid black;
      }
    }
  }

  &:fullscreen {
    padding: 5rem;
    align-items: center;
    height: 100%;
    background: var(--parsec-color-light-secondary-background);
  }
}

.spreadsheet-error {
  width: 100%;
}
</style>
