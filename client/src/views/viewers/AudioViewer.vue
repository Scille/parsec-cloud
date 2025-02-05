<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper :error="error">
    <template #viewer>
      <audio
        v-if="src.length"
        ref="audioElement"
        :src="src"
        @play="updateMediaData"
        @playing="updateMediaData"
        @canplay="updateMediaData"
        @pause="updateMediaData"
        @volumechange="updateMediaData"
        @muted="updateMediaData"
        @ended="updateMediaData"
        @timeupdate="onTimeUpdate"
        @error="onError"
      />
      <file-viewer-background
        :icon="musicalNotes"
        v-show="!error && !loading"
      />
    </template>
    <template #controls>
      <file-controls v-show="!loading">
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
      </file-controls>
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref, watch } from 'vue';
import { musicalNotes } from 'ionicons/icons';
import { FileViewerWrapper } from '@/views/viewers';
import { FileContentInfo } from '@/views/viewers/utils';
import { FileControls, FileControlsFlux, FileControlsPlayback, FileControlsVolume, FileViewerBackground } from '@/components/viewers';
import { SliderState } from 'megashark-lib';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const src = ref('');
const audioElement = ref<HTMLAudioElement>();
const length = ref(0);
const ended = ref(false);
const muted = ref(false);
const fluxProgress = ref<SliderState>({ progress: 0, paused: true });
const volume = ref<number>(1);
const error = ref('');
// Not much to load, we just want to avoid a blinking effect in case
// of invalid media
const loading = ref(true);

const cancelFluxProgressWatch = watch(
  () => fluxProgress.value,
  () => {
    if (audioElement.value) {
      audioElement.value.currentTime = fluxProgress.value.progress / 100;
      if (audioElement.value.paused !== fluxProgress.value.paused) {
        togglePlayback();
      }
    }
  },
);

const cancelVolumeWatch = watch(
  () => volume.value,
  () => {
    if (audioElement.value) {
      audioElement.value.volume = volume.value / 100;
    }
  },
);

onMounted(async () => {
  loading.value = true;
  src.value = URL.createObjectURL(new Blob([props.contentInfo.data], { type: props.contentInfo.mimeType }));
});

onUnmounted(() => {
  cancelFluxProgressWatch();
  cancelVolumeWatch();
});

async function onError(): Promise<void> {
  loading.value = false;
  error.value = 'fileViewers.mediaNotSupported';
}

function onTimeUpdate(): void {
  if (!fluxProgress.value.paused && audioElement.value?.currentTime !== undefined) {
    fluxProgress.value.progress = Math.floor(audioElement.value?.currentTime * 100);
  }
}

function togglePlayback(): void {
  if (audioElement.value) {
    audioElement.value.paused ? audioElement.value.play() : audioElement.value.pause();
  }
}

function updateMediaData(event: Event): void {
  loading.value = false;
  volume.value = Math.floor((event.target as HTMLAudioElement).volume * 100);
  muted.value = (event.target as HTMLAudioElement).muted;
  length.value = Math.floor((event.target as HTMLVideoElement).duration * 100);
  fluxProgress.value.progress = Math.floor((event.target as HTMLVideoElement).currentTime * 100);
  fluxProgress.value.paused = (event.target as HTMLVideoElement).paused;
  ended.value = (event.target as HTMLAudioElement).ended;
}
</script>

<style scoped lang="scss"></style>
