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
        <file-controls-button
          :icon="remove"
          @click="zoomOut"
        />
        <file-controls-button
          :icon="resize"
          @click="resetZoom"
        />
        <file-controls-input
          class="zoom-level-input"
          v-model="zoomLevelString"
          @on-submitted-value="onChange"
          :restrict-change="validateZoomLevel"
          suffix="%"
          @keydown.up.prevent="zoomIn"
          @keydown.down.prevent="zoomOut"
        />
        <file-controls-button
          :icon="add"
          @click="zoomIn"
        />
      </file-controls>
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import { add, remove, resize } from 'ionicons/icons';
import { computed, onMounted, ref, watch } from 'vue';
import { FileControls, FileControlsButton, FileControlsInput } from '@/components/viewers';
import { FileViewerWrapper } from '@/views/viewers';
import { FileContentInfo, imageViewerUtils } from '@/views/viewers/utils';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const viewerConfig = ref({
  zoomLevel: 100,
});

const src = ref('');
const zoomLevelString = ref(viewerConfig.value.zoomLevel.toString());

const zoomLevel = computed(() => {
  const level = viewerConfig.value.zoomLevel;
  if (level < 5 || level > 500) {
    return;
  }
  return level / 100;
});

onMounted(async () => {
  src.value = URL.createObjectURL(new Blob([props.contentInfo.data], { type: props.contentInfo.mimeType }));
});

watch(() => viewerConfig.value.zoomLevel, (newZoomLevel) => {
  zoomLevelString.value = newZoomLevel.toString();
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

function onChange(value: string): void {
  viewerConfig.value.zoomLevel = parseInt(value);
}

async function validateZoomLevel(value: string): Promise<string> {
  const level = parseInt(value);
  if (level < 5 || level > 500) {
    return '';
  }
  return value;
}
</script>

<style scoped lang="scss">
img {
  transform: scale(v-bind(zoomLevel));
  transition: transform ease-in-out 0.3s;
  max-width: 100%;
  max-height: 100%;
}

.zoom-level-input {
  width: 4rem;
}
</style>
