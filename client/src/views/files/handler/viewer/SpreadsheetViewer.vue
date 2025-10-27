<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper>
    <template #viewer>
      <ms-spinner
        id="spreadsheet-spinner"
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
        <file-controls-group class="file-controls-spreadsheet-container">
          <file-controls-dropdown
            :items="dropdownItems"
            :title="I18n.valueAsTranslatable(currentSheet)"
            :icon="chevronDown"
            class="spreadsheet-dropdown-button"
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
import {
  FileControls,
  FileControlsButton,
  FileControlsDropdown,
  FileControlsDropdownItemContent,
  FileControlsGroup,
  FileControlsZoom,
} from '@/components/files/handler/viewer';
import { FileViewerWrapper } from '@/views/files/handler/viewer';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import RevoGrid from '@revolist/vue3-datagrid';
import { chevronDown, scan } from 'ionicons/icons';
import { I18n, MsReportText, MsReportTheme, MsSpinner, Translatable } from 'megashark-lib';
import { onMounted, onUnmounted, ref, useTemplateRef } from 'vue';
import XLSX from 'xlsx';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

let workbook: XLSX.WorkBook | null = null;
const pages = ref<Array<string>>([]);
const currentSheet = ref('');
const loading = ref(true);
const error = ref('');
let documentWorker: Worker;
let pageWorker: Worker;
const zoomLevel = ref(1);
const rows = ref<Array<any>>([]);
const columns = ref<Array<{ prop: string; name: string }>>([]);
const loadingLabel = ref<Translatable>('');
const gridContainerRef = useTemplateRef<HTMLDivElement>('gridContainer');
const dropdownItems = ref<FileControlsDropdownItemContent[]>([]);

onMounted(async () => {
  documentWorker = new Worker(new URL('@/views/files/handler/viewer/workers/spreadsheet_document_loader.ts', import.meta.url));
  pageWorker = new Worker(new URL('@/views/files/handler/viewer/workers/spreadsheet_page_loader.ts', import.meta.url));

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
    pages.value.forEach((page, index) => {
      dropdownItems.value.push({
        label: I18n.valueAsTranslatable(page),
        callback: () => switchToPage(page),
        isActive: index === 0,
        dismissPopover: true,
      });
    });
    await switchToPage(workbook.SheetNames[0]);
  };

  pageWorker.onmessage = async function (e: MessageEvent): Promise<void> {
    const data: Array<any> = e.data.content;
    currentSheet.value = e.data.sheetName;
    const currentIndex = pages.value.indexOf(currentSheet.value);
    dropdownItems.value.forEach((item, index) => (item.isActive = index === currentIndex));

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

async function switchToPage(sheet: string): Promise<void> {
  if (!workbook) {
    return;
  }
  if (currentSheet.value === sheet) {
    return;
  }
  const ws = workbook.Sheets[sheet];
  rows.value = [];
  columns.value = [];
  if (!ws) {
    error.value = 'fileViewers.spreadsheet.loadSheetError';
    currentSheet.value = '';
    return;
  }
  loading.value = true;
  loadingLabel.value = { key: 'fileViewers.spreadsheet.loadingSheet', data: { page: sheet } };
  error.value = '';
  pageWorker.postMessage({ sheetName: sheet, sheet: ws });
}

function onZoomLevelChange(value: number): void {
  zoomLevel.value = value / 100;
}

async function toggleFullScreen(): Promise<void> {
  if (gridContainerRef.value) {
    await gridContainerRef.value.requestFullscreen();
  }
}
</script>

<style scoped lang="scss">
.spreadsheet-container {
  background-color: var(--parsec-color-light-secondary-inversed-contrast);
  width: 100%;
  height: 100%;
  overflow-y: auto;
}

.spreadsheet-content {
  transition: all 0.3s ease-in-out;
  transform: scale(v-bind(zoomLevel));
  transform-origin: top left;
  margin: auto;
  height: 100%;
  width: 100%;
}

.spreadsheet-error {
  width: 100%;
}

.file-controls-spreadsheet-container {
  gap: 1rem;
}

.spreadsheet-dropdown-button {
  padding: 0.25rem 0.5rem;
}
</style>
