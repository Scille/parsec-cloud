<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper>
    <template #viewer>
      <div
        class="pdf-container"
        v-if="pdf"
      >
        <ms-spinner v-show="loading" />
        <canvas
          v-show="!loading"
          class="canvas"
          ref="canvas"
        />
      </div>
    </template>
    <template #controls>
      <file-controls>
        <file-controls-zoom
          @change="onChange"
          ref="zoomControl"
        />
        <file-controls-pagination
          v-if="pdf"
          :length="pdf.numPages"
          @change="loadPage"
          ref="pagination"
        />
      </file-controls>
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import { inject, onMounted, ref, Ref, shallowRef } from 'vue';
import { FileContentInfo } from '@/views/viewers/utils';
import { FileViewerWrapper } from '@/views/viewers';
import { FileControls, FileControlsPagination, FileControlsZoom } from '@/components/viewers';
import { I18n, MsSpinner, Translatable } from 'megashark-lib';
import * as pdfjs from 'pdfjs-dist';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const loading = ref(true);
const canvas = ref();
const currentPage = ref(1);
const pdf: Ref<pdfjs.PDFDocumentProxy | null> = shallowRef(null);
const informationManager: InformationManager = inject(InformationManagerKey)!;
const actions: Ref<Array<{ icon?: string; text?: Translatable; handler: () => void }>> = ref([]);
const zoomControl = ref();
const scale = ref(1);

onMounted(async () => {
  loading.value = true;
  scale.value = zoomControl.value.getZoom() / 100;

  try {
    pdf.value = await pdfjs.getDocument(props.contentInfo.data).promise;
    await loadPage(1);
    for (let i = 1; i <= pdf.value.numPages; i++) {
      actions.value.push({ text: I18n.valueAsTranslatable(`Page ${i.toString()}`), handler: () => loadPage(i) });
    }
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

async function onChange(value: number): Promise<void> {
  scale.value = value / 100;
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
  max-height: 100%;
  display: flex;
  justify-content: center;

  & * {
    transition: all 0.3s ease-in-out;
  }
}
</style>
