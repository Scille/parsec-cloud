<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <video
    class="video"
    controls
    width="250"
    v-if="src.length"
  >
    <source
      :src="src"
      :type="contentInfo.mimeType"
    />
  </video>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { FileContentInfo } from '@/views/viewers/utils';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const src = ref('');

onMounted(async () => {
  src.value = URL.createObjectURL(new Blob([props.contentInfo.data], { type: props.contentInfo.mimeType }));
});
</script>

<style scoped lang="scss">
.video {
  width: 100%;
  height: 680px;
}
</style>
