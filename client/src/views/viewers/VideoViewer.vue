<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-viewer-wrapper :error="error">
    <template #viewer>
      <div class="video-container">
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
          @enterpictureinpicture="onTogglePictureInPicture(true)"
          @leavepictureinpicture="onTogglePictureInPicture(false)"
        >
          <source
            :src="src"
            :type="mimeType"
          />
        </video>
      </div>
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
          <file-controls-dropdown
            :items="dropdownItems"
            :icon="cog"
          />
        </file-controls-group>
        <file-controls-group>
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
import { cog, infinite, scan, timer } from 'ionicons/icons';
import { FileContentInfo, PlaybackSpeed, PlaybackSpeeds } from '@/views/viewers/utils';
import {
  FileControls,
  FileControlsButton,
  FileControlsDropdown,
  FileControlsDropdownItemContent,
  FileControlsFlux,
  FileControlsGroup,
  FileControlsPlayback,
  FileControlsVolume,
} from '@/components/viewers';
import { onMounted, onUnmounted, ref, watch } from 'vue';
import { FileViewerWrapper } from '@/views/viewers';
import { SliderState, PipIcon } from 'megashark-lib';
import { getMimeTypeFromBuffer } from '@/common/fileTypes';

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
const mimeType = ref<undefined | string>(undefined);

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

const loopState = ref(false);
const pipState = ref(false);
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
    label: 'fileViewers.players.pip',
    image: PipIcon,
    callback: togglePictureInPicture,
    isActive: false,
  },
  {
    label: 'fileViewers.players.loop',
    icon: infinite,
    callback: toggleLoop,
    isActive: false,
  },
]);

onMounted(async () => {
  mimeType.value = await getMimeTypeFromBuffer(props.contentInfo.data);
  src.value = URL.createObjectURL(new Blob([props.contentInfo.data], { type: mimeType.value }));
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

function onTogglePictureInPicture(value: boolean): void {
  pipState.value = value;
  dropdownItems.value[1].isActive = value;
}

async function togglePictureInPicture(): Promise<void> {
  if (pipState.value === false) {
    await videoElement.value?.requestPictureInPicture();
  } else {
    try {
      document.exitPictureInPicture();
    } catch (error: any) {
      console.warn(error);
    }
  }
}

async function changePlaybackSpeed(value: number): Promise<void> {
  if (videoElement.value) {
    videoElement.value.playbackRate = PlaybackSpeeds[value];
    dropdownItems.value[0].children?.forEach((item, index) => {
      item.isActive = value === index;
    });
  }
}

async function toggleLoop(): Promise<void> {
  if (videoElement.value) {
    loopState.value = !loopState.value;
    videoElement.value.loop = loopState.value;
    dropdownItems.value[2].isActive = loopState.value;
  }
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
.video-container {
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 2rem;
}

.video {
  border-radius: var(--parsec-radius-8);
  overflow: hidden;
  width: fit-content;
  height: fit-content;
  max-height: 100%;
  box-shadow: var(--parsec-shadow-light);
}
</style>
