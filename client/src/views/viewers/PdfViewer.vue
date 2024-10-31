<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="pdf-container"
    v-if="pdf"
  >
    <div :disabled="loading">
      <span
        class="zoom"
        @click="zoom(-0.2)"
      >
        ZOOM-
      </span>
      <span
        class="zoom"
        @click="zoom(0.2)"
      >
        ZOOM+
      </span>
      <span
        class="page"
        v-for="i in pdf.numPages"
        :key="i"
        :class="{ 'page-disabled': i === currentPage }"
        :disabled="i === currentPage"
        @click="loadPage(i)"
      >
        {{ i }}
      </span>
    </div>
    <ms-spinner v-show="loading" />
    <canvas
      v-show="!loading"
      class="canvas"
      ref="canvas"
    />
  </div>
</template>

<script setup lang="ts">
import { inject, onMounted, ref, Ref, shallowRef } from 'vue';
import { FileContentInfo } from '@/views/viewers/utils';
import { MsSpinner } from 'megashark-lib';
import * as pdfjs from 'pdfjs-dist';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const loading = ref(true);
const canvas = ref();
const scale = ref(1.0);
const currentPage = ref(1);
const pdf: Ref<pdfjs.PDFDocumentProxy | null> = shallowRef(null);
const informationManager: InformationManager = inject(InformationManagerKey)!;

onMounted(async () => {
  loading.value = true;

  try {
    pdf.value = await pdfjs.getDocument(props.contentInfo.data).promise;
    await loadPage(1);
  } catch (error: any) {
    window.electronAPI.log('error', `Failed to parse PDF: ${error}`);
    informationManager.present(
      new Information({
        message: 'FAILED TO PARSE PDF',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }

  loading.value = false;
});

async function zoom(factor: number): Promise<void> {
  scale.value += factor;
  await loadPage(currentPage.value);
}

async function loadPage(pageIndex: number): Promise<void> {
  if (!pdf.value) {
    return;
  }

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
  } catch (error: any) {
    window.electronAPI.log('error', `Failed to open PDF page: ${error}`);
    informationManager.present(
      new Information({
        message: 'FAILED TO PARSE PDF',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }

  loading.value = false;
}
</script>

<style scoped lang="scss">
.pdf-container {
  width: 100%;
  height: 600px;
}

.page {
  padding-right: 1em;
  cursor: pointer;
  user-select: none;
  font-weight: bold;

  &-disabled {
    color: grey;
    font-weight: normal;
  }
}

.zoom {
  cursor: pointer;
  user-select: none;
  font-weight: bold;
}
</style>
