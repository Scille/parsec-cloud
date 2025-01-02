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
        <file-controls-volume @on-volume-change="updateVolume" />
        <file-controls-fullscreen @click="toggleFullScreen" />
      </file-controls>
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { FileViewerWrapper } from '@/views/viewers';
import { FileContentInfo } from '@/views/viewers/utils';
import { FileControls, FileControlsFullscreen, FileControlsVolume } from '@/components/viewers';

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

function updateMediaData(event: Event): void {
  paused.value = (event.target as HTMLAudioElement).paused;
  ended.value = (event.target as HTMLAudioElement).ended;
}

function updateVolume(value: number): void {
  audioElement.value.volume = value;
}

async function toggleFullScreen(): Promise<void> {
  await audioElement.value?.requestFullscreen();
}
</script>

<style scoped lang="scss"></style>
