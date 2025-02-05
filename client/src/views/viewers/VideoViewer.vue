<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper :error="error">
    <template #viewer>
      <video
        v-if="src.length"
        ref="videoElement"
        class="video"
        @play="updateMediaData"
        @playing="updateMediaData"
        @canplay="updateMediaData"
        @pause="updateMediaData"
        @volumechange="updateMediaData"
        @muted="updateMediaData"
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
      <file-controls>
        <file-controls-playback
          :paused="fluxProgress.paused"
          :ended="ended"
          @click="togglePlayback"
        />
        <file-controls-flux
          v-model="fluxProgress"
          :length="length"
        />
        <file-controls-volume
          v-model="volume"
          :muted="muted"
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
const ended = ref(false);
const muted = ref(false);
const fluxProgress = ref<SliderState>({ progress: 0, paused: true });
const volume = ref<number>(1);
const error = ref('');

const cancelFluxProgressWatch = watch(
  () => fluxProgress.value,
  () => {
    if (videoElement.value) {
      videoElement.value.currentTime = fluxProgress.value.progress / 100;
      if (videoElement.value.paused !== fluxProgress.value.paused) {
        togglePlayback();
      }
    }
  },
);

const cancelVolumeWatch = watch(
  () => volume.value,
  () => {
    if (videoElement.value) {
      videoElement.value.volume = volume.value / 100;
    }
  },
);

onMounted(async () => {
  src.value = URL.createObjectURL(new Blob([props.contentInfo.data], { type: props.contentInfo.mimeType }));
});

onUnmounted(() => {
  cancelFluxProgressWatch();
  cancelVolumeWatch();
});

async function onError(): Promise<void> {
  error.value = 'fileViewers.mediaNotSupported';
}

function onTimeUpdate(): void {
  if (!fluxProgress.value.paused && videoElement.value && videoElement.value.currentTime !== undefined) {
    fluxProgress.value.progress = Math.floor(videoElement.value.currentTime * 100);
  }
}

function togglePlayback(): void {
  if (videoElement.value) {
    videoElement.value.paused ? videoElement.value.play() : videoElement.value.pause();
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
    volume.value = Math.floor((event.target as HTMLAudioElement).volume * 100);
    muted.value = (event.target as HTMLAudioElement).muted;
    ended.value = (event.target as HTMLVideoElement).ended;
    length.value = Math.floor((event.target as HTMLVideoElement).duration * 100);
    fluxProgress.value.progress = Math.floor((event.target as HTMLVideoElement).currentTime * 100);
    fluxProgress.value.paused = (event.target as HTMLVideoElement).paused;
  }
}
</script>

<style scoped lang="scss">
.video {
  width: 100%;
  max-height: 100%;
}
</style>
