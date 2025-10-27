<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-controls-group class="file-controls-playback">
    <file-controls-button
      :icon="playBack"
      v-show="false"
    />
    <file-controls-button
      :class="{ 'flip-horizontal-ion-icon': ended }"
      :icon="playbackIcon"
    />
    <file-controls-button
      :icon="playForward"
      v-show="false"
    />
  </file-controls-group>
</template>

<script setup lang="ts">
import { FileControlsButton, FileControlsGroup } from '@/components/files/handler/viewer';
import { pause, play, playBack, playForward, refresh } from 'ionicons/icons';
import { computed } from 'vue';

const props = defineProps<{
  ended: boolean;
  paused?: boolean;
}>();

const playbackIcon = computed((): string => {
  if (props.ended) {
    return refresh;
  }
  return props.paused === false ? pause : play;
});
</script>

<style scoped lang="scss">
.file-controls-playback::after {
  display: none !important;
}
</style>
