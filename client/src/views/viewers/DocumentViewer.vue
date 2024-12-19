<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper>
    <template #viewer>
      <ms-spinner v-show="loading" />
      <div
        v-show="!loading"
        class="document-content"
        v-html="htmlContent"
      />
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { FileViewerWrapper } from '@/views/viewers';
import { FileContentInfo } from '@/views/viewers/utils';
import mammoth from 'mammoth';
import { MsSpinner } from 'megashark-lib';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const loading = ref(true);
const htmlContent = ref('');

onMounted(async () => {
  loading.value = true;
  const result = await mammoth.convertToHtml({
    arrayBuffer: props.contentInfo.data.buffer,
  }, {
    styleMap: [
      "br[type='page'] => hr.page-break",
    ],
  });
  const pattern = '<hr class=\"page-break\" />';
  const pages = result.value.split(pattern);
  pages.map((page, index) => {
    pages[index] = `<div class="page">${page}</div>`;
  });
  result.value = pages.join(pattern);
  htmlContent.value = result.value;
  loading.value = false;
});
</script>

<style scoped lang="scss"></style>
