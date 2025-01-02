<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->
<template>
  <file-controls-group>
    <div class="volume">
      <file-controls-button
        :icon="getVolumeIcon()"
        @click="toggleVolume"
      />
      <ms-slider
        class="volume-slider"
        v-model="sliderState"
        :max-value="100"
      />
    </div>
  </file-controls-group>
</template>

<script setup lang="ts">
import { MsSlider, SliderState } from 'megashark-lib';
import { onUnmounted, ref, watch } from 'vue';
import { FileControlsButton, FileControlsGroup } from '@/components/viewers';
import { volumeHigh, volumeLow, volumeMedium, volumeMute } from 'ionicons/icons';

const sliderState = ref<SliderState>({ progress: 100 });
const storedVolume = ref<number>(0);

onUnmounted(() => {
  cancelVolumeWatch();
});

const emits = defineEmits<{
  (event: 'onVolumeChange', progress: number): void;
}>();

function toggleVolume(): void {
  if (sliderState.value.progress === 0) {
    sliderState.value.progress = storedVolume.value > 0 ? storedVolume.value : 100;
  } else {
    storedVolume.value = sliderState.value.progress;
    sliderState.value.progress = 0;
  }
}

const cancelVolumeWatch = watch(
  () => sliderState.value.progress,
  () => {
    emits('onVolumeChange', sliderState.value.progress / 100);
  },
);

function getVolumeIcon(): string {
  switch (true) {
    case sliderState.value.progress === 0:
      return volumeMute;
    case sliderState.value.progress <= 33:
      return volumeLow;
    case sliderState.value.progress <= 67:
      return volumeMedium;
    case sliderState.value.progress <= 100:
      return volumeHigh;
    default:
      return volumeMute;
  }
}
</script>

<style scoped lang="scss">
.volume {
  display: flex;

  &-slider {
    margin-right: 1rem;
    opacity: 1;
    width: 5rem;
    display: flex;
  }
}
</style>
