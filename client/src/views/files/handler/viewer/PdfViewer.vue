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
        ref="pdfContainer"
        v-if="pdf"
        @scroll="onScroll"
      >
        <ms-spinner v-show="loading" />
        <canvas
          class="canvas"
          ref="canvas"
          v-show="!loading && !error"
          v-for="numPage in pdf.numPages"
          :key="numPage"
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
          @change="onPaginationChange"
          ref="pagination"
          :page="currentPage"
        />
        <file-controls-button
          class="file-controls-fullscreen"
          @click="toggleFullScreen"
          :icon="scan"
        />
      </file-controls>
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import { FileControls, FileControlsButton, FileControlsPagination, FileControlsZoom } from '@/components/files/handler/viewer';
import { isWeb } from '@/parsec';
import { FileViewerWrapper } from '@/views/files/handler/viewer';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import { scan } from 'ionicons/icons';
import { I18n, MsReportText, MsReportTheme, MsSpinner } from 'megashark-lib';
import * as pdfjs from 'pdfjs-dist';
import { nextTick, onMounted, ref, Ref, shallowRef, useTemplateRef } from 'vue';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const loading = ref(true);
const error = ref('');
const currentPage = ref(1);
const pdf: Ref<pdfjs.PDFDocumentProxy | null> = shallowRef(null);
const pdfContainerRef = useTemplateRef<HTMLDivElement>('pdfContainer');
const zoomControlRef = useTemplateRef<InstanceType<typeof FileControlsZoom>>('zoomControl');
const scale = ref(1);
const canvasRef = useTemplateRef<HTMLCanvasElement[]>('canvas');
const isRendering = ref(false);
const enum CanvasStates {
  Rendered = 'rendered',
  Failed = 'failed',
}
const CanvasStateAttribute = 'data-canvas-state';

onMounted(async () => {
  loading.value = true;
  scale.value = (zoomControlRef.value?.getZoom() ?? 100) / 100;

  try {
    pdf.value = await pdfjs.getDocument({
      data: props.contentInfo.data,
      wasmUrl: import.meta.env.BASE_URL !== '/' ? `${import.meta.env.BASE_URL}/pdfjs/` : 'pdfjs/',
    }).promise;
    await loadPages();
    await renderPage(1);
  } catch (e: any) {
    window.electronAPI.log('error', `Failed to parse PDF: ${e}`);
    error.value = 'fileViewers.pdf.loadDocumentError';
  } finally {
    loading.value = false;
  }
});

function isInViewport(canvasElement: HTMLCanvasElement): boolean {
  const rect = canvasElement.getBoundingClientRect();
  const pdfContainerRect = pdfContainerRef.value!.getBoundingClientRect();
  return rect.bottom >= pdfContainerRect.top && rect.top <= pdfContainerRect.bottom;
}

function isCanvasRendered(canvasElement: HTMLCanvasElement): boolean {
  return canvasElement.getAttribute(CanvasStateAttribute) === CanvasStates.Rendered;
}

function isCanvasOnError(canvasElement: HTMLCanvasElement): boolean {
  return canvasElement.getAttribute(CanvasStateAttribute) === CanvasStates.Failed;
}

async function loadPage(pageIndex: number): Promise<void> {
  if (!pdf.value) {
    return;
  }

  error.value = '';
  loading.value = true;

  try {
    const page = await pdf.value.getPage(pageIndex);
    if (window.isDev() && isWeb() && pageIndex === 4) {
      throw new Error('Failed to load page');
    }

    const canvasElement = canvasRef.value?.at(pageIndex - 1);
    if (!canvasElement) {
      return;
    }
    const viewport = page.getViewport({ scale: scale.value });
    drawBlankCanvas(canvasElement, viewport);
  } catch (e: any) {
    const canvasElement = canvasRef.value?.at(pageIndex - 1);
    if (!canvasElement) {
      return;
    }
    drawErrorCanvas(canvasElement);
    window.electronAPI.log('error', `Failed to load PDF page: ${e}`);
  } finally {
    loading.value = false;
  }
}

function drawBlankCanvas(canvasElement: HTMLCanvasElement, viewport: pdfjs.PageViewport): void {
  const outputScale = window.devicePixelRatio || 1;
  canvasElement.width = viewport.width * outputScale;
  canvasElement.height = viewport.height * outputScale;
  canvasElement.style.width = `${Math.floor(viewport.width)}px`;
  canvasElement.style.height = `${Math.floor(viewport.height)}px`;

  canvasElement.removeAttribute(CanvasStateAttribute);
  canvasElement.classList.remove('error');
}

function drawErrorCanvas(canvasElement: HTMLCanvasElement): void {
  canvasElement.width = 300;
  canvasElement.height = 60;
  canvasElement.style.width = '300px';
  canvasElement.style.height = '60px';
  canvasElement.setAttribute(CanvasStateAttribute, CanvasStates.Failed);
  canvasElement.classList.add('error');

  const context = canvasElement.getContext('2d', { willReadFrequently: true });
  if (context) {
    const errorMessage = I18n.translate('fileViewers.pdf.loadPageError');
    context.font = '14px "Albert Sans"';

    // Calculate the width and height of the text
    const textMetrics = context.measureText(errorMessage);
    const textWidth = textMetrics.width;
    const textHeight = 14; // Approximate height based on font size

    // Calculate the position to center the text
    const x = (canvasElement.width - textWidth) / 2;
    const y = (canvasElement.height + textHeight) / 2;

    // Draw the text
    context.fillText(errorMessage, x, y);
  }
}

async function loadPages(): Promise<void> {
  if (!pdf.value) {
    return;
  }

  for (let i = 1; i <= pdf.value.numPages; i++) {
    await loadPage(i);
  }
}

async function renderPage(pageNumber: number): Promise<void> {
  const canvasElement = canvasRef.value?.at(pageNumber - 1);
  if (isRendering.value || !canvasElement || isCanvasRendered(canvasElement) || isCanvasOnError(canvasElement)) {
    return;
  }

  isRendering.value = true;

  try {
    const page = await pdf.value!.getPage(pageNumber);
    if (!page) {
      return;
    }

    const viewport = page.getViewport({ scale: scale.value });
    const outputScale = window.devicePixelRatio || 1;
    const context = canvasElement.getContext('2d', { willReadFrequently: true });
    const renderContext = {
      canvas: canvasElement,
      canvasContext: context!,
      transform: outputScale !== 1 ? [outputScale, 0, 0, outputScale, 0, 0] : undefined,
      viewport: viewport,
    };

    await page.render(renderContext).promise;
    canvasElement.setAttribute(CanvasStateAttribute, CanvasStates.Rendered);
  } catch (e: any) {
    const canvasElement = canvasRef.value?.at(pageNumber - 1);
    if (!canvasElement) {
      return;
    }
    drawErrorCanvas(canvasElement);
    window.electronAPI.log('error', `Failed to render PDF page: ${e}`);
  } finally {
    isRendering.value = false;
  }
}

async function onZoomLevelChange(value: number): Promise<void> {
  scale.value = value / 100;
  await loadPages();
  await renderPage(currentPage.value);
}

async function onPaginationChange(pageIndex: number): Promise<void> {
  if (!pdf.value || pageIndex <= 0 || pageIndex > pdf.value.numPages) {
    return;
  }

  const targetPage = canvasRef.value?.at(pageIndex - 1);
  if (targetPage) {
    await nextTick();
    targetPage.scrollIntoView({ behavior: 'smooth' });
  }
  currentPage.value = pageIndex;
}

async function onScroll(): Promise<void> {
  if (!pdf.value) {
    return;
  }

  const pdfContainerRect = pdfContainerRef.value!.getBoundingClientRect();
  let currentPageIndex = -1;
  // minDistance is initialized to Infinity to ensure that any distance will be less than it
  let minDistance = Infinity;

  for (const [pIndex, page] of [...(canvasRef.value || [])].entries()) {
    const pageRect = page.getBoundingClientRect();
    if (isInViewport(page)) {
      await renderPage(pIndex + 1);
    }
    // here we calculate the distance between the top of the page and the top of the document content
    const distance = Math.abs(pageRect.top - pdfContainerRect.top);
    // if the distance is less than the minimum distance, we update the minimum distance and the current page index
    if (distance < minDistance) {
      minDistance = distance;
      currentPageIndex = pIndex;
    } else {
      // if the distance is greater than the minimum distance, we break the loop
      break;
    }
  }
  if (currentPageIndex >= 0) {
    currentPage.value = currentPageIndex + 1;
  }
}

async function toggleFullScreen(): Promise<void> {
  if (document.fullscreenElement) {
    await document.exitFullscreen();
    return;
  }
  await pdfContainerRef.value!.requestFullscreen();
}
</script>

<style scoped lang="scss">
.pdf-container {
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

  & * {
    transition: all 0.3s ease-in-out;
  }
}

.pdf-error {
  width: 100%;
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.canvas {
  display: block;
  margin: 0 auto;
  background: var(--parsec-color-light-secondary-white);
  box-shadow: var(--parsec-shadow-light);

  &.error {
    border-left: 3px solid var(--parsec-color-light-danger-700);
    background: var(--parsec-color-light-danger-50);
    color: var(--parsec-color-light-secondary-text);
    text-align: center;
  }
}
</style>
