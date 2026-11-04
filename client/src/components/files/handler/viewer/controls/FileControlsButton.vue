<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-button
    class="file-controls-button color-secondary"
    :slot="!label ? 'icon-only' : ''"
    fill="outline"
  >
    <ion-icon
      class="file-controls-button-icon"
      :class="label ? 'button-left-icon' : ''"
      v-if="icon"
      slot="start"
      :icon="icon"
    />
    <ms-image
      class="file-controls-button-icon"
      :class="label ? 'button-left-icon' : ''"
      v-if="!icon && image"
      :image="image"
    />
    <span
      class="file-controls-button-label"
      v-if="label"
    >
      {{ $msTranslate(label) }}
    </span>
  </ion-button>
</template>

<script setup lang="ts">
import { IonButton, IonIcon } from '@ionic/vue';
import { MsImage, Translatable } from 'megashark-lib';

defineProps<{
  label?: Translatable;
  icon?: string;
  image?: string;
}>();
</script>

<style lang="scss" scoped>
.file-controls-button {
  --background: none;
  --background-hover: none;
  border-radius: ms.radius('md');
  min-height: 0;
  transition: all 0.2s ease-in-out;
  height: fit-content;
  --fill-color: #{ms.color('icon-neutral-default')} !important;

  &.button-disabled::part(native) {
    opacity: ms.opacity('disabled');
    box-shadow: none;
  }
}
</style>
