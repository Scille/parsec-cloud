<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper>
    <template #viewer>
      <video
        class="video"
        v-if="src.length && !error"
        ref="videoElement"
        @play="updateMediaData"
        @playing="updateMediaData"
        @canplay="
          updateMediaData($event);
          onReady($event);
        "
        @pause="updateMediaData"
        @volumechange="updateMediaData"
        @ended="updateMediaData"
        @error="onError"
      >
        <source
          :src="src"
          :type="contentInfo.mimeType"
        />
      </video>
      <ms-spinner v-if="loading" />
      <ms-report-text
        :theme="MsReportTheme.Error"
        v-if="error"
      >
        {{ $msTranslate(error) }}
      </ms-report-text>
    </template>
    <template #controls>
      <file-controls>
        <file-controls-button
          :class="{ 'flip-horizontal-ion-icon': ended }"
          :icon="getPlaybackIcon()"
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
import { refresh, play, pause, volumeHigh, volumeLow, volumeMedium, volumeMute, scan } from 'ionicons/icons';
import { onMounted, Ref, ref } from 'vue';
import { FileContentInfo } from '@/views/viewers/utils';
import { FileControls, FileControlsButton } from '@/components/viewers';
import { FileViewerWrapper } from '@/views/viewers';
import { MsReportText, MsReportTheme, MsSpinner } from 'megashark-lib';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const VOLUME_LEVELS = [0, 0.25, 0.5, 1];

const src = ref('');
const videoElement: Ref<HTMLVideoElement | undefined> = ref(undefined);
const paused = ref(true);
const volume = ref(1);
const ended = ref(false);
const error = ref('');
const loading = ref(true);

onMounted(async () => {
  loading.value = true;
  src.value = URL.createObjectURL(new Blob([props.contentInfo.data], { type: props.contentInfo.mimeType }));
});

async function onError(_event: Event): Promise<void> {
  loading.value = false;
  error.value = 'fileViewers.mediaNotSupported';
}

async function onReady(_event: Event): Promise<void> {
  loading.value = false;
}

async function togglePlayback(): Promise<void> {
  if (!videoElement.value) {
    return;
  }
  videoElement.value.paused ? await videoElement.value.play() : await videoElement.value.pause();
}

function toggleVolume(): void {
  if (!videoElement.value) {
    return;
  }
  videoElement.value.volume = VOLUME_LEVELS[(VOLUME_LEVELS.indexOf(videoElement.value.volume) + 1) % VOLUME_LEVELS.length];
}

async function toggleFullScreen(): Promise<void> {
  await videoElement.value?.requestFullscreen();
}

function updateMediaData(event: Event): void {
  volume.value = (event.target as HTMLVideoElement).volume;
  paused.value = (event.target as HTMLVideoElement).paused;
  ended.value = (event.target as HTMLVideoElement).ended;
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
