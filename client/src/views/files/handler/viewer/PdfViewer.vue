<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper
    v-if="loading || error"
    :error="error"
  >
    <template #viewer>
      <ms-spinner />
    </template>
  </file-viewer-wrapper>
  <div
    class="pdf-renderer"
    v-if="content && !error"
    v-show="!loading"
  >
    <native-pdf-viewer
      v-if="useNativeViewer"
      :content="content"
      @loaded="loading = false"
      @error="onRendererError"
    />
    <pdfjs-viewer
      v-else
      :content="content"
      @loaded="loading = false"
      @error="onRendererError"
    />
  </div>
</template>

<script setup lang="ts">
import { getFileContent } from '@/common/file';
import { FileViewerWrapper, NativePdfViewer, PdfjsViewer } from '@/views/files/handler/viewer';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import { MsSpinner } from 'megashark-lib';
import { onMounted, ref, Ref, shallowRef } from 'vue';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const loading = ref(true);
const error = ref('');
const content: Ref<Uint8Array | null> = shallowRef(null);
const useNativeViewer = navigator.pdfViewerEnabled && !(window as any).TESTING_FORCE_PDFJS;

onMounted(async () => {
  try {
    const fileContent = await getFileContent(props.contentInfo.workspaceHandle, props.contentInfo.path, props.contentInfo.timestamp);
    if (!fileContent) {
      throw new Error('failed to load content');
    }
    content.value = fileContent;
  } catch (e: any) {
    window.nativeAPI.log('error', `Failed to load PDF content: ${e}`);
    onRendererError();
  }
});

function onRendererError(): void {
  error.value = 'fileViewers.pdf.loadDocumentError';
  loading.value = false;
}
</script>

<style scoped lang="scss">
.pdf-renderer {
  display: contents;
}
</style>
