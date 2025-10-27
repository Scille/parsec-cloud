<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->
<template>
  <file-controls-group>
    <div class="volume">
      <file-controls-button
        :icon="getVolumeIcon()"
        @click="toggleVolume"
        class="volume-button"
      />
      <ms-slider
        class="volume-slider"
        v-model="sliderState"
        @change="updateSliderState"
        :max-value="100"
      />
    </div>
  </file-controls-group>
</template>

<script setup lang="ts">
import { FileControlsButton, FileControlsGroup } from '@/components/files/handler/viewer';
import { volumeHigh, volumeLow, volumeMedium, volumeMute, volumeOff } from 'ionicons/icons';
import { MsSlider, SliderState } from 'megashark-lib';
import { onUnmounted, ref, watch } from 'vue';

const sliderState = ref<SliderState>({ progress: 100 });
const storedVolume = ref<number>(0);
const mutedRef = ref(false);

const props = defineProps<{
  muted: boolean;
  modelValue: number;
}>();

const emits = defineEmits<{
  (event: 'update:modelValue', value: number): void;
}>();

function toggleVolume(): void {
  mutedRef.value = !mutedRef.value;
  onMutedChange();
}

// Update the slider as the volume is changing
const cancelSliderWatch = watch(
  () => props.modelValue,
  () => {
    sliderState.value.progress = props.modelValue;
    if (props.modelValue > 0 && mutedRef.value) {
      mutedRef.value = false;
    }
  },
);

const cancelMutedWatch = watch(
  () => props.muted,
  () => {
    mutedRef.value = props.muted;
    onMutedChange();
  },
);

function getVolumeIcon(): string {
  if (mutedRef.value) {
    return volumeMute;
  }
  switch (true) {
    case sliderState.value.progress === 0:
      return volumeOff;
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

function updateSliderState(value: SliderState): void {
  emits('update:modelValue', value.progress);
}

function onMutedChange(): void {
  if (mutedRef.value) {
    storedVolume.value = sliderState.value.progress;
    sliderState.value.progress = 0;
  } else {
    sliderState.value.progress = storedVolume.value > 0 ? storedVolume.value : 100;
  }
  updateSliderState(sliderState.value);
}

onUnmounted(() => {
  cancelSliderWatch();
  cancelMutedWatch();
});
</script>

<style scoped lang="scss">
.volume {
  display: flex;
  gap: 0.5rem;

  .volume-button:hover {
    background-color: transparent;
    box-shadow: none;
  }

  &-slider {
    margin-right: 1rem;
    opacity: 1;
    width: 5rem;
    display: flex;

    @include ms.responsive-breakpoint('sm') {
      margin-right: 0;
    }
  }
}
</style>
