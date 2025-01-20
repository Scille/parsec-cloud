<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper>
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
        @ended="updateMediaData"
        @timeupdate="onTimeUpdate"
      />
      <file-viewer-background :icon="musicalNotes" />
    </template>
    <template #controls>
      <file-controls>
        <file-controls-playback
          :paused="progress.paused"
          :ended="ended"
          @click="togglePlayback"
        />
        <file-controls-flux
          v-model="progress"
          :length="length"
        />
        <file-controls-volume @on-volume-change="updateVolume" />
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
const volume = ref(1);
const ended = ref(false);
const progress = ref<SliderState>({ progress: 0, paused: true });

const cancelProgressWatch = watch(
  () => progress.value,
  () => {
    if (audioElement.value) {
      audioElement.value.currentTime = progress.value.progress / 100;
      if (audioElement.value.paused !== progress.value.paused) {
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

function onTimeUpdate(): void {
  if (!progress.value.paused && audioElement.value?.currentTime !== undefined) {
    progress.value.progress = audioElement.value?.currentTime * 100;
  }
}

function togglePlayback(): void {
  if (audioElement.value) {
    audioElement.value.paused ? audioElement.value.play() : audioElement.value.pause();
  }
}

function updateMediaData(event: Event): void {
  volume.value = (event.target as HTMLAudioElement).volume;
  length.value = (event.target as HTMLVideoElement).duration * 100;
  progress.value.progress = (event.target as HTMLVideoElement).currentTime * 100;
  progress.value.paused = (event.target as HTMLVideoElement).paused;
  ended.value = (event.target as HTMLAudioElement).ended;
}

function updateVolume(value: number): void {
  if (audioElement.value) {
    audioElement.value.volume = value;
  }
}
</script>

<style scoped lang="scss"></style>
