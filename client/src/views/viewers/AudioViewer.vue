<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper>
    <template #viewer>
      <audio
        controls
        v-if="src.length"
        ref="audioElement"
        :src="src"
        @play="updateMediaData"
        @playing="updateMediaData"
        @canplay="updateMediaData"
        @pause="updateMediaData"
        @volumechange="updateMediaData"
        @ended="updateMediaData"
      />
    </template>
    <!-- Disabled till we add an illustration in the viewer -->
    <!-- <template #controls>
      <file-controls>
        <file-controls-button
          :class="{'flip-horizontal-ion-icon': ended}"
          :icon="getPlaybackIcon()"
          @click="togglePlayback"
        />
        <file-controls-button
          :icon="getVolumeIcon()"
          @click="toggleVolume"
        />
      </file-controls>
    </template> -->
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
// import { refresh, play, pause, volumeHigh, volumeLow, volumeMedium, volumeMute } from 'ionicons/icons';
import { onMounted, ref } from 'vue';
import { FileViewerWrapper } from '@/views/viewers';
import { FileContentInfo } from '@/views/viewers/utils';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

// const VOLUME_LEVELS = [0, 0.25, 0.5, 1];

const src = ref('');
const audioElement = ref();
const paused = ref(true);
const volume = ref(1);
const ended = ref(false);

onMounted(async () => {
  src.value = URL.createObjectURL(new Blob([props.contentInfo.data], { type: props.contentInfo.mimeType }));
});

// function togglePlayback(): void {
//   audioElement.value.paused ? audioElement.value.play() : audioElement.value.pause();
// }

// function toggleVolume(): void {
//   audioElement.value.volume = VOLUME_LEVELS[(VOLUME_LEVELS.indexOf(audioElement.value.volume) + 1) % VOLUME_LEVELS.length];
// }

function updateMediaData(event: Event): void {
  volume.value = (event.target as HTMLAudioElement).volume;
  paused.value = (event.target as HTMLAudioElement).paused;
  ended.value = (event.target as HTMLAudioElement).ended;
}

// function getVolumeIcon(): string {
//   switch (volume.value) {
//     case 0:
//       return volumeMute;
//     case 0.25:
//       return volumeLow;
//     case 0.5:
//       return volumeMedium;
//     case 1:
//       return volumeHigh;
//     default:
//       return volumeMute;
//   }
// }

// function getPlaybackIcon(): string {
//   if (ended.value) {
//     return refresh;
//   }
//   switch (paused.value) {
//     case true:
//       return play;
//     case false:
//       return pause;
//   }
// }
</script>

<style scoped lang="scss"></style>
