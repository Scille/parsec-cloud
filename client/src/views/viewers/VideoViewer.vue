<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper>
    <template #viewer>
      <video
        class="video"
        v-if="src.length"
        ref="videoElement"
        @play="updateMediaData"
        @playing="updateMediaData"
        @canplay="updateMediaData"
        @pause="updateMediaData"
        @volumechange="updateMediaData"
      >
        <source
          :src="src"
          :type="contentInfo.mimeType"
        />
      </video>
    </template>
    <template #controls>
      <file-controls>
        <file-controls-button
          :icon="paused ? play : pause"
          @click="togglePlayback"
        />
        <file-controls-button
          :icon="getVolumeIcon()"
          @click="toggleVolume"
        />
        <file-controls-button
          :icon="scan"
          @click="toggleFullScreen"
        />
      </file-controls>
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import { play, pause, volumeHigh, volumeLow, volumeMedium, volumeMute, scan } from 'ionicons/icons';
import { onMounted, ref } from 'vue';
import { FileContentInfo } from '@/views/viewers/utils';
import { FileControls, FileControlsButton } from '@/components/viewers';
import { FileViewerWrapper } from '@/views/viewers';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const VOLUME_LEVELS = [0, 0.25, 0.5, 1];

const src = ref('');
const videoElement = ref();
const paused = ref(true);
const volume = ref(1);

onMounted(async () => {
  src.value = URL.createObjectURL(new Blob([props.contentInfo.data], { type: props.contentInfo.mimeType }));
});

function togglePlayback(): void {
  videoElement.value.paused ? videoElement.value.play() : videoElement.value.pause();
}

function toggleVolume(): void {
  videoElement.value.volume = VOLUME_LEVELS[(VOLUME_LEVELS.indexOf(videoElement.value.volume) + 1) % VOLUME_LEVELS.length];
}

function toggleFullScreen(): void {
  videoElement.value?.requestFullscreen();
}

function updateMediaData(event: Event): void {
  volume.value = (event.target as HTMLVideoElement).volume;
  paused.value = (event.target as HTMLVideoElement).paused;
}

function getVolumeIcon(): string {
  switch (volume.value) {
    case 0:
      return volumeMute;
    case 0.25:
      return volumeLow;
    case 0.5:
      return volumeMedium;
    case 1:
      return volumeHigh;
    default:
      return volumeMute;
  }
}
</script>

<style scoped lang="scss">
.video {
  width: 100%;
  max-height: 100%;
}
</style>
