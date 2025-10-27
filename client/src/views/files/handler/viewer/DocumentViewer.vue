<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper :error="error">
    <template #viewer>
      <ms-spinner v-show="loading" />
      <div
        class="document-container"
        @scroll="onScroll"
        ref="documentContainer"
        v-show="!loading"
      >
        <div
          class="document-content"
          ref="documentContent"
        />
      </div>
    </template>
    <template #controls>
      <file-controls v-show="!loading">
        <file-controls-zoom
          @change="onZoomLevelChange"
          ref="zoomControl"
        />
        <file-controls-pagination
          v-if="pages.length > 1"
          :length="pages.length"
          @change="onPaginationChange"
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
import { FileViewerWrapper } from '@/views/files/handler/viewer';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import { renderAsync } from 'docx-preview';
import { scan } from 'ionicons/icons';
import { MsSpinner } from 'megashark-lib';
import { nextTick, onMounted, ref, useTemplateRef } from 'vue';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const loading = ref(true);
const documentContainerRef = useTemplateRef<HTMLDivElement>('documentContainer');
const documentContentRef = useTemplateRef<HTMLDivElement>('documentContent');
const pages = ref<HTMLElement[]>([]);
const zoomLevel = ref(1);
const currentPage = ref(1);
const error = ref('');

onMounted(async () => {
  try {
    loading.value = true;
    await renderAsync(props.contentInfo.data.buffer, documentContentRef.value!, undefined, {
      ignoreLastRenderedPageBreak: false,
      className: 'docx-page',
    });

    // recommended way to get all pages by the library developer
    pages.value = Array.from(documentContentRef.value!.querySelectorAll('section.docx-page'));
    loading.value = false;
  } catch (e: any) {
    window.electronAPI.log('error', `Failed to load docx file: ${e}`);
    error.value = 'fileViewers.document.loadDocumentError';
    loading.value = false;
  }
});

function onZoomLevelChange(value: number): void {
  zoomLevel.value = value / 100;
}

async function onPaginationChange(pageIndex: number): Promise<void> {
  if (!pages.value || pageIndex <= 0 || pageIndex > pages.value.length) {
    return;
  }

  const targetPage = pages.value?.at(pageIndex - 1);
  if (targetPage) {
    await nextTick();
    targetPage.scrollIntoView({ behavior: 'smooth' });
  }
  currentPage.value = pageIndex;
}

function onScroll(): void {
  if (!pages.value) {
    return;
  }

  const documentContainerRect = documentContainerRef.value!.getBoundingClientRect();
  let currentPageIndex = -1;
  // minDistance is initialized to Infinity to ensure that any distance will be less than it
  let minDistance = Infinity;

  for (const [pIndex, page] of pages.value.entries()) {
    const pageRect = page.getBoundingClientRect();
    // here we calculate the distance between the top of the page and the top of the document content
    const distance = Math.abs(pageRect.top - documentContainerRect.top);
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
  await documentContainerRef.value!.requestFullscreen();
}
</script>

<style scoped lang="scss">
.document-container {
  background-color: var(--parsec-color-light-secondary-premiere);
  width: 100%;
  height: 100%;
  overflow-y: auto;

  &:fullscreen {
    align-items: center;
    height: 100%;
  }
}

.document-content {
  transition: all 0.3s ease-in-out;
  transform: scale(v-bind(zoomLevel));
  transform-origin: top left;
  width: fit-content;
  margin: auto;
  height: 100%;
}
</style>
