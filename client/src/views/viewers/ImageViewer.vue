<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper>
    <template #viewer>
      <img
        v-if="src.length"
        :src="src"
      />
    </template>
    <template #controls>
      <file-viewer-action-bar
        :actions="[
          { icon: remove, handler: zoomOut },
          { icon: resize, handler: resetZoom },
          { icon: add, handler: zoomIn },
        ]"
      />
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import { add, remove, resize } from 'ionicons/icons';
import { computed, onMounted, ref } from 'vue';
import { FileViewerActionBar } from '@/components/viewers';
import { FileViewerWrapper } from '@/views/viewers';
import { FileContentInfo, imageViewerUtils } from '@/views/viewers/utils';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const viewerConfig = ref({
  zoomLevel: 100,
});

const src = ref('');
const zoomLevel = computed(() => {
  const level = viewerConfig.value.zoomLevel;
  console.log('Setting zoom level to', level);
  if (level < 5 || level > 500) {
    return;
  }
  return level / 100;
});

onMounted(async () => {
  src.value = URL.createObjectURL(new Blob([props.contentInfo.data], { type: props.contentInfo.mimeType }));
});

function zoomOut(): void {
  viewerConfig.value = imageViewerUtils.zoomOut(viewerConfig.value);
}

function resetZoom(): void {
  viewerConfig.value = imageViewerUtils.resetZoom(viewerConfig.value);
}

function zoomIn(): void {
  viewerConfig.value = imageViewerUtils.zoomIn(viewerConfig.value);
}
</script>

<style scoped lang="scss">
img {
  transform: scale(v-bind(zoomLevel));
  transition: transform ease-in-out 0.3s;
  max-width: 100%;
  max-height: 100%;
}
</style>
