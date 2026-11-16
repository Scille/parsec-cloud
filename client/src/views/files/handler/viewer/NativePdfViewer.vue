<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper>
    <template #viewer>
      <iframe
        v-if="url"
        class="native-pdf-viewer"
        :src="url"
        @load="emits('loaded')"
      />
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import { FileViewerWrapper } from '@/views/files/handler/viewer';
import { onBeforeUnmount, onMounted, ref } from 'vue';

// Displays the PDF with the browser's own viewer.

const props = defineProps<{
  content: Uint8Array;
}>();

const emits = defineEmits<{
  (e: 'loaded'): void;
  (e: 'error'): void;
}>();

const url = ref('');

onMounted(() => {
  try {
    url.value = URL.createObjectURL(new Blob([props.content as Uint8Array<ArrayBuffer>], { type: 'application/pdf' }));
  } catch (e: any) {
    window.nativeAPI.log('error', `Failed to create PDF object URL: ${e}`);
    emits('error');
  }
});

onBeforeUnmount(() => {
  if (url.value) {
    URL.revokeObjectURL(url.value);
  }
});
</script>

<style scoped lang="scss">
.native-pdf-viewer {
  width: 100%;
  height: 100%;
  border: none;
}
</style>
