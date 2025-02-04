<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper :error="error">
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
        @timeupdate="onTimeUpdate"
        @error="onError"
      >
        <source
          :src="src"
          :type="contentInfo.mimeType"
        />
      </video>
    </template>
    <template #controls>
      <file-controls-flux
        v-model="progress"
        :length="length"
      />
      <file-controls>
        <file-controls-playback
          :paused="progress.paused"
          :ended="ended"
          @click="togglePlayback"
        />
        <file-controls-volume @on-volume-change="updateVolume" />
        <file-controls-button
          @click="toggleFullScreen"
          :icon="scan"
        />
      </file-controls>
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import { scan } from 'ionicons/icons';
import { FileContentInfo } from '@/views/viewers/utils';
import { FileControls, FileControlsButton, FileControlsFlux, FileControlsPlayback, FileControlsVolume } from '@/components/viewers';
import { onMounted, onUnmounted, ref, watch } from 'vue';
import { FileViewerWrapper } from '@/views/viewers';
import { SliderState } from 'megashark-lib';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const src = ref('');
const videoElement = ref<HTMLVideoElement>();
const length = ref(0);
const volume = ref(1);
const ended = ref(false);
const progress = ref<SliderState>({ progress: 0, paused: true });
const error = ref('');

const cancelProgressWatch = watch(
  () => progress.value,
  () => {
    if (videoElement.value) {
      videoElement.value.currentTime = progress.value.progress / 100;
      if (videoElement.value.paused !== progress.value.paused) {
        togglePlayback();
      }
    }
  },
);

onMounted(async () => {
  src.value = URL.createObjectURL(new Blob([props.contentInfo.data], { type: props.contentInfo.mimeType }));
});

onUnmounted(() => {
  cancelProgressWatch();
});

async function onError(): Promise<void> {
  error.value = 'fileViewers.mediaNotSupported';
}

function onTimeUpdate(): void {
  if (!progress.value.paused && videoElement.value && videoElement.value.currentTime !== undefined) {
    progress.value.progress = videoElement.value.currentTime * 100;
  }
}

function togglePlayback(): void {
  if (videoElement.value) {
    videoElement.value.paused ? videoElement.value.play() : videoElement.value.pause();
  }
}

function updateVolume(value: number): void {
  if (videoElement.value) {
    videoElement.value.volume = value;
  }
}

async function toggleFullScreen(): Promise<void> {
  await videoElement.value?.requestFullscreen();
}

function updateMediaData(event: Event): void {
  // in some cases, the `error` event is not emitted, probably
  // because the video element waits for more data to make up
  // its mind, even if there's no more data. So instead, when
  // trying to update the media data, we check if the `duration`
  // is set or not.
  if (Number.isNaN((event.target as HTMLVideoElement).duration)) {
    error.value = 'fileViewers.mediaNotSupported';
  } else {
    volume.value = (event.target as HTMLVideoElement).volume;
    ended.value = (event.target as HTMLVideoElement).ended;
    length.value = (event.target as HTMLVideoElement).duration * 100;
    progress.value.progress = (event.target as HTMLVideoElement).currentTime * 100;
    progress.value.paused = (event.target as HTMLVideoElement).paused;
  }
}
</script>

<style scoped lang="scss">
.video {
  width: 100%;
  max-height: 100%;
}
</style>
