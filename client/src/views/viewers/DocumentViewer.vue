<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper>
    <template #viewer>
      <ms-spinner v-show="loading" />
      <div
        ref="document"
        v-show="!loading"
        class="document-content"
        v-html="htmlContent"
      />
    </template>
    <template #controls>
      <file-controls>
        <file-controls-fullscreen @click="toggleFullScreen" />
      </file-controls>
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { FileViewerWrapper } from '@/views/viewers';
import { FileContentInfo } from '@/views/viewers/utils';
import { FileControls, FileControlsFullscreen } from '@/components/viewers';
import mammoth from 'mammoth';
import { MsSpinner } from 'megashark-lib';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const loading = ref(true);
const htmlContent = ref('');
const document = ref();

onMounted(async () => {
  loading.value = true;
  const result = await mammoth.convertToHtml({ arrayBuffer: props.contentInfo.data.buffer });
  htmlContent.value = result.value;
  loading.value = false;
});

async function toggleFullScreen(): Promise<void> {
  await document.value?.requestFullscreen();
}
</script>

<style scoped lang="scss"></style>
