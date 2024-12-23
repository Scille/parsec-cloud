<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper>
    <template #viewer>
      <ms-spinner v-show="loading" />
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
import { renderAsync } from 'docx-preview';
import { MsSpinner } from 'megashark-lib';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const loading = ref(true);
const documentContent = ref();

onMounted(async () => {
  loading.value = true;
  await renderAsync(props.contentInfo.data.buffer, documentContent.value, undefined, {
    ignoreLastRenderedPageBreak: false,
  });
  loading.value = false;
});
</script>

<style scoped lang="scss">
.document-content {
  width: 100%;
  height: 100%;
  overflow-y: auto;
  padding: 20px;
}
</style>
