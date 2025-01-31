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
        <file-controls-group>
          <!-- <file-controls-dropdown
            :items="dropdownItems"
            :icon="cog"
          /> -->
          <file-controls-button
            @click="toggleFullScreen"
            :icon="scan"
          />
        </file-controls-group>
      </file-controls>
    </template>
  </file-viewer-wrapper>
</template>

<script setup lang="ts">
import { scan } from 'ionicons/icons';
import { FileContentInfo } from '@/views/viewers/utils';
import {
  FileControls,
  FileControlsButton,
  FileControlsFlux,
  FileControlsGroup,
  FileControlsPlayback,
  FileControlsVolume,
} from '@/components/viewers';
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

// FIXME Dropdown items are left as an example for future implementation

// const pipState = ref(false);
// const loopState = ref(false);
// const dropdownItems = ref<FileControlsDropdownItemContent[]>([
//   {
//     label: 'Quality',
//     icon: cog,
//     children: [
//       { label: '480p', callback: (): Promise<void> => changeResolution(0) },
//       { label: '720p', callback: (): Promise<void> => changeResolution(1) },
//       { label: '1080p', callback: (): Promise<void> => changeResolution(2), isActive: true },
//       { label: '2160p', callback: (): Promise<void> => changeResolution(3) },
//     ],
//   },
//   {
//     label: 'Playback speed',
//     icon: cog,
//     children: [
//       { label: 'x0.25', callback: (): Promise<void> => changePlaybackSpeed(0) },
//       { label: 'x0.5', callback: (): Promise<void> => changePlaybackSpeed(1) },
//       { label: 'x1', callback: (): Promise<void> => changePlaybackSpeed(2), isActive: true },
//       { label: 'x1.25', callback: (): Promise<void> => changePlaybackSpeed(3) },
//       { label: 'x1.5', callback: (): Promise<void> => changePlaybackSpeed(4) },
//       { label: 'x2', callback: (): Promise<void> => changePlaybackSpeed(5) },
//     ],
//   },
//   {
//     label: 'Picture in picture',
//     icon: cog,
//     callback: changePip,
//   },
//   {
//     label: 'Loop',
//     icon: cog,
//     callback: changeLoop,
//   },
//   {
//     label: 'Language',
//     icon: cog,
//     children: [
//       { label: 'English', callback: (): Promise<void> => changeAudioLanguage(0), isActive: true },
//       { label: 'French', callback: (): Promise<void> => changeAudioLanguage(1) },
//     ],
//   },
//   {
//     label: 'Subtitles',
//     icon: cog,
//     children: [
//       { label: 'None', callback: (): Promise<void> => changeSubtitlesLanguage(0), isActive: true },
//       { label: 'English', callback: (): Promise<void> => changeSubtitlesLanguage(1) },
//       { label: 'French', callback: (): Promise<void> => changeSubtitlesLanguage(2) },
//     ],
//   },
// ]);

// async function changePip(): Promise<void> {
//   pipState.value = !pipState.value;
//   dropdownItems.value[2].isActive = pipState.value;
// }

// async function changeLoop(): Promise<void> {
//   loopState.value = !loopState.value;
//   dropdownItems.value[3].isActive = loopState.value;
// }

// async function changeResolution(option: number): Promise<void> {
//   dropdownItems.value[0].children?.forEach((item, index) => {
//     item.isActive = option === index;
//   });
// }

// async function changePlaybackSpeed(option: number): Promise<void> {
//   dropdownItems.value[1].children?.forEach((item, index) => {
//     item.isActive = option === index;
//   });
// }

// async function changeAudioLanguage(option: number): Promise<void> {
//   dropdownItems.value[4].children?.forEach((item, index) => {
//     item.isActive = option === index;
//   });
// }

// async function changeSubtitlesLanguage(option: number): Promise<void> {
//   dropdownItems.value[5].children?.forEach((item, index) => {
//     item.isActive = option === index;
//   });
// }

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
