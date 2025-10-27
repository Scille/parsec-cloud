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
          class="file-controls-volume"
        />
        <file-controls-dropdown
          :items="dropdownItems"
          :icon="cog"
        />
      </file-controls>
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import { getMimeTypeFromBuffer } from '@/common/fileTypes';
import {
  FileControls,
  FileControlsDropdown,
  FileControlsDropdownItemContent,
  FileControlsFlux,
  FileControlsPlayback,
  FileControlsVolume,
  FileViewerBackground,
} from '@/components/files/handler/viewer';
import { FileViewerWrapper } from '@/views/files/handler/viewer';
import { FileContentInfo, PlaybackSpeed, PlaybackSpeeds } from '@/views/files/handler/viewer/utils';
import { cog, infinite, musicalNotes, timer } from 'ionicons/icons';
import { SliderState } from 'megashark-lib';
import { onMounted, onUnmounted, ref, useTemplateRef, watch } from 'vue';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const src = ref('');
const audioElementRef = useTemplateRef<HTMLAudioElement>('audioElement');
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
    if (audioElementRef.value) {
      audioElementRef.value.currentTime = fluxProgress.value.progress / 100;
      if (audioElementRef.value.paused !== fluxProgress.value.paused) {
        togglePlayback();
      }
    }
  },
);

const cancelVolumeWatch = watch(
  () => volume.value,
  () => {
    if (audioElementRef.value) {
      audioElementRef.value.volume = volume.value / 100;
    }
  },
);

const loopState = ref(false);
const dropdownItems = ref<FileControlsDropdownItemContent[]>([
  {
    label: 'fileViewers.players.playback.title',
    icon: timer,
    children: [
      { label: 'fileViewers.players.playback.speed.0_25', callback: (): Promise<void> => changePlaybackSpeed(PlaybackSpeed.Speed_0_25) },
      { label: 'fileViewers.players.playback.speed.0_5', callback: (): Promise<void> => changePlaybackSpeed(PlaybackSpeed.Speed_0_5) },
      {
        label: 'fileViewers.players.playback.speed.1',
        callback: (): Promise<void> => changePlaybackSpeed(PlaybackSpeed.Speed_1),
        isActive: true,
      },
      { label: 'fileViewers.players.playback.speed.1_5', callback: (): Promise<void> => changePlaybackSpeed(PlaybackSpeed.Speed_1_5) },
      { label: 'fileViewers.players.playback.speed.2', callback: (): Promise<void> => changePlaybackSpeed(PlaybackSpeed.Speed_2) },
    ],
  },
  {
    label: 'fileViewers.players.loop',
    icon: infinite,
    callback: toggleLoop,
    isActive: false,
  },
]);

onMounted(async () => {
  loading.value = true;
  const mimeType = await getMimeTypeFromBuffer(props.contentInfo.data);
  src.value = URL.createObjectURL(new Blob([props.contentInfo.data.buffer as ArrayBuffer], { type: mimeType }));
});

onUnmounted(() => {
  cancelFluxProgressWatch();
  cancelVolumeWatch();
});

async function onError(): Promise<void> {
  loading.value = false;
  error.value = 'fileViewers.errors.mediaNotSupported';
}

function onTimeUpdate(): void {
  if (!fluxProgress.value.paused && audioElementRef.value?.currentTime !== undefined) {
    fluxProgress.value.progress = Math.floor(audioElementRef.value?.currentTime * 100);
  }
}

function togglePlayback(): void {
  if (audioElementRef.value) {
    audioElementRef.value.paused ? audioElementRef.value.play() : audioElementRef.value.pause();
  }
}

async function changePlaybackSpeed(value: number): Promise<void> {
  if (audioElementRef.value) {
    audioElementRef.value.playbackRate = PlaybackSpeeds[value];
    dropdownItems.value[0].children?.forEach((item, index) => {
      item.isActive = value === index;
    });
  }
}

async function toggleLoop(): Promise<void> {
  if (audioElementRef.value) {
    loopState.value = !loopState.value;
    audioElementRef.value.loop = loopState.value;
    dropdownItems.value[1].isActive = loopState.value;
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
