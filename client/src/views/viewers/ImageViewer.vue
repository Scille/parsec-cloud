<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <img
    v-if="src.length"
    :src="src"
  />
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { FileContentInfo, ImageViewerConfig } from '@/views/viewers/utils';

const props = defineProps<{
  contentInfo: FileContentInfo;
  config: ImageViewerConfig;
}>();

const src = ref('');
const zoomLevel = computed(() => {
  const level = props.config.zoomLevel;
  console.log('Setting zoom level to', level);
  if (level < 5 || level > 500) {
    return;
  }
  return level / 100;
});

onMounted(async () => {
  src.value = URL.createObjectURL(new Blob([props.contentInfo.data], { type: props.contentInfo.mimeType }));
});
</script>

<style scoped lang="scss">
img {
  transform: scale(v-bind(zoomLevel));
  transition: transform ease-in-out 0.3s;
  max-width: 100%;
  max-height: 100%;
}
</style>
