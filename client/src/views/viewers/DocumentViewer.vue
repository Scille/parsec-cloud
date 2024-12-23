<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper>
    <template #viewer>
      <ms-spinner v-show="loading" />
      <!-- <div
        v-show="!loading"
        class="document-content"
        v-html="htmlContent"
      /> -->
      <div
        v-show="!loading"
        class="document-content"
        ref="documentContent"
      />
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { FileViewerWrapper } from '@/views/viewers';
import { FileContentInfo } from '@/views/viewers/utils';
import mammoth from 'mammoth';
import { parseAsync, renderAsync, renderDocument } from 'docx-preview';
import { MsSpinner } from 'megashark-lib';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const loading = ref(true);
const htmlContent = ref('');
const documentContent = ref();

onMounted(async () => {
  loading.value = true;
  console.log(parseAsync(props.contentInfo.data.buffer));
  const result = await mammoth.convertToHtml({ arrayBuffer: props.contentInfo.data.buffer });
  const result2 = await renderAsync(props.contentInfo.data.buffer, documentContent.value);
  htmlContent.value = result.value;
  loading.value = false;
});
</script>

<style scoped lang="scss"></style>
