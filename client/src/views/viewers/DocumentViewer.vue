<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-spinner v-show="loading" />
  <div
    v-show="!loading"
    class="document-content"
    v-html="htmlContent"
  />
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
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
  const result = await mammoth.convertToHtml({ arrayBuffer: props.contentInfo.data.buffer });
  htmlContent.value = result.value;
  loading.value = false;
});
</script>

<style scoped lang="scss"></style>
