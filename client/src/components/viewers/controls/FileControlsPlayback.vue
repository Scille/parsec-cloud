<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-controls-group>
    <file-controls-button
      :class="{ 'flip-horizontal-ion-icon': ended }"
      :icon="playbackIcon"
    />
  </file-controls-group>
</template>

<script setup lang="ts">
import { refresh, play, pause } from 'ionicons/icons';
import { FileControlsButton, FileControlsGroup } from '@/components/viewers';
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

<style scoped lang="scss"></style>
