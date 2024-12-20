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
        @ended="updateMediaData"
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
          :class="{ 'flip-horizontal-ion-icon': ended }"
          :icon="getPlaybackIcon()"
          @click="togglePlayback"
        />
        <file-controls-volume @on-volume-change="updateVolume" />
        <file-controls-button
          :icon="scan"
          @click="toggleFullScreen"
        />
      </file-controls>
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import { refresh, play, pause, scan } from 'ionicons/icons';
import { onMounted, ref } from 'vue';
import { FileContentInfo } from '@/views/viewers/utils';
import { FileControls, FileControlsButton, FileControlsVolume } from '@/components/viewers';
import { FileViewerWrapper } from '@/views/viewers';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const src = ref('');
const videoElement = ref();
const paused = ref(true);
const ended = ref(false);

onMounted(async () => {
  src.value = URL.createObjectURL(new Blob([props.contentInfo.data], { type: props.contentInfo.mimeType }));
});

function togglePlayback(): void {
  videoElement.value.paused ? videoElement.value.play() : videoElement.value.pause();
}

function updateVolume(value: number): void {
  videoElement.value.volume = value;
}

function toggleFullScreen(): void {
  videoElement.value?.requestFullscreen();
}

function updateMediaData(event: Event): void {
  paused.value = (event.target as HTMLVideoElement).paused;
  ended.value = (event.target as HTMLVideoElement).ended;
}

function getPlaybackIcon(): string {
  if (ended.value) {
    return refresh;
  }
  switch (paused.value) {
    case true:
      return play;
    case false:
      return pause;
  }
}
</script>

<style scoped lang="scss">
.video {
  width: 100%;
  max-height: 100%;
}
</style>
