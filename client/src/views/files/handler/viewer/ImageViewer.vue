<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper>
    <template #viewer>
      <div
        class="image-viewer-container"
        ref="imageViewerContainer"
      >
        <div class="image-viewer">
          <ms-draggable
            v-if="src.length"
            :disabled="!isZoomedMoreThanViewport"
            :restrict-direction="restrictDirection"
            @dragging="updateDraggingRestrictions"
            ref="draggableElement"
          >
            <img
              ref="imgElement"
              :src="src"
              @load="updateDraggingRestrictions()"
            />
          </ms-draggable>
        </div>
      </div>
    </template>
    <template #controls>
      <file-controls>
        <file-controls-zoom
          @change="onChange"
          ref="zoomControl"
        />
        <file-controls-button
          @click="toggleFullScreen"
          :icon="scan"
        />
      </file-controls>
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import { getMimeTypeFromBuffer } from '@/common/fileTypes';
import { FileControls, FileControlsButton, FileControlsZoom } from '@/components/files/handler/viewer';
import { FileViewerWrapper } from '@/views/files/handler/viewer';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import { scan } from 'ionicons/icons';
import { MsDraggable } from 'megashark-lib';
import { onMounted, onUnmounted, ref, useTemplateRef, watch } from 'vue';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const src = ref('');
const zoomControlRef = useTemplateRef<InstanceType<typeof FileControlsZoom>>('zoomControl');
const zoomLevel = ref(1);
const imgElementRef = useTemplateRef<HTMLImageElement>('imgElement');
const imageViewerContainerRef = useTemplateRef<HTMLDivElement>('imageViewerContainer');
const isZoomedMoreThanViewport = ref(false);
const restrictDirection = ref({
  up: true,
  down: true,
  left: true,
  right: true,
});
const draggableElementRef = useTemplateRef<InstanceType<typeof MsDraggable>>('draggableElement');

function updateDraggingRestrictions(resetPosition = false): void {
  if (resetPosition) {
    draggableElementRef.value?.resetPosition();
  }

  restrictDirection.value = {
    up: true,
    down: true,
    left: true,
    right: true,
  };

  if (imgElementRef.value && imageViewerContainerRef.value) {
    const imgRect = imgElementRef.value.getBoundingClientRect();
    const containerRect = imageViewerContainerRef.value.getBoundingClientRect();
    isZoomedMoreThanViewport.value = imgRect.width > containerRect.width || imgRect.height > containerRect.height;

    // image is overflowing to the top
    if (imgRect.top < containerRect.top) {
      restrictDirection.value.down = false;
    }
    // image is overflowing to the bottom
    if (imgRect.bottom > containerRect.bottom) {
      restrictDirection.value.up = false;
    }
    // image is overflowing to the left
    if (imgRect.left < containerRect.left) {
      restrictDirection.value.right = false;
    }
    // image is overflowing to the right
    if (imgRect.right > containerRect.right) {
      restrictDirection.value.left = false;
    }
  }
}

onMounted(async () => {
  const mimeType = await getMimeTypeFromBuffer(props.contentInfo.data);
  src.value = URL.createObjectURL(new Blob([props.contentInfo.data.buffer as ArrayBuffer], { type: mimeType }));
  zoomLevel.value = (zoomControlRef.value?.getZoom() ?? 100) / 100;
  window.addEventListener('resize', () => updateDraggingRestrictions(true));
});

onUnmounted(() => {
  window.removeEventListener('resize', () => updateDraggingRestrictions(true));
});

watch(zoomLevel, () => {
  updateDraggingRestrictions(true);
});

function onChange(value: number): void {
  zoomLevel.value = value / 100;
}

async function toggleFullScreen(): Promise<void> {
  if (document.fullscreenElement) {
    await document.exitFullscreen();
    return;
  }
  await imageViewerContainerRef.value?.requestFullscreen();
}
</script>

<style scoped lang="scss">
.image-viewer-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  width: 100%;
  // overflow: auto;
  background: var(--parsec-color-light-secondary-premiere);
}
.image-viewer {
  display: flex;
  width: fit-content;
  height: fit-content;
  padding: 2rem;
}

img {
  position: relative;
  transition: all ease-in-out;
  max-width: 100%;
  max-height: 100%;
  box-shadow: var(--parsec-shadow-light);
  transform: scale(v-bind(zoomLevel));
  user-select: none;
}
</style>
