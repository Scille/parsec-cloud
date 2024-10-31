<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="pages">
    <ion-button
      v-for="page in pages"
      @click="switchToPage(page)"
      :disabled="page === currentPage || loading"
      :key="page"
    >
      {{ page }}
    </ion-button>
  </div>
  <ms-spinner v-show="loading" />
  <div
    v-show="!loading"
    class="spreadsheet-content"
    v-html="htmlContent"
  />
</template>

<script setup lang="ts">
import { IonButton } from '@ionic/vue';
import { MsSpinner } from 'megashark-lib';
import { onBeforeMount, onMounted, ref } from 'vue';
import XLSX from 'xlsx';
import { FileContentInfo } from '@/views/viewers/utils';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

let workbook: XLSX.WorkBook | null = null;
const htmlContent = ref('');
const pages = ref<Array<string>>([]);
const currentPage = ref('');
const loading = ref(true);
let worker: Worker | null = null;

onMounted(async () => {
  workbook = XLSX.read(props.contentInfo.data, { type: 'array' });
  pages.value = workbook.SheetNames;
  await switchToPage(workbook.SheetNames[0]);
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
    window.electronAPI.log('error', 'Spreadsheet page not found');
    return;
  }
  loading.value = true;

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
</script>

<style scoped lang="scss">
.spreadsheet-content {
  width: 100%;
  height: 680px;
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
}
</style>
