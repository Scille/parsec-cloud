<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-controls-group>
    <file-controls-button
      :image="FitToScreenIcon"
      @click="resetZoom"
      :disabled="zoomLevel === defaultZoomLevel"
      :title="$msTranslate('fileViewers.controls.zoom.reset')"
      id="reset-zoom"
    />
    <div class="zoom-controls">
      <file-controls-button
        :image="RemoveIcon"
        @click="zoomOut"
        :disabled="zoomLevel === ZOOM_LEVELS[0]"
        :title="$msTranslate('fileViewers.controls.zoom.out')"
      />
      <file-controls-input
        class="zoom-level-input"
        v-model="zoomLevel"
        @on-submitted-value="onChange"
        :restrict-change="validateZoomLevel"
        :suffix="I18n.valueAsTranslatable('%')"
        @keydown.up.prevent="zoomIn"
        @keydown.down.prevent="zoomOut"
        :maxlength="3"
      />
      <file-controls-button
        :image="AddIcon"
        @click="zoomIn"
        :disabled="zoomLevel === ZOOM_LEVELS[ZOOM_LEVELS.length - 1]"
        :title="$msTranslate('fileViewers.controls.zoom.in')"
      />
    </div>
  </file-controls-group>
</template>

<script setup lang="ts">
import { FileControlsButton, FileControlsGroup, FileControlsInput } from '@/components/files/handler/viewer';
import { AddIcon, FitToScreenIcon, I18n, RemoveIcon } from 'megashark-lib';
import { ref } from 'vue';

const ZOOM_LEVELS = [
  '5',
  '10',
  '20',
  '30',
  '40',
  '50',
  '60',
  '70',
  '80',
  '90',
  '100',
  '125',
  '150',
  '175',
  '200',
  '250',
  '300',
  '400',
  '500',
];

const props = defineProps<{
  initialZoomLevel?: number;
}>();

const emits = defineEmits<{
  (e: 'change', value: number): void;
}>();

defineExpose({
  getZoom,
});

const defaultZoomLevel = (props.initialZoomLevel || 100).toString();
const zoomLevel = ref(defaultZoomLevel);

function getZoom(): number {
  return Number(zoomLevel.value);
}

function setZoomLevel(level: string): void {
  const closestLevel = ZOOM_LEVELS.reduce((previousValue, currentValue) => {
    return Math.abs(Number(currentValue) - Number(level)) < Math.abs(Number(previousValue) - Number(level)) ? currentValue : previousValue;
  });
  zoomLevel.value = closestLevel;
  emits('change', Number(zoomLevel.value));
}

function zoomOut(): void {
  const currentIndex = ZOOM_LEVELS.indexOf(zoomLevel.value);
  if (currentIndex === 0) {
    return;
  }
  currentIndex === -1 ? setZoomLevel(zoomLevel.value) : setZoomLevel(ZOOM_LEVELS[currentIndex - 1]);
}

function resetZoom(): void {
  setZoomLevel(defaultZoomLevel);
}

function zoomIn(): void {
  const currentIndex = ZOOM_LEVELS.indexOf(zoomLevel.value);
  if (currentIndex === ZOOM_LEVELS.length - 1) {
    return;
  }
  currentIndex === -1 ? setZoomLevel(zoomLevel.value) : setZoomLevel(ZOOM_LEVELS[currentIndex + 1]);
}

function onChange(value: string): void {
  setZoomLevel(value);
}

async function validateZoomLevel(value: string): Promise<string> {
  const level = parseInt(value);
  if (level < 5 || level > 500) {
    return '';
  }
  return value;
}
</script>

<style scoped lang="scss">
.zoom-level-input {
  width: 4.5rem;

  @include ms.responsive-breakpoint('xs') {
    display: none;
  }
}

.zoom-controls {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}
</style>
