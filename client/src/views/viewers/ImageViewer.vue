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
      <file-controls>
        <file-controls-zoom
          @change="onChange"
          ref="zoomControl"
        />
      </file-controls>
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { FileControls, FileControlsZoom } from '@/components/viewers';
import { FileViewerWrapper } from '@/views/viewers';
import { FileContentInfo } from '@/views/viewers/utils';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const src = ref('');
const zoomControl = ref();
const zoomLevel = ref(1);

onMounted(async () => {
  src.value = URL.createObjectURL(new Blob([props.contentInfo.data], { type: props.contentInfo.mimeType }));
  zoomLevel.value = zoomControl.value.getZoom() / 100;
});

function onChange(value: number): void {
  zoomLevel.value = value / 100;
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
