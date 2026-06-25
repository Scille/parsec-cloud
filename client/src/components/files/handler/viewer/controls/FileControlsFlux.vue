<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->
<template>
  <file-controls-group class="file-controls-flux-container">
    <div class="file-controls-flux">
      <div class="progress-label-start button-medium">
        {{ progressDuration.toFormat('hh:mm:ss') }}
      </div>
      <ms-slider
        class="progress-slider"
        v-model="sliderState"
        @change="updateSliderState"
        :max-value="length"
        :increment-value="500"
      />
      <div class="progress-label-end button-medium">
        {{ duration.toFormat('hh:mm:ss') }}
      </div>
    </div>
  </file-controls-group>
</template>

<script setup lang="ts">
import { FileControlsGroup } from '@/components/files/handler/viewer';
import { Duration } from 'luxon';
import { MsSlider, SliderState } from 'megashark-lib';
import { computed, onUnmounted, ref, watch } from 'vue';

const props = defineProps<{
  modelValue: SliderState;
  length: number;
}>();

const emits = defineEmits<{
  (event: 'update:modelValue', state: SliderState): void;
}>();

const sliderState = ref<SliderState>({ progress: 0, paused: true });
const progressDuration = ref<Duration>(Duration.fromMillis(0));

// Update the slider as the media is playing
const cancelProgressWatch = watch(
  () => props.modelValue.progress,
  () => {
    sliderState.value.progress = props.modelValue.progress;
    progressDuration.value = Duration.fromMillis(props.modelValue.progress * 10);
  },
);
const cancelPausedWatch = watch(
  () => props.modelValue.paused,
  () => {
    sliderState.value.paused = props.modelValue.paused;
  },
);
const duration = computed(() => {
  return Duration.fromMillis(props.length * 10);
});

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
    width: 7rem;
    text-align: right;
  }

  .progress-label-end {
    width: 7rem;
    text-align: left;
  }
}
</style>
