<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->
<template>
  <file-controls-group class="file-controls-flux">
    <ms-slider
      class="progress-slider"
      v-model="sliderState"
      @change="updateSliderState"
      :max-value="length"
      :increment-value="500"
    />
    <div class="progress-label">
      {{ `${(sliderState.progress / 100).toFixed(0)} / ${(length / 100).toFixed(0)}` }}
    </div>
  </file-controls-group>
</template>

<script setup lang="ts">
import { MsSlider, SliderState } from 'megashark-lib';
import { FileControlsGroup } from '@/components/viewers';
import { onUnmounted, ref, watch } from 'vue';

const props = defineProps<{
  modelValue: SliderState;
  length: number;
}>();

const emits = defineEmits<{
  (event: 'update:modelValue', state: SliderState): void;
}>();

const sliderState = ref<SliderState>({ progress: 0, paused: true });

// Update the slider as the media is playing
const cancelProgressWatch = watch(
  () => props.modelValue.progress,
  () => {
    sliderState.value.progress = props.modelValue.progress;
  },
);
const cancelPausedWatch = watch(
  () => props.modelValue.paused,
  () => {
    sliderState.value.paused = props.modelValue.paused;
  },
);

onUnmounted(() => {
  cancelProgressWatch();
  cancelPausedWatch();
});

function updateSliderState(value: SliderState): void {
  emits('update:modelValue', value);
}
</script>

<style scoped lang="scss">
.file-controls-flux {
  justify-content: start;
  align-items: center;
  gap: 1rem !important;

  .progress-slider {
    width: 30rem;
  }

  .progress-label {
    text-align: center;
  }
}
</style>
