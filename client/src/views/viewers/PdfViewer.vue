<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper>
    <template #viewer>
      <ms-report-text
        class="pdf-error"
        v-show="error"
        :theme="MsReportTheme.Error"
      >
        {{ $msTranslate(error) }}
      </ms-report-text>
      <div
        class="pdf-container"
        v-if="pdf"
        ref="pdfRef"
      >
        <ms-spinner v-show="loading" />
        <canvas
          v-show="!loading && !error"
          class="canvas"
          ref="canvas"
        />
      </div>
    </template>
    <template #controls>
      <file-controls v-show="pdf">
        <file-controls-zoom
          @change="onZoomLevelChange"
          ref="zoomControl"
        />
        <file-controls-pagination
          v-if="pdf"
          :length="pdf.numPages"
          @change="loadPage"
          ref="pagination"
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
import { onMounted, ref, Ref, shallowRef } from 'vue';
import { FileContentInfo } from '@/views/viewers/utils';
import { FileViewerWrapper } from '@/views/viewers';
import { FileControls, FileControlsButton, FileControlsPagination, FileControlsZoom } from '@/components/viewers';
import { I18n, MsSpinner, Translatable, MsReportText, MsReportTheme } from 'megashark-lib';
import * as pdfjs from 'pdfjs-dist';
import { scan } from 'ionicons/icons';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const loading = ref(true);
const error = ref('');
const canvas = ref();
const pdfRef = ref();
const currentPage = ref(1);
const pdf: Ref<pdfjs.PDFDocumentProxy | null> = shallowRef(null);
const actions: Ref<Array<{ icon?: string; text?: Translatable; handler: () => void }>> = ref([]);
const zoomControl = ref();
const scale = ref(1);

onMounted(async () => {
  loading.value = true;
  scale.value = zoomControl.value.getZoom() / 100;

  try {
    pdf.value = await pdfjs.getDocument(props.contentInfo.data).promise;
    for (let i = 1; i <= pdf.value.numPages; i++) {
      actions.value.push({ text: I18n.valueAsTranslatable(`Page ${i.toString()}`), handler: () => loadPage(i) });
    }
    await loadPage(1);
  } catch (e: any) {
    window.electronAPI.log('error', `Failed to parse PDF: ${e}`);
    error.value = 'fileViewers.pdf.loadDocumentError';
  } finally {
    loading.value = false;
  }
});

async function onZoomLevelChange(value: number): Promise<void> {
  scale.value = value / 100;
  await loadPage(currentPage.value);
}

async function loadPage(pageIndex: number): Promise<void> {
  if (!pdf.value) {
    return;
  }

  error.value = '';
  loading.value = true;

  try {
    const page = await pdf.value.getPage(pageIndex);
    const viewport = page.getViewport({ scale: scale.value });
    const outputScale = window.devicePixelRatio || 1;

    canvas.value.width = viewport.width;
    canvas.value.height = viewport.height;
    canvas.value.style.width = `${Math.floor(viewport.width)}px`;
    canvas.value.style.height = `${Math.floor(viewport.height)}px`;
    const context = canvas.value.getContext('2d');
    const transform = outputScale !== 1 ? [outputScale, 0, 0, outputScale, 0, 0] : undefined;
    const renderContext = {
      canvasContext: context,
      transform: transform,
      viewport: viewport,
    };
    page.render(renderContext);
    currentPage.value = pageIndex;
  } catch (e: any) {
    window.electronAPI.log('error', `Failed to open PDF page: ${e}`);
    error.value = 'fileViewers.pdf.loadPageError';
  } finally {
    loading.value = false;
  }
}

async function toggleFullScreen(): Promise<void> {
  if (pdfRef.value?.fullscreenElement) {
    await pdfRef.value?.exitFullscreen();
    return;
  }
  await pdfRef.value?.requestFullscreen();
}
</script>

<style scoped lang="scss">
.pdf-container {
  width: 100%;
  max-height: 100%;
  display: flex;
  justify-content: center;

  & * {
    transition: all 0.3s ease-in-out;
  }

  &:fullscreen {
    align-items: center;
    height: 100%;
    background: var(--parsec-color-light-secondary-background);
  }
}

.pdf-error {
  width: 100%;
}
</style>
