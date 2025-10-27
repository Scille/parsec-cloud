<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->
<template>
  <file-controls-group class="file-controls-flux-container">
    <div class="file-controls-flux">
      <div class="progress-label-start button-medium">
        {{ `${(sliderState.progress / 100).toFixed(0)}` }}
      </div>
      <ms-slider
        class="progress-slider"
        v-model="sliderState"
        @change="updateSliderState"
        :max-value="length"
        :increment-value="500"
      />
      <div class="progress-label-end button-medium">
        {{ `${(length / 100).toFixed(0)}` }}
      </div>
    </div>
  </file-controls-group>
</template>

<script setup lang="ts">
import { FileControlsGroup } from '@/components/files/handler/viewer';
import { MsSlider, SliderState } from 'megashark-lib';
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
.file-controls-flux-container {
  justify-content: start;
  align-items: center;
  max-width: 30rem;
  width: 100%;

  .file-controls-flux {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    width: 100%;
  }

  .progress-slider {
    width: 100%;
  }

  [class*='progress-label'] {
    width: 3.5rem;
    color: var(--parsec-color-light-secondary-text);
    text-overflow: ellipsis;
    white-space: nowrap;
    overflow: hidden;
  }

  .progress-label-start {
    text-align: right;
  }

  .progress-label-end {
    text-align: left;
  }
}
</style>
