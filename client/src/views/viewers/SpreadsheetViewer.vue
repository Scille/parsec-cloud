<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper>
    <template #viewer>
      <ms-spinner
        :title="loadingLabel"
        v-show="loading"
      />
      <ms-report-text
        class="spreadsheet-error"
        :theme="MsReportTheme.Error"
        v-if="error"
      >
        {{ $msTranslate(error) }}
      </ms-report-text>
      <div
        class="spreadsheet-container"
        v-show="!loading && !error"
        ref="gridContainer"
      >
        <revo-grid
          class="spreadsheet-content"
          :source="rows"
          :columns="columns"
          hide-attribution
          :readonly="true"
          theme="material"
          :row-headers="true"
          :resize="true"
        />
      </div>
    </template>
    <template #controls>
      <file-controls v-show="!loading && workbook">
        <file-controls-group>
          <file-controls-button
            v-for="(action, key) in actions"
            :key="key"
            @click="action.handler"
            :label="action.text"
          />
        </file-controls-group>
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
import { MsSpinner, I18n, Translatable, MsReportText, MsReportTheme } from 'megashark-lib';
import { onMounted, onUnmounted, Ref, ref } from 'vue';
import XLSX from 'xlsx';
import { FileControls, FileControlsButton, FileControlsGroup, FileControlsZoom } from '@/components/viewers';
import { FileViewerWrapper } from '@/views/viewers';
import { FileContentInfo } from '@/views/viewers/utils';
import { scan } from 'ionicons/icons';
import RevoGrid from '@revolist/vue3-datagrid';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

let workbook: XLSX.WorkBook | null = null;
const pages = ref<Array<string>>([]);
const currentPage = ref('');
const loading = ref(true);
const error = ref('');
let documentWorker: Worker;
let pageWorker: Worker;
const actions: Ref<Array<{ icon?: string; text?: Translatable; handler: () => void }>> = ref([]);
const zoomControl = ref();
const zoomLevel = ref(1);
const rows = ref<Array<any>>([]);
const columns = ref<Array<{ prop: string; name: string }>>([]);
const loadingLabel = ref<Translatable>('');
const gridContainer = ref();

onMounted(async () => {
  documentWorker = new Worker(new URL('@/views/viewers/workers/spreadsheet_document_loader.ts', import.meta.url));
  pageWorker = new Worker(new URL('@/views/viewers/workers/spreadsheet_page_loader.ts', import.meta.url));

  loading.value = true;
  loadingLabel.value = 'fileViewers.spreadsheet.loadingDocument';
  pages.value = [];
  rows.value = [];
  columns.value = [];
  workbook = null;

  documentWorker.onmessage = async function (e: MessageEvent): Promise<void> {
    const result: { ok: boolean; error: any; value: XLSX.WorkBook } = e.data;
    if (!result.ok) {
      window.electronAPI.log('error', `Failed to load spreadsheet document: ${e.data.error.toString()}`);
      error.value = 'fileViewers.spreadsheet.loadDocumentError';
      loading.value = false;
      return;
    }
    workbook = e.data.value as XLSX.WorkBook;
    pages.value = workbook.SheetNames;
    for (const page of pages.value) {
      actions.value.push({ text: I18n.valueAsTranslatable(page), handler: () => switchToPage(page) });
    }
    await switchToPage(workbook.SheetNames[0]);
  };

  pageWorker.onmessage = async function (e: MessageEvent): Promise<void> {
    currentPage.value = e.data.page;
    const data: Array<any> = e.data.content;

    columns.value = [];
    if (data.length > 0) {
      for (const name of Object.keys(data[0])) {
        columns.value.push({ prop: name, name: name });
      }
    }
    rows.value = data;
    loading.value = false;
    loadingLabel.value = '';
  };

  documentWorker.postMessage(props.contentInfo.data);
});

onUnmounted(() => {
  documentWorker.terminate();
  pageWorker.terminate();
});

async function switchToPage(page: string): Promise<void> {
  if (!workbook) {
    return;
  }
  const ws = workbook.Sheets[page];
  rows.value = [];
  columns.value = [];
  if (!ws) {
    error.value = 'fileViewers.spreadsheet.loadSheetError';
    currentPage.value = '';
    return;
  }
  loading.value = true;
  loadingLabel.value = { key: 'fileViewers.spreadsheet.loadingSheet', data: { page: page } };
  error.value = '';
  pageWorker.postMessage(ws);
}

function onZoomLevelChange(value: number): void {
  zoomLevel.value = value / 100;
}

async function toggleFullScreen(): Promise<void> {
  if (gridContainer.value) {
    await gridContainer.value.requestFullscreen();
  }
}
</script>

<style scoped lang="scss">
.spreadsheet-container {
  background-color: grey;
  width: 100%;
  height: 100%;
  overflow-y: auto;
}

.spreadsheet-content {
  transform: scale(v-bind(zoomLevel));
  transform-origin: top left;
  margin: auto;
  height: 100%;
  width: 100%;

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
