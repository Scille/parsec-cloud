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
    <template #controls>
      <file-controls>
        <!-- Disabled till we add an illustration in the viewer -->
        <!-- <file-controls-button
          :class="{'flip-horizontal-ion-icon': ended}"
          :icon="getPlaybackIcon()"
          @click="togglePlayback"
        /> -->
        <file-controls-volume @on-volume-change="updateVolume" />
      </file-controls>
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
// import { refresh, play, pause, volumeHigh, volumeLow, volumeMedium, volumeMute } from 'ionicons/icons';
import { onMounted, ref } from 'vue';
import { FileViewerWrapper } from '@/views/viewers';
import { FileContentInfo } from '@/views/viewers/utils';
import { FileControls, FileControlsVolume } from '@/components/viewers';

const props = defineProps<{
  contentInfo: FileContentInfo;
}>();

const src = ref('');
const audioElement = ref();
const paused = ref(true);
const ended = ref(false);

onMounted(async () => {
  src.value = URL.createObjectURL(new Blob([props.contentInfo.data], { type: props.contentInfo.mimeType }));
});

// function togglePlayback(): void {
//   audioElement.value.paused ? audioElement.value.play() : audioElement.value.pause();
// }

function updateMediaData(event: Event): void {
  paused.value = (event.target as HTMLAudioElement).paused;
  ended.value = (event.target as HTMLAudioElement).ended;
}

function updateVolume(value: number): void {
  audioElement.value.volume = value;
}

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
